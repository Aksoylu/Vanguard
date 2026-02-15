use hyper::header::{self, HeaderValue};
use hyper::StatusCode;
use hyper::{Body, Request, Response};
use std::fs::Metadata;
use std::net::IpAddr;
use std::path::PathBuf;

use crate::core::shared_memory::HTTP_PROXY_MANAGER;
use crate::models::traffic_policy::scope_traffic_policy::ScopeTrafficPolicy;
use crate::log_info;

use crate::render::Render;
use crate::utils::file_utility::{
    generate_file_tag, get_content_type, get_last_modified, is_file_exist, open_file,
};
use crate::utils::time_utility::{run_in_time_buffer, start_clock, stop_clock};
use tokio_util::io::ReaderStream;

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
        traffic_policy: &ScopeTrafficPolicy,
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

        let client = HTTP_PROXY_MANAGER.get(traffic_policy);

        let response = run_in_time_buffer(
            traffic_policy.upstream_settings.get_http_client_timeout() * 1000,
            client.request(new_request),
        )
        .await;

        if response.is_err() {
            log_info!(
                "{} |TIMEOUT| {} {} from {} to {} via ip {}",
                protocol_name,
                request_method,
                request_path,
                request_host,
                &endpoint_to_navigate,
                &client_ip
            );
            return Ok(Response::builder()
                .status(StatusCode::GATEWAY_TIMEOUT)
                .body(Body::from(Render::internal_server_error(
                    request_host,
                    "Upstream request timed out",
                )))
                .unwrap());
        }

        let response = response.unwrap()?;

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
        metadata: &Metadata,
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

        // Getting file size and last modified info, then we can create a etag
        let content_type = get_content_type(serving_path);
        let content_length = metadata.len();
        let last_modified = get_last_modified(metadata);

        let file_etag = generate_file_tag(content_length, last_modified);

        // Is file up to date in client, we should return 301 NOT_MODIFIED
        if let Some(if_none_match) = req.headers().get(header::IF_NONE_MATCH) {
            if if_none_match == file_etag.as_str() {
                return Ok(Response::builder()
                    .status(StatusCode::NOT_MODIFIED)
                    .body(Body::empty())
                    .unwrap());
            }
        }

        let file_pointer = open_file(serving_path).await;
        if file_pointer.is_none() {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap());
        }

        // Zero-copy streaming body
        let content_stream = ReaderStream::new(file_pointer.unwrap());
        let body = Body::wrap_stream(content_stream);

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

        Ok(Response::builder()
            .header("Content-Type", content_type.as_ref())
            .header("Content-Length", content_length.to_string())
            .header("ETag", file_etag)
            .header("Connection", "keep-alive")
            .body(body)
            .unwrap())
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
        let start_time = start_clock();

        let protocol_name = match protocol {
            Protocol::HTTP => "HTTP",
            Protocol::HTTPS => "HTTPS",
        };

        let index_html_path = serving_path.join("index.html");
        if is_file_exist(&index_html_path) {
            if let Ok(metadata) = tokio::fs::metadata(&index_html_path).await {
                return CommonHandler::iws_static_file_execution(
                    protocol,
                    request_host,
                    &index_html_path,
                    &metadata,
                    req,
                    client_ip,
                )
                .await;
            }
        }

        let original_uri = req.uri().clone();
        let url_path = original_uri.path().strip_prefix("/").unwrap_or("");
        let request_method = req.method().clone();
        let request_path = original_uri.path().to_string();

        let absolute_path = serving_path.join("index.html");

        let dir_content: String = Render::directory_explorer_page(&absolute_path, url_path);

        let elapsed_time = stop_clock(start_time);

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

        let content_length = dir_content.len();

        Ok(Response::builder()
            .header("Content-Type", "text/html")
            .header("Content-Length", content_length.to_string())
            .header("Connection", "keep-alive")
            .body(Body::from(dir_content))
            .unwrap())
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
            request_host,
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

        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(internal_server_error_content))
            .unwrap())
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
            request_host,
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

        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(internal_server_error_content))
            .unwrap())
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
            request_host,
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

        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(internal_server_error_content))
            .unwrap())
    }
}
