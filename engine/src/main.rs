mod assets;
mod boot;
mod constants;
mod core;
mod models;
mod render;
mod resources;
mod rpc_service;
mod utils;

use boot::Boot;

use crate::assets::banner::print_banner;
use crate::assets::startup_disclaimer::print_startup_disclaimer;

use crate::core::shared_memory::{HTTPS_SERVER, HTTP_SERVER, RPC_SERVER};
use crate::utils::boot_display_utility::BootDisplayUtility;

#[tokio::main]
async fn main() {
    print_startup_disclaimer();
    print_banner();

    let boot_result = Boot::init();

    let boot_display = BootDisplayUtility::init(boot_result);
    boot_display.render();

    let http_server = HTTP_SERVER.read().unwrap().clone();
    tokio::spawn(async move {
        http_server.start().await;
    });

    let https_server = HTTPS_SERVER.read().unwrap().clone();
    tokio::spawn(async move {
        https_server.start().await;
    });

    let jrpc_server = RPC_SERVER.read().unwrap().clone();
    jrpc_server.start().await;
}
