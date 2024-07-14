mod constants;
mod core;
mod models;
mod rpc_service;
mod runtime;
mod utils;

use core::{http_server::HttpServer, https_server::HttpsServer};
use rpc_service::rpc_server::RPCServer;
use runtime::Runtime;
    // TODO: make runtime dynamically changable with jrpc
    // TODO: make runtime uses constant paths. If not exist, create folders and files by using defaults
#[tokio::main]
async fn main() {
    //TODO: make this object can reachable from all tokio threads
    //TODO: make inform other threads if any change happens in runtime object
    let runtime = Runtime::init();

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

    tokio::spawn(async move {
        http_server.start().await;
    });

    let jsonrpc_server = RPCServer::singleton(
        runtime.config.rpc_server.ip_address,
        runtime.config.rpc_server.port,
        runtime.config.rpc_server.private_key,
    );

    jsonrpc_server.start().await;
}
