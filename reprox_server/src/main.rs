mod core;
mod utils;
mod models;

use core::init::HttpServer;
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
    
    http_server.start().await;
}
