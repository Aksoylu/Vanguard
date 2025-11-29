mod assets;
mod boot;
mod commands;
mod constants;
mod core;
mod models;
mod utils;

use crate::{assets::{banner::print_banner, startup_disclaimer::print_startup_disclaimer}, boot::Boot, core::{interprinter::Interprinter, rpc_client::RPCClient, shared_memory::RPC_CLIENT}, utils::console::console_read};

use utils::console::separator;


#[tokio::main]
async fn main() {
    print_startup_disclaimer();
    print_banner();
    separator(5);

    let boot_data = Boot::init();

    let mut rpc_client = RPC_CLIENT.write().unwrap();
    *rpc_client = RPCClient::init(boot_data);
    
    let interprinter = Interprinter::new();

    loop {
        let input: String = console_read(">>> "); 
        let trimmed_input = input.trim().to_lowercase();

        if trimmed_input == "exit" || trimmed_input == "quit" {
            println!("Çıkılıyor...");
            break;
        }

        interprinter.run(input).await;
    }

}
