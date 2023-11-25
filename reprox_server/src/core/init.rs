use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, Client, header, Uri,
};
use tokio::sync::Mutex;
use std::{net::SocketAddr, collections::HashMap, sync::Arc};

use crate::utils::parse_ip_address::parse_ip_address;

use super::static_controllers::STATIC_ROUTER;


#[derive(Debug, Clone)]
pub struct HttpServer {
    ip_address: String,
    port: u16,
    socket: SocketAddr,
    routes: HashMap<String, String>
}

// Gelen isteği yöneten fonksiyon
async fn handle_requestfn(req: Request<hyper::Body>) -> Result<Response<hyper::Body>, hyper::Error> {
    let target_uri = "http://example.com"; // Hedef sunucunun adresi
    let client = Client::new();

    // Yeni istek oluştur
    let (mut parts, body) = req.into_parts();
    let uri = format!("{}{}", target_uri, parts.uri);
    parts.uri = uri.parse().unwrap(); // unwrap: Örnek amaçlı, hata yönetimi yapılmamıştır

    // Hedef sunucuya isteği gönder
    let proxy_req = Request::from_parts(parts, body);

    let res = client.request(proxy_req).await.unwrap();
    Ok(res)
}

impl HttpServer {
    pub fn singleton(ip_address: &String, port: &u16, routes: HashMap<String, String>) -> Self {
        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let parsed_port = port.clone();

        let socket = SocketAddr::from((parsed_ip_address, parsed_port));

        Self {
            ip_address: ip_address.clone(),
            port: port.clone(),
            socket,
            routes,
        }
    }

    pub async fn start(&self) {
        let shared_data = Arc::new(Mutex::new(self.clone()));

        let make_svc = make_service_fn(move |_conn| {
            let shared_data = Arc::clone(&shared_data);

            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let shared_data = Arc::clone(&shared_data);
                    let locked_data = shared_data.clone(); // Clone Arc for inner use
                    let fut = async move {
                        let data = locked_data.lock().await; // Await lock acquisition
                        match data.handle_request(req).await {
                            Ok(response) => Ok::<_, hyper::Error>(response),
                            Err(_) => Ok::<_, hyper::Error>(Response::new(Body::from("Error processing request"))),
                        }
                    };
                    fut
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
        let request_host = req.headers().get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map_or_else(|| "/".to_string(), |host_value| host_value.to_string());

        let request_path = req.uri().to_string();

        if request_host == self.ip_address.clone()
        {
            if STATIC_ROUTER.contains_key(request_path.as_str())
            {
                let controller = STATIC_ROUTER.get(request_path.as_str()).unwrap();
                return controller(req);
            }

            let response = Response::new(Body::from("Requested URL not found..."));
            return Ok(response);
        }

        println!("request host: {}",request_host);
        println!("routes: {:?}", self.routes);
        if self.routes.contains_key(&request_host)
        {
            let url_to_navigate = self.routes.get(&request_host).unwrap();
            let response = self.navigate_url(url_to_navigate, req);
        }

        let response = Response::new(Body::from("Requested Reprox redirection URL not found..."));
        Ok(response)
    }

    async fn navigate_url(&self, url_to_navigate: &String, req: Request<Body>) -> Result<Response<Body>, hyper::Error>
    {
        if let Ok(uri) = url_to_navigate.parse::<Uri>() {
            let client = Client::new();

            let mut forwarded_req = Request::builder()
                .method(req.method())
                .uri(uri)
                .version(req.version());

            for (key, value) in req.headers() {
                forwarded_req = forwarded_req.header(key, value.clone());
            }

            let body = hyper::body::to_bytes(req.into_body()).await?;
            let forwarded_req = forwarded_req.body(Body::from(body));

            // Send the modified request to the destination URL
            let response = client.request(forwarded_req.unwrap_or_default()).await?;

            return Ok(response);
        }
       
        let error_message: String = format!("Reprox Error: endpoint not supported: {}", &url_to_navigate);
        let error_response = Response::new(Body::from(error_message));
        return Ok(error_response);
    }

}

