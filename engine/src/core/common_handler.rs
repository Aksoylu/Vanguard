use hyper::client::HttpConnector;
use hyper::header::HeaderValue;
use hyper::StatusCode;
use hyper::{Body, Client, Request, Response};
use std::net::IpAddr;
use std::path::PathBuf;

use crate::log_info;

use crate::render::Render;
use crate::utils::file_utility::{get_content_type, is_file_exist, read_file_as_binary};

pub enum Protocol {
    HTTP,
    HTTPS,
}

pub struct CommonHandler {}

impl CommonHandler {
    pub async fn url_execution(
        protocol: Protocol,
        request_host: &String,
        endpoint_to_navigate: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
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
            "{} |EXECUTION| {} {} {} ({} ms) from {} to {} via ip {}",
            protocol_name,
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

    pub async fn iws_static_file_execution(
        protocol: Protocol,
        request_host: &String,
        serving_path: &PathBuf,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_path = original_uri.path().to_string();

        let file_content: Option<Vec<u8>> = read_file_as_binary(&serving_path).await;

        let elapsed_time = start_time.elapsed().as_millis();

        if file_content.is_none() {
            log_info!(
                "{} |IWS RECORD FOUND| {} {} {} ({} ms) from {} via ip {}",
                protocol_name,
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
            "{} |IWS EXECUTION| {} {} {} ({} ms) from {} to {} via ip {}",
            protocol_name,
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
    pub async fn iws_static_directory_execution(
        protocol: Protocol,
        request_host: &String,
        serving_path: &PathBuf,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let index_html_path = serving_path.join("index.html");
        if is_file_exist(&index_html_path) {
            return CommonHandler::iws_static_file_execution(
                protocol,
                request_host,
                &index_html_path,
                req,
                client_ip,
            )
            .await;
        }

        let original_uri = req.uri().clone();
        let url_path = original_uri.path().strip_prefix("/").unwrap_or("");
        let request_method = req.method().clone();
        let request_path = original_uri.path().to_string();

        let absolute_path = serving_path.join("index.html");

        let dir_content: String = Render::directory_explorer_page(&absolute_path, &url_path);

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "{} |IWS EXECUTION| {} {} {} ({} ms) from {} to {} via ip {}",
            protocol_name,
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

    pub async fn not_found_error(
        protocol: Protocol,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_path = original_uri.path().to_string();

        let internal_server_error_content = Render::internal_server_error(
            &request_host,
            format!(
                "Requested domain/target is not assigned to a valid {} source",
                protocol_name
            )
            .as_str(),
        );

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "{} |ROUTE NOT FOUND| {} {} {} ({} ms) from {} via ip {}",
            protocol_name,
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

    pub async fn iws_empty_path_error(
        protocol: Protocol,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_path = original_uri.path().to_string();

        let internal_server_error_content = Render::internal_server_error(
            &request_host,
            format!(
                "Requested domain/target is not assigned to a valid {} source",
                protocol_name
            )
            .as_str(),
        );

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "{} |IWS EMPTY EXECUTION| {} {} {} ({} ms) from {} via ip {}",
            protocol_name,
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

    pub async fn iws_route_not_found_error(
        protocol: Protocol,
        request_host: &String,
        req: Request<Body>,
        client_ip: IpAddr,
    ) -> Result<Response<Body>, hyper::Error> {
        let start_time: std::time::Instant = std::time::Instant::now();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let original_uri = req.uri().clone();
        let request_method = req.method().clone();
        let request_path = original_uri.path().to_string();

        let internal_server_error_content = Render::internal_server_error(
            &request_host,
            format!(
                "Requested domain/target is not assigned to a valid {} source",
                protocol_name
            )
            .as_str(),
        );

        let elapsed_time = start_time.elapsed().as_millis();

        log_info!(
            "{} |IWS RECORD NOT FOUND| {} {} {} ({} ms) from {} via ip {}",
            protocol_name,
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
