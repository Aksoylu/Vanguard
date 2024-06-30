use hyper::client::HttpConnector;
use hyper::{
    header,
    server::conn::Http,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tokio_rustls::TlsAcceptor;

use crate::utils::network_utility::parse_ip_address;
use crate::utils::tls_utility::{configure_tls, load_certs, load_private_key};

use super::models::HttpsRoute;

#[derive(Clone)]
pub struct HttpsServer {
    ip_address: String,
    port: u16,
    socket: SocketAddr,
    routes: HashMap<String, HttpsRoute>,
}

impl HttpsServer {
    pub fn singleton(ip_address: String, port: u16, routes: HashMap<String, HttpsRoute>) -> Self {
        let ip = parse_ip_address(ip_address.clone());
        let socket = SocketAddr::from((ip, port));

        Self {
            ip_address,
            port,
            socket,
            routes,
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
                            Err(_) => Ok::<_, hyper::Error>(Response::new(Body::from(
                                "Error processing request",
                            ))),
                        }
                    }
                }))
            }
        });

        // Load the certificates and keys
        let certs = load_certs("cert.pem").unwrap();
        let key = load_private_key("key.pem").unwrap();
        let tls_cfg = configure_tls(certs, key).unwrap();
        let tls_acceptor = TlsAcceptor::from(Arc::new(tls_cfg));

        if let Err(e) = Server::bind(&self.socket).serve(make_svc).await {
            eprintln!("Server error: {}", e);
            return;
        }

        println!("Reprox Server started on {:?}", &self.socket);

        let listener = TcpListener::bind(&self.socket).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let tls_acceptor = tls_acceptor.clone();
            let http_server = Arc::clone(&http_server);

            tokio::spawn(async move {
                let stream = match tls_acceptor.accept(stream).await {
                    Ok(stream) => stream,
                    Err(e) => {
                        eprintln!("TLS accept error: {:?}", e);
                        return;
                    }
                };

                let service = service_fn(move |req: Request<Body>| {
                    let http_server = Arc::clone(&http_server);
                    async move {
                        let data = http_server.lock().await;
                        match data.handle_request(req).await {
                            Ok(response) => Ok::<_, hyper::Error>(response),
                            Err(_) => Ok::<_, hyper::Error>(Response::new(Body::from(
                                "Error processing request",
                            ))),
                        }
                    }
                });

                let http = Http::new();
                if let Err(e) = http.serve_connection(stream, service).await {
                    eprintln!("Server error: {}", e);
                }
            });
        }
    }

    async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let request_host = req
            .headers()
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map_or_else(|| "/".to_string(), |host_value| host_value.to_string());

        if !self.routes.contains_key(&request_host) {
            let response =
                Response::new(Body::from("Requested Reprox redirection URL not found..."));
            return Ok(response);
        }

        if self.routes.get(&request_host).is_none() {
            let response = Response::new(Body::from(
                "Requested Reprox URL is not configured properly",
            ));
            return Ok(response);
        }

        let current_route = self.routes.get(&request_host).unwrap();

        if String::is_empty(&current_route.source) {
            let response =
                Response::new(Body::from("Requested Reprox redirection URL not found..."));
            return Ok(response);
        }

        let response = self.navigate_url(&current_route.target, req).await;
        return response;
    }

    async fn navigate_url(
        &self,
        endpoint_to_navigate: &String,
        req: Request<Body>,
    ) -> Result<Response<Body>, hyper::Error> {
        let original_uri = req.uri().clone();

        let mut new_uri = format!("https://{}{}", endpoint_to_navigate, original_uri.path());
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
}
