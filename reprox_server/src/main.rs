mod config;
mod core;
mod rpc_service;
mod settings;
mod utils;

use config::Config;
use core::{
    http_server::HttpServer,
    https_server::HttpsServer,
    router::Router,
};
use rpc_service::rpc_server::RPCServer;

#[tokio::main]
async fn main() {
    let config: Config = Config::load();
    let routes = Router::load();

    let http_server = HttpServer::singleton(
        config.http_server.ip_address.clone(),
        config.http_server.port,
        routes.get_http_routes(),
    );

    tokio::spawn(async move {
        http_server.start().await;
    });

    let https_server = HttpsServer::singleton(
        config.https_server.ip_address.clone(),
        config.https_server.port,
        routes.get_https_routes(),
    );

    tokio::spawn(async move {
        https_server.start().await;
    });

    match config.rpc_server {
        Some(rpc_config) => {
            let jsonrpc_server = RPCServer::singleton(
                rpc_config.ip_address,
                rpc_config.port,
                rpc_config.private_key,
            );
            jsonrpc_server.start().await;
        }
        None => {}
    }
}
