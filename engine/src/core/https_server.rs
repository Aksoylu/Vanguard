use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::net::IpAddr;
use std::path::PathBuf;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::constants::Constants;
use crate::core::common_handler::{CommonHandler, Protocol};
use crate::core::shared_memory::ROUTER;
use crate::models::route::{HttpsRoute, SecureIwsRoute};
use crate::{log_debug, log_error, log_info};

use crate::render::Render;
use crate::utils::network_utility::{extract_host, parse_ip_address};
use crate::utils::tls_utility::create_ssl_context;

use tokio::net::TcpStream as TokioTcpStream;
use tokio::sync::oneshot as TokioChannel;

// Global Http Server Instance: Initially empty default config, updated in Runtime init

#[derive(Clone)]
pub struct HttpsServer {
    socket: SocketAddr,
    https_routes: HashMap<String, HttpsRoute>,
    secure_iws_routes: HashMap<String, SecureIwsRoute>,
}

impl Default for HttpsServer {
    fn default() -> Self {
        let default_ip_address = parse_ip_address(Constants::DEFAULT_HTTP_IP.to_string());
        let default_port = Constants::DEFAULT_HTTP_PORT;

        let default_socket_instance: SocketAddr =
            SocketAddr::from((default_ip_address, default_port));

        let default_https_route_table: HashMap<String, HttpsRoute> = HashMap::new();
        let default_secure_iws_route_table: HashMap<String, SecureIwsRoute> = HashMap::new();

        Self {
            socket: default_socket_instance,
            https_routes: default_https_route_table,
            secure_iws_routes: default_secure_iws_route_table,
        }
    }
}

impl HttpsServer {
    pub fn init(ip_address: String, port: u16) -> Self {
        let ip = parse_ip_address(ip_address.clone());
        let socket = SocketAddr::from((ip, port));

        let router = ROUTER.read().unwrap();

        Self {
            socket,
            https_routes: router.get_https_routes(),
            secure_iws_routes: router.get_secure_iws_routes(),
        }
    }

    pub async fn start(&self) {
        let ssl_context: TlsAcceptor =
            create_ssl_context(self.https_routes.clone(), self.secure_iws_routes.clone());

        let listener: TcpListener = TcpListener::bind(&self.socket).await.unwrap();

        log_info!("Vanguard Engine Https server started on {:?}", &self.socket);

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let tls_acceptor: TlsAcceptor = ssl_context.clone();
            let https_server = Arc::new(self.clone());
            tokio::spawn(async move {
                https_server.lifecycle(tls_acceptor, stream).await;
            });
        }
    }

    async fn lifecycle(&self, tls_acceptor: TlsAcceptor, tcp_stream: tokio::net::TcpStream) {
        let remote_client_addr = match tcp_stream.peer_addr() {
            Ok(addr) => addr,
            Err(error) => {
                log_error!("Unable to get client address: {}", error.to_string());
                return;
            }
        };

        let self_clone = Arc::new(self.clone());
        tokio::spawn(async move {
            // 1. Creating a "oneshot" channel to receive the negotiated protocol from the TLS handshake.
            let (transmitter, receiver) = TokioChannel::channel();

            // 2. Performing the TLS handshake.
            // Here we are using `accept_with` to peek into the session and see if the client wants HTTP/2 (h2).
            let accept_tls = tls_acceptor
                .accept_with(tcp_stream, |session| {
                    let negotiated_protocol = session.alpn_protocol().map(|p| p.to_vec());
                    let stream_result = transmitter.send(negotiated_protocol);

                    if stream_result.is_err() {
                        log_error!(
                            "Unable to send negotiated protocol: {:?}",
                            stream_result.err().unwrap()
                        );
                        return;
                    }
                })
                .await;

            // 3. Handling the TLS handshake result.
            let tls_stream = match accept_tls {
                Ok(tls_stream) => tls_stream,
                Err(e) => {
                    log_error!("TLS Handshake failed: {:?}", e);
                    return;
                }
            };

            // 4. Determining if we should use HTTP/2 or fallback to HTTP/1.1.
            let mut server_engine = Http::new();
            if let Ok(Some(protocol)) = receiver.await {
                if protocol == b"h2" {
                    log_debug!("HTTP/2 connection negotiated via ALPN");
                    server_engine.http2_only(true);
                }
            }

            // 5. Creating a service instance to handle incoming requests.
            let service = service_fn(move |req: Request<Body>| {
                let self_clone = Arc::clone(&self_clone);
                let client_ip = remote_client_addr.ip();

                async move {
                    match self_clone.handle_request(req, client_ip).await {
                        Ok(response) => Ok::<_, hyper::Error>(response),
                        Err(err) => {
                            return Ok::<_, hyper::Error>(Response::new(Body::from(
                                Render::internal_server_error("/", format!("{:?}", err).as_str()),
                            )));
                        }
                    }
                }
            });

            if let Err(e) = server_engine.serve_connection(tls_stream, service).await {
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

        if !String::is_empty(&request_host) {
            log_debug!(
                "HTTPS outband request source ({}) is known. Forwarding request to {}",
                &request_host,
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
            &request_host
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
                &request_host
            );

            return CommonHandler::iws_route_not_found_error(
                Protocol::HTTPS,
                request_host,
                req,
                client_ip,
            )
            .await;
        }

        let mut requested_disk_path: PathBuf = PathBuf::from(&current_iws_route.serving_path);
        requested_disk_path.push(url_path);

        let read_metadata = tokio::fs::metadata(&requested_disk_path).await;
        if read_metadata.is_err() {
            log_debug!(
                "HTTPS outband IWS request source ({}) is known. But requested path '{}' doesn't exist",
                &request_host,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_route_not_found_error(
                Protocol::HTTPS,
                request_host,
                req,
                client_ip,
            )
            .await;
        }

        let metadata = read_metadata.unwrap();

        if metadata.is_file() {
            log_debug!(
                "HTTPS outband IWS request source ({}) is known. Serving file from disk (Secure IWS registry) at path: {}",
                &request_host,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_file_execution(
                Protocol::HTTPS,
                request_host,
                &requested_disk_path,
                &metadata,
                req,
                client_ip,
            )
            .await;
        }

        if metadata.is_dir() {
            log_debug!(
                "HTTPS outband IWS request source ({}) is known. Serving directory from disk (Secure IWS registry) at path: {}",
                &request_host,
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
            &request_host,
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
