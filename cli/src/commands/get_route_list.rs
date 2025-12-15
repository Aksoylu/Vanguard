use std::{collections::HashMap, str::FromStr};

use crate::{
    common::enums::route_type::RouteType,
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error,
    models::{
        commands::{
            get_route_list_request::GetRouteListRequest,
            get_route_list_response::GetRouteListResponse,
        },
        entity::{
            http_route::HttpRoute, https_route::HttpsRoute, iws_route::IwsRoute,
            secure_iws_route::SecureIwsRoute,
        },
    },
    utils::console::{print_colored, separator},
};
use clap::Args;
use crossterm::style::Color;
use hyper::StatusCode;

#[derive(Debug, Args, Clone)]
pub struct GetRouteListArgs {
    pub route_type: String,
}

pub async fn get_route_list(args: GetRouteListArgs) {
    let parsed_route_type = RouteType::from_str(&args.route_type);
    if parsed_route_type.is_err() {
        log_error!("{}", parsed_route_type.err().unwrap_or_default());
        return;
    }

    let route_type = parsed_route_type.unwrap();

    let get_http_route_list_request = GetRouteListRequest {
        route_type: route_type.clone(),
    };

    let result = execute(get_http_route_list_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let get_route_list_response = result.unwrap();

    if get_route_list_response.code != StatusCode::OK.as_u16() {
        log_error!(
            "Error while fetching route list from Vanguard Engine. Details: {}",
            get_route_list_response.message
        );

        return;
    }

    separator(36);
    if route_type == RouteType::Http || route_type == RouteType::All {
        print_http_routes(get_route_list_response.http_routes);
    }
    if route_type == RouteType::Https || route_type == RouteType::All {
        print_https_routes(get_route_list_response.https_routes);
    }
    if route_type == RouteType::Iws || route_type == RouteType::All {
        print_iws_routes(get_route_list_response.iws_routes);
    }
    if route_type == RouteType::SecureIws || route_type == RouteType::All {
        print_secure_iws_routes(get_route_list_response.secure_iws_routes);
    }
    separator(36);
}

async fn execute(input: GetRouteListRequest) -> Result<GetRouteListResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("get_route_list", serialized_input).await?;
        let result = rpc_call_response.result;

        let json_string = result
            .as_str()
            .ok_or_else(|| RPCBaseError::build("Yanıt dize formatında değil"))?;

        let response: GetRouteListResponse = serde_json::from_str(json_string)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;

        Ok(response)
    }?;

    Ok(lock)
}

// ----------------------------------------------------
// --- Print Functions Implementation -------------------
// ----------------------------------------------------

fn print_http_routes(http_routes: Option<HashMap<String, HttpRoute>>) {
    if http_routes.is_none() {
        return;
    }

    let routes = http_routes.unwrap();

    println!("\n--- HTTP Routes ({}) ---", routes.len());
    if routes.is_empty() {
        println!("  (No HTTP routes defined)");
        return;
    }

    for (i, (domain, route)) in routes.iter().enumerate() {
        let index = format!("#{}", i + 1);
        print_colored(index.as_str(), Color::Yellow);
        println!("  Domain: {}", domain);
        println!("  Target: {}", route.target);
    }
}

fn print_https_routes(https_routes: Option<HashMap<String, HttpsRoute>>) {
    if https_routes.is_none() {
        return;
    }

    let routes = https_routes.unwrap();

    println!("\n--- HTTPS Routes ({}) ---", routes.len());
    if routes.is_empty() {
        println!("  (No HTTPS routes defined)");
        return;
    }

    for (i, (domain, route)) in routes.iter().enumerate() {
        let ssl_cert_path = &route.ssl_context.certificate_file_path;
        let ssl_private_key_path = &route.ssl_context.private_key_file_path;

        let index = format!("#{}", i + 1);
        print_colored(index.as_str(), Color::Yellow);
        println!("  Domain: {}", domain);
        println!("  Target: {}", route.target);
        println!("  SSL Certificate path: {}", ssl_cert_path);
        println!("  SSL Private Key path: {}", ssl_private_key_path);
    }
}

fn print_iws_routes(iws_routes: Option<HashMap<String, IwsRoute>>) {
    if iws_routes.is_none() {
        return;
    }
    let routes = iws_routes.unwrap();

    println!(
        "\n--- Integrated Web Server (IWS) Routes ({}) ---",
        routes.len()
    );
    if routes.is_empty() {
        println!("  (No IWS routes defined)");
        return;
    }

    for (i, (domain, route)) in routes.iter().enumerate() {
        let index = format!("#{}", i + 1);
        print_colored(index.as_str(), Color::Yellow);
        println!("  Domain: {}", domain);
        println!("  Serving Path: {}", route.serving_path);
    }
}

fn print_secure_iws_routes(secure_iws_routes: Option<HashMap<String, SecureIwsRoute>>) {
    if secure_iws_routes.is_none() {
        return;
    }

    let routes = secure_iws_routes.unwrap();

    println!(
        "\n--- Secure Integrated Web Server (IWS) ({}) ---",
        routes.len()
    );
    if routes.is_empty() {
        println!("  (No Secure IWS routes defined)");
        return;
    }

    for (i, (domain, route)) in routes.iter().enumerate() {
        let ssl_cert_path = &route.ssl_context.certificate_file_path;
        let ssl_private_key_path = &route.ssl_context.private_key_file_path;
        let index = format!("#{}", i + 1);
        print_colored(index.as_str(), Color::Yellow);
        println!("  Domain: {}", domain);
        println!("  Target: {}", route.serving_path);
        println!("  SSL Certificate path: {}", ssl_cert_path);
        println!("  SSL Private Key path: {}", ssl_private_key_path);
    }
}
