use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::net::IpAddr;
use std::path::PathBuf;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tokio_rustls::TlsAcceptor;

use crate::core::common_handler::{CommonHandler, Protocol};
use crate::models::route::{HttpsRoute, SecureIwsRoute};
use crate::{log_debug, log_error, log_info};

use crate::render::Render;
use crate::utils::directory_utility::is_directory_exist;
use crate::utils::file_utility::is_file_exist;
use crate::utils::network_utility::{extract_host, parse_ip_address};
use crate::utils::tls_utility::create_ssl_context;

#[derive(Clone)]
pub struct HttpsServer {
    socket: SocketAddr,
    https_routes: HashMap<String, HttpsRoute>,
    secure_iws_routes: HashMap<String, SecureIwsRoute>,
}

impl HttpsServer {
    pub fn singleton(
        ip_address: String,
        port: u16,
        https_routes: HashMap<String, HttpsRoute>,
        secure_iws_routes: HashMap<String, SecureIwsRoute>,
    ) -> Self {
        let ip = parse_ip_address(ip_address.clone());
        let socket = SocketAddr::from((ip, port));

        Self {
            socket,
            https_routes,
            secure_iws_routes,
        }
    }

    pub async fn start(&self) {
        let https_server: Arc<Mutex<HttpsServer>> = Arc::new(Mutex::new(self.clone()));
        let ssl_context: TlsAcceptor =
            create_ssl_context(self.https_routes.clone(), self.secure_iws_routes.clone());

        let listener: TcpListener = TcpListener::bind(&self.socket).await.unwrap();

        log_info!("Vanguard Engine Https server started on {:?}", &self.socket);

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let tls_acceptor: TlsAcceptor = ssl_context.clone();
            let https_server: Arc<Mutex<HttpsServer>> = Arc::clone(&https_server);
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone
                    .lifecycle(tls_acceptor, stream, https_server)
                    .await;
            });
        }
    }

    async fn lifecycle(
        &self,
        tls_acceptor: TlsAcceptor,
        stream: tokio::net::TcpStream,
        https_server: Arc<Mutex<HttpsServer>>,
    ) {
        let remote_client_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(_) => {
                log_error!("Unable to get client address");
                return;
            }
        };

        tokio::spawn(async move {
            let stream = match tls_acceptor.accept(stream).await {
                Ok(stream) => stream,
                Err(e) => {
                    log_error!("TLS accept error: {:?}", e);
                    return;
                }
            };

            let service = service_fn(move |req: Request<Body>| {
                let https_server: Arc<Mutex<HttpsServer>> = Arc::clone(&https_server);
                let client_ip = remote_client_addr.ip();

                async move {
                    let data: tokio::sync::MutexGuard<HttpsServer> = https_server.lock().await;

                    match data.handle_request(req, client_ip).await {
                        Ok(response) => Ok::<_, hyper::Error>(response),
                        Err(err) => {
                            return Ok::<_, hyper::Error>(Response::new(Body::from(
                                Render::internal_server_error("/", format!("{:?}", err).as_str()),
                            )));
                        }
                    }
                }
            });

            let http = Http::new();
            if let Err(e) = http.serve_connection(stream, service).await {
                log_error!("Server error: {}", e);
            }
        });
    }

    async fn handle_https_route(
        &self,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        log_debug!(
            "HTTPS outband request source found in https route registry:  {:?}",
            request_host
        );

        let current_https_route = self.https_routes.get(request_host).unwrap();

        if !String::is_empty(&current_https_route.source) {
            log_debug!(
                "HTTPS outband request source ({}) is known. Forwarding request to {}",
                &current_https_route.source,
                &current_https_route.target
            );

            return CommonHandler::url_execution(
                Protocol::HTTPS,
                request_host,
                &current_https_route.target,
                req,
                client_ip.clone(),
            )
            .await;
        }

        log_debug!(
            "HTTPS outband request source ({}) as domain/target is is unknown",
            &current_https_route.source
        );

        return CommonHandler::not_found_error(Protocol::HTTPS, request_host, req, client_ip).await;
    }

    async fn handle_secure_iws_route(
        &self,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let url_path = req.uri().path().strip_prefix("/").unwrap_or("");

        log_debug!(
            "HTTPS outband request source found in Secure IWS registry:  {:?}",
            &request_host
        );

        let current_iws_route = self.secure_iws_routes.get(request_host).unwrap();
        if String::is_empty(&current_iws_route.serving_path) {
            log_debug!(
                "HTTPS outband IWS request source ({}) as domain/target is is unknown",
                &current_iws_route.source
            );

            return CommonHandler::iws_route_not_found_error(Protocol::HTTPS, request_host, req, client_ip).await;
        }

        let mut requested_disk_path: PathBuf = PathBuf::from(&current_iws_route.serving_path);
        requested_disk_path.push(url_path);

        if is_file_exist(&requested_disk_path) {
            log_debug!(
                "HTTPS outband IWS request source ({}) is known. Serving file from disk (Secure IWS registry) at path: {}",
                &current_iws_route.source,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_file_execution(
                Protocol::HTTPS,
                request_host,
                &requested_disk_path,
                req,
                client_ip,
            )
            .await;
        }

        if is_directory_exist(&requested_disk_path) {
            log_debug!(
                "HTTPS outband IWS request source ({}) is known. Serving directory from disk (Secure IWS registry) at path: {}",
                &current_iws_route.source,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_directory_execution(
                Protocol::HTTPS,
                request_host,
                &requested_disk_path,
                req,
                client_ip,
            )
            .await;
        }

        log_debug!(
            "HTTPS outband IWS request source ({}) is known. But requested path '{}' doesn't exist",
            &current_iws_route.source,
            &current_iws_route.serving_path
        );

        CommonHandler::iws_empty_path_error(Protocol::HTTPS, &request_host, req, client_ip).await
    }

    async fn handle_request(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let request_host = extract_host(&req);

        log_debug!("HTTPS outband request received: {:?}", &req);
        log_debug!("HTTPS outband request host: {:?}", &request_host);

        /* Forwarding HTTPS requests */
        log_debug!("Looking for Https route table:");

        if self.https_routes.contains_key(&request_host) {
            return self.handle_https_route(&request_host, req, client_ip).await;
        }

        /* Processing IWS requests */
        log_debug!("Looking for Secure IWS route table:");

        if self.secure_iws_routes.contains_key(&request_host) {
            return self
                .handle_secure_iws_route(&request_host, req, client_ip)
                .await;
        }

        /* Handle not found */
        log_info!(
            "HTTPS outband request host {:?} not found in Secure IWS or HTTPS Route table.",
            &request_host
        );

        CommonHandler::not_found_error(Protocol::HTTPS, &request_host, req, client_ip).await
    }
}
