use hyper::{
    server::{
        conn::{AddrIncoming, AddrStream},
        Builder,
    },
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use std::{path::PathBuf, sync::Arc};

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
};

use crate::{
    constants::Constants,
    core::{
        common_handler::{CommonHandler, Protocol}, connection_lock::ConnectionLock, shared_memory::{CONNECTION_MANAGER, ROUTER}
    },
    log_debug, log_error, log_info,
    models::route::{HttpRoute, IwsRoute},
    render::Render,
    utils::{
        network_utility::{extract_host, parse_ip_address},
        time_utility::run_in_time_buffer,
    },
};

#[derive(Debug, Clone)]
pub struct HttpServer {
    socket: SocketAddr,
    http_routes: HashMap<String, HttpRoute>,
    iws_routes: HashMap<String, IwsRoute>,
}

impl Default for HttpServer {
    fn default() -> Self {
        let default_ip_address = parse_ip_address(Constants::DEFAULT_HTTP_IP.to_string());
        let default_port = Constants::DEFAULT_HTTP_PORT;

        let default_socket_instance: SocketAddr =
            SocketAddr::from((default_ip_address, default_port));

        let default_http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let default_iws_route_table: HashMap<String, IwsRoute> = HashMap::new();

        Self {
            socket: default_socket_instance,
            http_routes: default_http_route_table,
            iws_routes: default_iws_route_table,
        }
    }
}

impl HttpServer {
    pub fn init(ip_address: String, port: u16) -> Self {
        let ip = parse_ip_address(ip_address.clone());
        let socket = SocketAddr::from((ip, port));

        let router = ROUTER.read().unwrap();

        Self {
            socket,
            http_routes: router.get_http_routes(),
            iws_routes: router.get_iws_routes(),
        }
    }

    pub async fn start(&self) {
        let http_server = Arc::new(self.clone());

        // Creates a service factory
        let make_svc = make_service_fn(|connection: &AddrStream| {
            let client = connection.remote_addr();
            let http_server = Arc::clone(&http_server);

            async move {
                let start_new_connection = Arc::new(CONNECTION_MANAGER.try_acquire_connection());

                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let http_server = Arc::clone(&http_server);
                    let client_ip = client.ip();
                    let connection_lock = Arc::clone(&start_new_connection);

                    async move {
                        http_server
                            .lifecycle(req, client_ip, &connection_lock)
                            .await
                    }
                }))
            }
        });

        log_info!("Vanguard Engine Http server started on {:?}", &self.socket);

        let execution_result = self.get_server_engine().serve(make_svc).await;

        if execution_result.is_err() {
            let error = execution_result.err().unwrap();
            log_error!("Vanguard Engine Http server error {:?}", error);
        }
    }

    /// @TODO: it will be build by using current configurations
    fn get_server_engine(&self) -> Builder<AddrIncoming> {
        let server_engine = Server::bind(&self.socket)
            .tcp_nodelay(true)
            .http1_keepalive(true)
            .http1_header_read_timeout(std::time::Duration::from_secs(
                Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT,
            ))
            .tcp_keepalive(Some(std::time::Duration::from_secs(
                Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            )))
            .http1_max_buf_size(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE as usize);

        server_engine
    }

    /// Executes the request lifecycle
    async fn lifecycle(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
        connection_lock: &Option<ConnectionLock>,
    ) -> Result<Response<Body>, hyper::Error> {
        if connection_lock.is_none() {
            log_error!(
                "Rejecting request from ip address {:?}: Max connections reached",
                client_ip
            );

            return Ok::<_, hyper::Error>(
                Response::builder()
                    .status(hyper::StatusCode::SERVICE_UNAVAILABLE)
                    .body(Body::from(Render::internal_server_error(
                        "/",
                        "Max connections reached",
                    )))
                    .unwrap(),
            );
        }

        if !CONNECTION_MANAGER.check_rate_limit(client_ip) {
            log_info!("Rate limit exceeded for ip address {:?}", client_ip);
            return Ok::<_, hyper::Error>(
                Response::builder()
                    .status(hyper::StatusCode::TOO_MANY_REQUESTS)
                    .body(Body::from(Render::internal_server_error(
                        "/",
                        "Rate limit exceeded",
                    )))
                    .unwrap(),
            );
        }

        CONNECTION_MANAGER.increment_total_requests();
        let response = run_in_time_buffer(
            Constants::DEFAULT_SERVER_READ_TIMEOUT,
            self.handle_request(req, client_ip),
        )
        .await;

        if response.is_err() {
            return Ok::<_, hyper::Error>(Response::new(Body::from(
                Render::internal_server_error("/", "Request timed out"),
            )));
        }

        let completed_response = response.unwrap();

        if completed_response.is_err() {
            return Ok::<_, hyper::Error>(Response::new(Body::from(
                Render::internal_server_error(
                    "/",
                    format!("{:?}", completed_response.err().unwrap()).as_str(),
                ),
            )));
        }

        let result = completed_response.unwrap();
        Ok::<_, hyper::Error>(result)
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

        CommonHandler::not_found_error(Protocol::HTTP, &request_host, req, client_ip).await
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

        if !String::is_empty(&current_http_route.target) {
            log_debug!(
                "HTTP outband request source ({}) is known. Forwarding request to {}",
                &request_host,
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
            &request_host
        );

        CommonHandler::not_found_error(Protocol::HTTP, request_host, req, client_ip).await
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
                &request_host
            );

            return CommonHandler::iws_route_not_found_error(
                Protocol::HTTP,
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
                "HTTP outband IWS request source ({}) is known. But requested path '{}' doesn't exist",
                &request_host,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_route_not_found_error(
                Protocol::HTTP,
                request_host,
                req,
                client_ip,
            )
            .await;
        }

        let metadata = read_metadata.unwrap();

        if metadata.is_file() {
            log_debug!(
                "HTTP outband IWS request source ({}) is known. Serving file from disk (IWS registry) at path: {}",
                &request_host,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_file_execution(
                Protocol::HTTP,
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
                "HTTP outband IWS request source ({}) is known. Serving directory from disk (IWS registry) at path: {}",
                &request_host,
                &current_iws_route.serving_path
            );

            return CommonHandler::iws_static_directory_execution(
                Protocol::HTTP,
                request_host,
                &requested_disk_path,
                req,
                client_ip,
            )
            .await;
        }

        log_debug!(
            "HTTP outband IWS request source ({}) is known. But requested path '{}' doesn't exist",
            &request_host,
            &current_iws_route.serving_path
        );

        CommonHandler::iws_empty_path_error(Protocol::HTTP, request_host, req, client_ip).await
    }
}
