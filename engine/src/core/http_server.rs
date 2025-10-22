use hyper::{
    header::HeaderValue,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, StatusCode,
};

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;

use hyper::client::HttpConnector;

use crate::{
    log_debug, log_error, log_info, log_warn,
    models::route::{HttpRoute, IwsRoute},
    render::Render,
    utils::{
        directory_utility::is_directory_exist,
        file_utility::{get_content_type, is_file_exist, read_file_as_binary},
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
        return self.navigate_http_route_not_found(req, client_ip).await;
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

            return self
                .navigate_url(&current_http_route.target, req, client_ip.clone())
                .await;
        }

        log_debug!(
            "HTTP outband request source ({}) as domain/target is is unknown",
            &current_http_route.source
        );

        return self
            .navigate_http_route_not_found(req, client_ip.clone())
            .await;
    }

    async fn navigate_url(
        &self,
        endpoint_to_navigate: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_host = req.uri().host().unwrap_or("unknown").to_string();
        let request_path = original_uri.path().to_string();

        let mut new_uri = format!("http://{}{}", endpoint_to_navigate, request_path);
        if let Some(query) = original_uri.query() {
            new_uri.push('?');
            new_uri.push_str(query);
        }

        let (mut parts, body) = req.into_parts();
        parts.uri = new_uri.parse().unwrap();
        parts.headers.insert(
            "x-forwarded-for",
            HeaderValue::from_str(&client_ip.to_string()).unwrap(),
        );

        let new_request = Request::from_parts(parts, body);

        let http = HttpConnector::new();
        let client: Client<HttpConnector> = Client::builder().build(http);

        let response = client.request(new_request).await?;

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "HTTP |EXECUTION| {} {} {} ({} ms) from {} to {} via ip {}",
            request_method,
            request_path,
            &response.status().as_u16(),
            elapsed_time,
            request_host,
            &endpoint_to_navigate,
            &client_ip
        );

        Ok(response)
    }

    async fn navigate_http_route_not_found(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_host = req.uri().host().unwrap_or("unknown").to_string();
        let request_path = original_uri.path().to_string();

        let internal_server_error_content = Render::internal_server_error(
            &request_host,
            "Requested domain/target is not assigned to a valid HTTP source",
        );

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "HTTP |ROUTE NOT FOUND| {} {} {} ({} ms) from {} via ip {}",
            request_method,
            request_path,
            StatusCode::NOT_FOUND.as_u16(),
            elapsed_time,
            request_host,
            &client_ip
        );

        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(internal_server_error_content))
            .unwrap());
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

            return self
                .navigate_iws_route_not_found(req, client_ip.clone())
                .await;
        }

        let mut requested_disk_path: PathBuf = PathBuf::from(&current_iws_route.serving_path);
        requested_disk_path.push(url_path);

        if is_file_exist(&requested_disk_path) {
            log_debug!(
                "HTTP outband IWS request source ({}) is known. Serving file from disk (IWS registry) at path: {}",
                &current_iws_route.source,
                &current_iws_route.serving_path
            );

            return self
                .navigate_iws_static_file(&requested_disk_path, req, client_ip.clone())
                .await;
        }

        if is_directory_exist(&requested_disk_path) {
            log_debug!(
                "HTTP outband IWS request source ({}) is known. Serving directory from disk (IWS registry) at path: {}",
                &current_iws_route.source,
                &current_iws_route.serving_path
            );

            return self
                .navigate_iws_static_directory(&requested_disk_path, req, client_ip.clone())
                .await;
        }

        log_debug!(
            "HTTP outband IWS request source ({}) is known. But requested path '{}' doesn't exist",
            &current_iws_route.source,
            &current_iws_route.serving_path
        );

        return self.navigate_iws_empty_path(req, client_ip.clone()).await;
    }

    async fn navigate_iws_route_not_found(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_host = req.uri().host().unwrap_or("unknown").to_string();
        let request_path = original_uri.path().to_string();

        let internal_server_error_content = Render::internal_server_error(
            &request_host,
            "Requested domain/target is not assigned to a valid HTTP source",
        );

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "HTTP |IWS RECORD NOT FOUND| {} {} {} ({} ms) from {} via ip {}",
            request_method,
            request_path,
            StatusCode::NOT_FOUND.as_u16(),
            elapsed_time,
            request_host,
            &client_ip
        );

        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(internal_server_error_content))
            .unwrap());
    }

    async fn navigate_iws_static_file(
        &self,
        serving_path: &PathBuf,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_host = req.uri().host().unwrap_or("unknown").to_string();
        let request_path = original_uri.path().to_string();

        let file_content: Option<Vec<u8>> = read_file_as_binary(&serving_path).await;

        let elapsed_time = start_time.elapsed().as_millis();

        if file_content.is_none() {
            log_info!(
                "HTTP |IWS RECORD FOUND| {} {} {} ({} ms) from {} via ip {}",
                request_method,
                request_path,
                StatusCode::NOT_FOUND.as_u16(),
                elapsed_time,
                request_host,
                &client_ip
            );

            return Ok(Response::builder()
                .status(StatusCode::FOUND)
                .body(Body::from("302 - Requested file is empty"))
                .unwrap());
        }

        let content_type = get_content_type(&serving_path);

        log_info!(
            "HTTP |IWS EXECUTION| {} {} {} ({} ms) from {} to {} via ip {}",
            request_method,
            request_path,
            StatusCode::OK.as_u16(),
            elapsed_time,
            request_host,
            &serving_path.display(),
            &client_ip
        );
        return Ok(Response::builder()
            .header("Content-Type", content_type.as_ref())
            .body(Body::from(file_content.unwrap()))
            .unwrap());
    }

    /// If Index.html exist, render index.html as text
    /// If Index.html not exist, get directory childs, prepare a html content and render as text
    async fn navigate_iws_static_directory(
        &self,
        serving_path: &PathBuf,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let index_html_path = serving_path.join("index.html");
        if is_file_exist(&index_html_path) {
            return self
                .navigate_iws_static_file(&index_html_path, req, client_ip.clone())
                .await;
        }

        let original_uri = req.uri().clone();
        let url_path = original_uri.path().strip_prefix("/").unwrap_or("");
        let request_method = req.method().clone();
        let request_host = req.uri().host().unwrap_or("unknown").to_string();
        let request_path = original_uri.path().to_string();

        let absolute_path = serving_path.join("index.html");

        let dir_content: String = Render::directory_explorer_page(&absolute_path, &url_path);

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "HTTP |IWS EXECUTION| {} {} {} ({} ms) from {} to {} via ip {}",
            request_method,
            request_path,
            StatusCode::OK.as_u16(),
            elapsed_time,
            request_host,
            &serving_path.display(),
            &client_ip
        );

        return Ok(Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(dir_content))
            .unwrap());
    }

    // todo
    async fn navigate_iws_empty_path(
        &self,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_host = req.uri().host().unwrap_or("unknown").to_string();
        let request_path = original_uri.path().to_string();

        let internal_server_error_content = Render::internal_server_error(
            &request_host,
            "Requested domain/target is not assigned to a valid HTTP source",
        );

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "HTTP |IWS EMPTY EXECUTION| {} {} {} ({} ms) from {} via ip {}",
            request_method,
            request_path,
            StatusCode::NOT_FOUND.as_u16(),
            elapsed_time,
            request_host,
            &client_ip
        );

        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(internal_server_error_content))
            .unwrap());
    }
}
