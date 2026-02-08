use hyper::{server::conn::Http, service::service_fn, Body, Request, Response};
use std::net::IpAddr;
use std::path::PathBuf;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::constants::Constants;
use crate::core::common_handler::{CommonHandler, Protocol};
use crate::core::connection_lock::ConnectionLock;
use crate::core::shared_memory::{CONNECTION_MANAGER, ROUTER, SHUTDOWN_SIGNAL};
use crate::models::route::https_route::HttpsRoute;
use crate::models::route::secure_iws_route::SecureIwsRoute;
use crate::utils::http_utility::get_content_length;
use crate::utils::time_utility::run_in_time_buffer;
use crate::{log_debug, log_error, log_info};

use crate::render::Render;
use crate::utils::network_utility::{extract_host, parse_ip_address};
use crate::utils::tls_utility::create_ssl_context;

use tokio::net::TcpStream as TokioTcpStream;
use tokio::sync::oneshot::{self as TokioChannel};

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

        let mut shutdown_event = SHUTDOWN_SIGNAL.subscriber.clone();
        let on_shutdown = async move {
            let _on_shutdown = shutdown_event.wait_for(|&s| s).await;
        };
        tokio::pin!(on_shutdown);

        loop {
            tokio::select! {
                _on_shutdown = &mut on_shutdown => {
                    break;
                }
                result = listener.accept() => {
                    let (tcp_stream, client) = result.unwrap();
                    let tls_acceptor: TlsAcceptor = ssl_context.clone();
                    let https_server = Arc::new(self.clone());
                    let client_ip = client.ip();

                    let start_new_connection = CONNECTION_MANAGER.try_acquire_connection();

                    tokio::spawn(async move {
                        https_server
                            .execute_request(tls_acceptor, tcp_stream, client_ip, start_new_connection)
                            .await;
                    });
                }
            }
        }
    }

    async fn execute_request(
        &self,
        tls_acceptor: TlsAcceptor,
        tcp_stream: TokioTcpStream,
        client_ip: IpAddr,
        start_new_connection: Option<ConnectionLock>,
    ) {
        let https_server = Arc::new(self.clone());
        let connection_lock = Arc::new(start_new_connection);
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
            let mut server_engine = https_server.get_server_engine();
            if let Ok(Some(protocol)) = receiver.await {
                if protocol == b"h2" {
                    log_debug!("HTTP/2 connection negotiated via ALPN");
                    server_engine.http2_only(true);
                }
            }

            // 6. Creating a service instance to handle incoming requests.
            let service = service_fn(move |req: Request<Body>| {
                let https_server_instance = Arc::clone(&https_server);
                let connection_lock = Arc::clone(&connection_lock);

                async move {
                    https_server_instance
                        .lifecycle(req, client_ip, &connection_lock)
                        .await
                }
            });

            if let Err(e) = server_engine.serve_connection(tls_stream, service).await {
                log_error!("Server error: {}", e);
            }
        });
    }

    /// @TODO: it will be build by using current configurations
    fn get_server_engine(&self) -> Http {
        let mut server_engine = Http::new();

        // 5. Setting timeouts and limits for scalability
        server_engine
            .http1_header_read_timeout(std::time::Duration::from_secs(
                Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT,
            ))
            .max_buf_size(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE as usize)
            .http2_keep_alive_timeout(std::time::Duration::from_secs(
                Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            ))
            .http2_max_concurrent_streams(Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST as u32)
            .http2_initial_connection_window_size(Some(Constants::DEFAULT_HTTP_INITIAL_CONNECTION_WINDOW_SIZE))
            .http2_initial_stream_window_size(Some(Constants::DEFAULT_HTTP2_STREAM_WINDOW_SIZE))
            .http2_max_frame_size(Some(Constants::DEFAULT_HTTP2_MAX_FRAME_SIZE))
            .http2_max_header_list_size(Constants::DEFAULT_HTTP2_MAX_HEADER_LIST_SIZE);

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

        // Global Request Body Size Check
        let content_length = get_content_length(&req);
        if content_length.is_err() {
            return Ok(Response::builder()
                .status(hyper::StatusCode::BAD_REQUEST)
                .body(Body::from(Render::internal_server_error(
                    &request_host,
                    content_length.err().unwrap().get_message(),
                )))
                .unwrap());
        }

        let parsed_content_length = content_length.unwrap();
        if parsed_content_length > Constants::DEFAULT_MAX_REQUEST_BODY_SIZE {
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
