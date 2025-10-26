use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;

use crate::{
    core::common_handler::{CommonHandler, Protocol},
    log_debug, log_error, log_info,
    models::route::{HttpRoute, IwsRoute},
    render::Render,
    utils::{
        directory_utility::is_directory_exist,
        file_utility::is_file_exist,
        network_utility::{extract_host, parse_ip_address},
    },
};

#[derive(Debug, Clone)]
pub struct HttpServer {
    socket: SocketAddr,
    http_routes: HashMap<String, HttpRoute>,
    iws_routes: HashMap<String, IwsRoute>,
}

impl HttpServer {
    pub fn singleton(
        ip_address: String,
        port: u16,
        http_routes: HashMap<String, HttpRoute>,
        iws_routes: HashMap<String, IwsRoute>,
    ) -> Self {
        let socket = SocketAddr::from((parse_ip_address(ip_address.clone()), port));

        Self {
            socket,
            http_routes,
            iws_routes,
        }
    }

    pub async fn start(&self) {
        let http_server = Arc::new(Mutex::new(self.clone()));

        let make_svc = make_service_fn(|connection: &hyper::server::conn::AddrStream| {
            let client = connection.remote_addr();
            let http_server = Arc::clone(&http_server);

            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let http_server = Arc::clone(&http_server);
                    let client_ip = client.ip();

                    async move {
                        let data = http_server.lock().await;

                        match data.handle_request(req, client_ip).await {
                            Ok(response) => Ok::<_, hyper::Error>(response),
                            Err(err) => {
                                return Ok::<_, hyper::Error>(Response::new(Body::from(
                                    Render::internal_server_error(
                                        "/",
                                        format!("{:?}", err).as_str(),
                                    ),
                                )));
                            }
                        }
                    }
                }))
            }
        });

        log_info!("Vanguard Engine Http server started on {:?}", &self.socket);

        if let Err(e) = Server::bind(&self.socket).serve(make_svc).await {
            log_error!("Vanguard Engine Http server error {:?}", e);
        }
    }

    async fn handle_request(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let request_host = extract_host(&req);

        log_debug!("HTTP outband request received: {:?}", &req);
        log_debug!("HTTP outband request host: {:?}", &request_host);

        /* Forwarding HTTP requests */
        log_debug!("Looking for Http route table:");

        if self.http_routes.contains_key(&request_host) {
            return self.handle_http_route(&request_host, req, client_ip).await;
        }

        /* Processing IWS requests */
        log_debug!("Looking for IWS route table:");

        if self.iws_routes.contains_key(&request_host) {
            return self.handle_iws_route(&request_host, req, client_ip).await;
        }

        /* Handle not found */
        log_info!(
            "Http outband request host {:?} not found in IWS or HTTP Route table.",
            &request_host
        );

        CommonHandler::not_found_error(Protocol::HTTP, req, client_ip).await
    }

    async fn handle_http_route(
        &self,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        log_debug!(
            "HTTP outband request source found in http route registry:  {:?}",
            request_host
        );

        let current_http_route = self.http_routes.get(request_host).unwrap();

        if !String::is_empty(&current_http_route.source) {
            log_debug!(
                "HTTP outband request source ({}) is known. Forwarding request to {}",
                &current_http_route.source,
                &current_http_route.target
            );

            return CommonHandler::url_execution(
                Protocol::HTTP,
                &request_host,
                &current_http_route.target,
                req,
                client_ip.clone(),
            )
            .await;
        }

        log_debug!(
            "HTTP outband request source ({}) as domain/target is is unknown",
            &current_http_route.source
        );

        CommonHandler::not_found_error(Protocol::HTTP, req, client_ip).await
    }

    async fn handle_iws_route(
        &self,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let url_path = req.uri().path().strip_prefix("/").unwrap_or("");

        log_debug!(
            "HTTP outband request source found in IWS registry:  {:?}",
            &request_host
        );

        let current_iws_route = self.iws_routes.get(request_host).unwrap();
        if String::is_empty(&current_iws_route.serving_path) {
            log_debug!(
                "HTTP outband IWS request source ({}) as domain/target is is unknown",
                &current_iws_route.source
            );

            return CommonHandler::iws_route_not_found_error(Protocol::HTTP, req, client_ip).await;
        }

        let mut requested_disk_path: PathBuf = PathBuf::from(&current_iws_route.serving_path);
        requested_disk_path.push(url_path);

        if is_file_exist(&requested_disk_path) {
            log_debug!(
                "HTTP outband IWS request source ({}) is known. Serving file from disk (IWS registry) at path: {}",
                &current_iws_route.source,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_file_execution(
                Protocol::HTTP,
                &requested_disk_path,
                req,
                client_ip,
            )
            .await;
        }

        if is_directory_exist(&requested_disk_path) {
            log_debug!(
                "HTTP outband IWS request source ({}) is known. Serving directory from disk (IWS registry) at path: {}",
                &current_iws_route.source,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_directory_execution(
                Protocol::HTTP,
                &requested_disk_path,
                req,
                client_ip,
            )
            .await;
        }

        log_debug!(
            "HTTP outband IWS request source ({}) is known. But requested path '{}' doesn't exist",
            &current_iws_route.source,
            &current_iws_route.serving_path
        );

        CommonHandler::iws_empty_path_error(Protocol::HTTP, req, client_ip).await
    }
}
