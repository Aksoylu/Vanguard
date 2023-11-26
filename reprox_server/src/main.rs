
mod core;
mod utils;
mod models;
mod rpc_service;

use core::http_server::HttpServer;
use rpc_service::{rpc_server::RPCServer, RPC_ROUTER};
use utils::{config, routing};


#[tokio::main]
async fn main() {
    let config = config::Environments::load();
    let routes = routing::Routing::load();

    let http_server = HttpServer::singleton(
        &config.clone().http_server_ip_address,
        &config.clone().http_server_port,
        routes.get(),
    );    

    tokio::spawn(async move {
        http_server.start().await;

    });

    if &config.clone().rpc_enabled == &true
    {
        let jsonrpc_server = RPCServer::singleton(
            &config.clone().http_server_ip_address,
            &config.clone().rpc_server_port,
            &config.clone().rpc_key,
        );

        jsonrpc_server.start().await;
    }
}
