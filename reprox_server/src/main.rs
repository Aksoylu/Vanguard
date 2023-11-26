
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

    let http_server_task = tokio::spawn(async move {
        http_server.start().await;
    });

    let jsonrpc_server_task = if &config.clone().rpc_enabled == &true {
        let jsonrpc_server = RPCServer::singleton(
            &config.clone().http_server_ip_address,
            &config.clone().rpc_server_port,
            &config.clone().rpc_key,
        );
        
        tokio::spawn(async move {
            jsonrpc_server.start();
        })
    } else {
        // Return a dummy task if RPC is not enabled
        tokio::spawn(async {})
    };

    tokio::try_join!(http_server_task, jsonrpc_server_task)
        .expect("Failed to run servers");
}