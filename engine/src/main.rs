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
use crate::utils::console_utility::approve_dialog;

#[tokio::main]
async fn main() {
    print_startup_disclaimer();
    print_banner();

    let boot_result = Boot::init();

    if !boot_result.is_config_loaded_successfully {
        println!("Config file can not be readed. File is corrupt or JSON format is broken. Please check on path: {}",  boot_result.config_path.to_string_lossy());

        let approval =
            approve_dialog("Do you want to overwrite 'default' Vanguard Config on current [y/n]");
        if approval {
            Boot::save_config(boot_result.config_path.to_owned(), &boot_result.config);
        }
    }

    if !boot_result.is_router_loaded_successfully {
        println!( "Router file can not be readed. File is corrut or JSON format is broken. Please check on path: {}", boot_result.config_path.to_string_lossy() );

        let approval = approve_dialog(
            "Do you want to overwrite 'default' Vanguard Route file on current [y/n]",
        );
        if approval {
            Boot::save_router(boot_result.route_path.to_owned(), &boot_result.router);
        }
    }

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
