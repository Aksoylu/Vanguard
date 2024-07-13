mod constants;
mod core;
mod models;
mod rpc_service;
mod runtime;
mod utils;

use core::{http_server::HttpServer, https_server::HttpsServer};
use rpc_service::rpc_server::RPCServer;
use runtime::Runtime;

#[tokio::main]
async fn main() {
    // TODO: make runtime dynamically changable with jrpc
    // TODO: make runtime uses constant paths. If not exist, create folders and files by using defaults

    let runtime = Runtime::init();

    if runtime.config.rpc_server.is_some() {
        let rpc_server_config = runtime.config.rpc_server.unwrap();

        let jsonrpc_server = RPCServer::singleton(
            rpc_server_config.ip_address,
            rpc_server_config.port,
            rpc_server_config.private_key,
        );

        tokio::spawn(async move {
            jsonrpc_server.start().await;
        });
    }

    let https_server = HttpsServer::singleton(
        runtime.config.https_server.ip_address.clone(),
        runtime.config.https_server.port,
        runtime.router.get_https_routes(),
    );

    tokio::spawn(async move {
        https_server.start().await;
    });

    let http_server = HttpServer::singleton(
        runtime.config.http_server.ip_address.clone(),
        runtime.config.http_server.port,
        runtime.router.get_http_routes(),
    );
    http_server.start().await;
}
