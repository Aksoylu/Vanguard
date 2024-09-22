use hyper::client::HttpConnector;
use hyper::StatusCode;
use hyper::{header, server::conn::Http, service::service_fn, Body, Client, Request, Response};
use std::path::PathBuf;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tokio_rustls::TlsAcceptor;

use crate::models::route::{HttpsRoute, SecureIwsRoute};
use crate::render::dir_index_page::DirIndexPage;
use crate::render::internal_error_page::InternalErrorPage;
use crate::render::not_found_page::NotFoundPage;
use crate::utils::directory_utility::is_directory_exist;
use crate::utils::file_utility::{get_content_type, is_file_exist, read_file_as_binary};
use crate::utils::network_utility::parse_ip_address;
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
        println!("Vanguard Engine Https server started on {:?}", &self.socket);

        let https_server: Arc<Mutex<HttpsServer>> = Arc::new(Mutex::new(self.clone()));
        let ssl_context: TlsAcceptor =
            create_ssl_context(self.https_routes.clone(), self.secure_iws_routes.clone());

        
        let listener: TcpListener = TcpListener::bind(&self.socket).await.unwrap();

        /* LifeCycle */
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
        tokio::spawn(async move {
            let stream = match tls_acceptor.accept(stream).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("TLS accept error: {:?}", e);
                    return;
                }
            };

            let service = service_fn(move |req: Request<Body>| {
                let https_server: Arc<Mutex<HttpsServer>> = Arc::clone(&https_server);
                async move {
                    let data: tokio::sync::MutexGuard<HttpsServer> = https_server.lock().await;

                    match data.handle_request(req).await {
                        Ok(response) => Ok::<_, hyper::Error>(response),
                        Err(err) => {
                            return Ok::<_, hyper::Error>(Response::new(Body::from(
                                InternalErrorPage::new("/", format!("{:?}", err).as_str()).render(),
                            )));
                        }
                    }
                }
            });

            let http = Http::new();
            if let Err(e) = http.serve_connection(stream, service).await {
                eprintln!("Server error: {}", e);
            }
        });
    }

    async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let request_host = req
            .headers()
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map_or_else(|| "/".to_string(), |host_value| host_value.to_string());

        /* Forwarding HTTPS requests */
        if self.https_routes.contains_key(&request_host) {
            let current_https_route = self.https_routes.get(&request_host).unwrap();

            if String::is_empty(&current_https_route.source) {
                let internal_server_error_content = self.render_internal_server_page(
                    &request_host,
                    "Requested target is not assigned to a valid HTTPS source",
                );
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(internal_server_error_content))
                    .unwrap());
            }

            return self.navigate_url(&current_https_route.target, req).await;
        }

        /* Processing IWS requests */
        if self.secure_iws_routes.contains_key(&request_host) {
            let current_iws_route = self.secure_iws_routes.get(&request_host).unwrap();

            if String::is_empty(&current_iws_route.source) {
                let internal_server_error_content = self.render_internal_server_page(
                    &request_host,
                    "Requested target is not associated with a file system for Vanguard Secure IWS",
                );

                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(internal_server_error_content))
                    .unwrap());
            }

            return self
                .serve_from_disk(&current_iws_route.serving_path, req)
                .await;
        }

        let internal_server_error_content = self.render_internal_server_page(
            &request_host,
            "Requested target is not registered on Vanguard Secure IWS",
        );
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(internal_server_error_content))
            .unwrap());
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

        let mut absolute_path = PathBuf::from(serving_path);
        absolute_path.push(url_path);

        if is_file_exist(&absolute_path) {
            let file_content: Option<Vec<u8>> = read_file_as_binary(&absolute_path).await;
            if file_content.is_none() {
                let internal_server_error_content = self.render_internal_server_page(
                    &url_path,
                    "Requested path is totally points an empty buffer, file or source on",
                );
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(internal_server_error_content))
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

        let not_found_content = self.render_not_found_page(&url_path);
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(not_found_content))
            .unwrap());
    }

    fn render_dir_index_page(&self, dir_path: &PathBuf, url_path: &str) -> String {
        let content = DirIndexPage::new(dir_path, url_path);

        content.render()
    }

    fn render_not_found_page(&self, url_path: &str) -> String {
        let content = NotFoundPage::new(url_path);

        content.render()
    }

    fn render_internal_server_page(&self, url_path: &str, reason: &str) -> String {
        let content = InternalErrorPage::new(url_path, reason);

        content.render()
    }
}
