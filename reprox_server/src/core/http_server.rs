use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

use crate::utils::parse_ip_address::parse_ip_address;

use hyper::client::HttpConnector;

#[derive(Debug, Clone)]
pub struct HttpServer {
    ip_address: String,
    port: u16,
    socket: SocketAddr,
    routes: HashMap<String, String>,
}

impl HttpServer {
    pub fn singleton(ip_address: String, port: u16, routes: HashMap<String, String>) -> Self {
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
                            Err(_) => Ok::<_, hyper::Error>(Response::new(Body::from("Error processing request"))),
                        }
                    }
                }))
            }
        });

        println!("Reprox Server started on {:?}", &self.socket);

        if let Err(e) = Server::bind(&self.socket).serve(make_svc).await {
            eprintln!("Server error: {}", e);
            return;
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

        let default_url = "".to_owned();
        let endpoint_to_navigate = self.routes.get(&request_host).unwrap_or(&default_url);

        if endpoint_to_navigate == &default_url {
            let response =
                Response::new(Body::from("Requested Reprox redirection URL not found..."));
            return Ok(response);
        }

        let response = self.navigate_url(endpoint_to_navigate, req).await;
        return response;
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
}
