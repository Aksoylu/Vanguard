mod core;
mod models;
mod rpc_service;
mod utils;

use core::http_server::HttpServer;
use rpc_service::rpc_server::RPCServer;
use utils::{config, routing};

#[tokio::main]
async fn main() {
    let config = config::Environments::load();
    let mut routes = routing::Routing::load();

    let http_server = HttpServer::singleton(
        config.http_server_ip_address.clone(),
        config.http_server_port,
        routes.get(),
    );

    tokio::spawn(async move {
        http_server.start().await;
    });

    if config.rpc_enabled {
        let jsonrpc_server = RPCServer::singleton(
            config.http_server_ip_address,
            config.rpc_server_port,
            config.rpc_key,
        );

        jsonrpc_server.start().await;
    }


}
