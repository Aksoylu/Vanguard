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
        common_handler::{CommonHandler, Protocol},
        connection_lock::ConnectionLock,
        shared_memory::{
            CONNECTION_MANAGER, RELOAD_SIGNAL, ROUTER, RUNTIME_BOOT_INFO, SHUTDOWN_SIGNAL,
        },
    },
    log_debug, log_error, log_info,
    models::route::{http_route::HttpRoute, iws_route::IwsRoute},
    models::traffic_policy::scope_traffic_policy::ScopeTrafficPolicy,
    render::Render,
    utils::{
        http_utility::calculate_content_length,
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

    /// Starts the HTTP server
    pub async fn start(&self) {
        let http_server = Arc::new(self.clone());
        log_info!("Vanguard Engine Http server started on {:?}", &self.socket);

        loop {
            // Creates a fresh service factory for each server instance
            let http_server_clone = Arc::clone(&http_server);
            let make_svc = make_service_fn(move |connection: &AddrStream| {
                let client = connection.remote_addr();
                let http_server = Arc::clone(&http_server_clone);

                async move {
                    let start_new_connection =
                        Arc::new(CONNECTION_MANAGER.try_acquire_connection());

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

            let mut shutdown_event = SHUTDOWN_SIGNAL.subscriber.clone();
            let mut reload_event = RELOAD_SIGNAL.subscriber.clone();

            let stop_signal = async move {
                tokio::select! {
                    _ = shutdown_event.wait_for(|&s| s) => {
                        log_info!("HTTP Server received shutdown signal.");
                    }
                    _ = reload_event.wait_for(|&r| r) => {
                        log_info!("HTTP Server received reload signal. Restarting engine...");
                    }
                }
            };

            let execution_result = self
                .get_server_engine()
                .serve(make_svc)
                .with_graceful_shutdown(stop_signal)
                .await;

            if execution_result.is_err() {
                let error = execution_result.err().unwrap();
                log_error!("Vanguard Engine Http server error {:?}", error);
                break;
            }

            if *SHUTDOWN_SIGNAL.subscriber.borrow() {
                log_info!("HTTP Server shutting down loop.");
                break;
            }

            log_info!("HTTP Server reloading engine with new configuration...");
        }
    }

    /// Returns a new HTTP server engine with the current traffic policy
    fn get_server_engine(&self) -> Builder<AddrIncoming> {
        // Clone traffic_policy to drop the RwLockReadGuard immediately
        let traffic_policy = {
            let runtime_info = RUNTIME_BOOT_INFO.read().unwrap();
            runtime_info.config.get_http_effective_policy()
        };

        let server_engine = Server::bind(&self.socket)
            .http1_header_read_timeout(std::time::Duration::from_secs(
                traffic_policy
                    .http1_protocol_settings
                    .get_http1_header_read_timeout(),
            ))
            .tcp_keepalive(Some(std::time::Duration::from_secs(
                traffic_policy.upstream_settings.get_pool_idle_timeout(),
            )))
            .http1_max_buf_size(
                traffic_policy.upstream_settings.get_max_request_body_size() as usize
            )
            .http1_only(traffic_policy.http1_protocol_settings.get_http1_only())
            .tcp_nodelay(traffic_policy.http1_protocol_settings.get_tcp_nodelay())
            .http1_keepalive(traffic_policy.http1_protocol_settings.get_http1_keepalive())
            .http1_half_close(true)
            .http1_writev(true);

        server_engine
    }

    /// Executes the request lifecycle
    async fn lifecycle(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
        connection_lock: &Option<ConnectionLock>,
    ) -> Result<Response<Body>, hyper::Error> {
        let request_host = extract_host(&req);

        // Get traffic_policy to drop the RwLockReadGuard immediately
        let (traffic_policy, global_rate_limit) = {
            let runtime_info = RUNTIME_BOOT_INFO.read().unwrap();
            (
                runtime_info.config.get_http_effective_policy(),
                runtime_info
                    .config
                    .global_traffic_policy
                    .server
                    .max_requests_per_minute,
            )
        };

        // Global Request Body Size Check
        let content_length = calculate_content_length(&req).map_err(|_| {
            Response::builder()
                .status(hyper::StatusCode::BAD_REQUEST)
                .body(Body::from(Render::internal_server_error(
                    &request_host,
                    "Invalid Content-Length header",
                )))
                .unwrap()
        });

        let content_length = content_length.unwrap();

        if content_length > traffic_policy.upstream_settings.get_max_request_body_size() {
            return Ok(Response::builder()
                .status(hyper::StatusCode::PAYLOAD_TOO_LARGE)
                .body(Body::from(Render::internal_server_error(
                    &request_host,
                    "Payload too large",
                )))
                .unwrap());
        }

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

        if !CONNECTION_MANAGER.check_rate_limit(client_ip, global_rate_limit) {
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

        let response = run_in_time_buffer(
            traffic_policy.upstream_settings.get_http_client_timeout() * 1000,
            self.handle_request(req, client_ip, traffic_policy),
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
        traffic_policy: ScopeTrafficPolicy,
    ) -> Result<Response<Body>, hyper::Error> {
        let request_host = extract_host(&req);

        log_debug!("HTTP outband request received: {:?}", &req);
        log_debug!("HTTP outband request host: {:?}", &request_host);

        /* Forwarding HTTP requests */
        log_debug!("Looking for Http route table:");

        if self.http_routes.contains_key(&request_host) {
            return self
                .handle_http_route(&request_host, req, client_ip, traffic_policy)
                .await;
        }

        /* Processing IWS requests */
        log_debug!("Looking for IWS route table:");

        if self.iws_routes.contains_key(&request_host) {
            return self
                .handle_iws_route(&request_host, req, client_ip, traffic_policy)
                .await;
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
        mut traffic_policy: ScopeTrafficPolicy,
    ) -> Result<Response<Body>, hyper::Error> {
        log_debug!(
            "HTTP outband request source found in http route registry:  {:?}",
            request_host
        );

        let current_http_route = self.http_routes.get(request_host).unwrap();

        // Merge route-specific overrides
        if let Some(ref route_overrides) = current_http_route.traffic_policy {
            traffic_policy.merge(route_overrides);
        }

        // Merge path-specific overrides
        if let Some(ref path_overrides) = current_http_route.path_policy {
            traffic_policy.merge_path_policy(path_overrides);
        }

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
                &traffic_policy,
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
        _traffic_policy: ScopeTrafficPolicy,
    ) -> Result<Response<Body>, hyper::Error> {
        let url_path = req.uri().path().strip_prefix("/").unwrap_or("");

        log_debug!(
            "HTTP outband request source found in IWS registry:  {:?}",
            &request_host
        );

        let current_iws_route = self.iws_routes.get(request_host).unwrap();
        if !std::path::Path::new(&current_iws_route.serving_path).exists() {
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
