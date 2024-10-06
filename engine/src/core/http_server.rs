use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server, StatusCode,
};

use std::{collections::HashMap, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use hyper::client::HttpConnector;

use crate::{
    core::log_service::LogService,
    models::route::{HttpRoute, IwsRoute},
    render::{
        dir_index_page::DirIndexPage, internal_error_page::InternalErrorPage,
        not_found_page::NotFoundPage,
    },
    utils::{
        directory_utility::is_directory_exist,
        file_utility::{get_content_type, is_file_exist, read_file_as_binary},
        network_utility::parse_ip_address,
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

        let make_svc = make_service_fn(|_conn| {
            let http_server = Arc::clone(&http_server);

            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let http_server = Arc::clone(&http_server);
                    async move {
                        let data = http_server.lock().await;

                        match data.handle_request(req).await {
                            Ok(response) => Ok::<_, hyper::Error>(response),
                            Err(err) => {
                                return Ok::<_, hyper::Error>(Response::new(Body::from(
                                    InternalErrorPage::new("/", format!("{:?}", err).as_str())
                                        .render(),
                                )));
                            }
                        }
                    }
                }))
            }
        });

        LogService::success(format!(
            "Vanguard Engine Http server started on {:?}",
            &self.socket
        ));

        if let Err(e) = Server::bind(&self.socket).serve(make_svc).await {
            LogService::error(format!("Vanguard Engine Http server error {:?}", e));
        }
    }

    async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let request_host = req
            .headers()
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map_or_else(|| "/".to_string(), |host_value| host_value.to_string());

        LogService::output(format!("HTTP outband request received: {:?}", &req));
        LogService::output(format!("HTTP outband request host: {:?}", &request_host));

        /* Forwarding HTTP requests */
        LogService::output("Looking for Http route table:");
        if self.http_routes.contains_key(&request_host) {
            LogService::success(format!(
                "HTTP outband request source found in http route registry:  {:?}",
                &request_host
            ));

            let current_http_route = self.http_routes.get(&request_host).unwrap();

            if String::is_empty(&current_http_route.source) {
                LogService::error(format!(
                    "HTTP outband request source ({}) as domain/target is is unknown",
                    &current_http_route.source
                ));

                let internal_server_error_content = self.render_internal_server_error(
                    &request_host,
                    "Requested domain/target is not assigned to a valid HTTPS source",
                );

                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(internal_server_error_content))
                    .unwrap());
            }

            LogService::success(format!(
                "HTTP outband request source ({}) is known. Forwarding request to {}",
                &current_http_route.source, &current_http_route.target
            ));

            return self.navigate_url(&current_http_route.target, req).await;
        } else {
            LogService::output(format!(
                "HTTP outband request not registered in Http Route table {:?}",
                &request_host
            ));
        }

        /* Processing IWS requests */
        LogService::output("Looking for IWS route table:");
        if self.iws_routes.contains_key(&request_host) {
            LogService::success(format!(
                "HTTP outband request source found in IWS registry:  {:?}",
                &request_host
            ));

            let current_iws_route = self.iws_routes.get(&request_host).unwrap();

            if !is_directory_exist(&PathBuf::from(current_iws_route.serving_path.to_string())) {
                LogService::error(format!(
                    "HTTP outband request source ({}) as domain/target is is unknown",
                    &request_host
                ));
    
                let internal_server_error_content = self.render_internal_server_error(
                    &request_host,
                    "Requested domain has not registered on Vanguard",
                );
    
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(internal_server_error_content))
                    .unwrap());
            }


                
            LogService::success(format!(
                "HTTP outband request source is serving on IWS registry path: {}",
                &current_iws_route.serving_path
            ));

            return self
                .serve_from_disk(&current_iws_route.serving_path, req)
                .await;
        }
        else {
            LogService::output(format!(
                "HTTP outband request not registered in IWS Route table {:?}",
                &request_host
            ));
        }

        /* Handle not found */
        LogService::output(format!(
            "Http outband request host {:?} not found in IWS or HTTP Route table.",
            &request_host
        ));

        let internal_server_error_content = self.render_internal_server_error(
            &request_host,
            "Requested domain has not registered on Vanguard",
        );

        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(internal_server_error_content))
            .unwrap())
    }

    async fn navigate_url(
        &self,
        endpoint_to_navigate: &String,
        req: Request<Body>,
    ) -> Result<Response<Body>, hyper::Error> {
        let original_uri = req.uri().clone();

        let mut new_uri = format!("http://{}{}", endpoint_to_navigate, original_uri.path());
        if let Some(query) = original_uri.query() {
            new_uri.push('?');
            new_uri.push_str(query);
        }

        let (mut parts, body) = req.into_parts();
        parts.uri = new_uri.parse().unwrap();

        let new_request = Request::from_parts(parts, body);

        let http = HttpConnector::new();
        let client: Client<HttpConnector> = Client::builder().build(http);

        let response = client.request(new_request).await?;

        Ok(response)
    }

    async fn serve_from_disk(
        &self,
        serving_path: &String,
        req: Request<Body>,
    ) -> Result<Response<Body>, hyper::Error> {
        let url_path = req.uri().path().strip_prefix("/").unwrap_or("");

        let mut absolute_path: PathBuf = PathBuf::from(serving_path);
        absolute_path.push(url_path);

        if is_file_exist(&absolute_path) {
            let file_content: Option<Vec<u8>> = read_file_as_binary(&absolute_path).await;
            if file_content.is_none() {
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("500 - Internal Server Error"))
                    .unwrap());
            }

            let content_type = get_content_type(&absolute_path);

            return Ok(Response::builder()
                .header("Content-Type", content_type.as_ref()) // Set the Content-Type header
                .body(Body::from(file_content.unwrap()))
                .unwrap());
        }

        /* If directory exist;
               If Index.html exist, render index.html as text
               If Index.html not exist, get directory childs, prepare a html content and render as text
        */
        if is_directory_exist(&absolute_path) {
            let mut index_html_path = absolute_path.clone();
            index_html_path = index_html_path.join(PathBuf::from("index.html"));

            if is_file_exist(&index_html_path) {
                let file_content = read_file_as_binary(&index_html_path).await;
                if file_content.is_some() {
                    return Ok(Response::builder()
                        .header("Content-Type", "text/html")
                        .body(Body::from(file_content.unwrap()))
                        .unwrap());
                }
            }

            let dir_content = self.render_dir_index_page(&absolute_path, &url_path);
            return Ok(Response::builder()
                .header("Content-Type", "text/html")
                .body(Body::from(dir_content))
                .unwrap());
        }

        let not_found_content = self.render_not_found_error(&url_path);
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(not_found_content))
            .unwrap());
    }

    fn render_dir_index_page(&self, dir_path: &PathBuf, url_path: &str) -> String {
        let content = DirIndexPage::new(dir_path, url_path);

        content.render()
    }

    fn render_not_found_error(&self, url_path: &str) -> String {
        let content = NotFoundPage::new(url_path);

        content.render()
    }

    fn render_internal_server_error(&self, url_path: &str, reason: &str) -> String {
        let content = InternalErrorPage::new(url_path, reason);

        content.render()
    }
}
