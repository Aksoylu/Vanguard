use serde::de::value::Error;

use crate::{
    core::{interprinter::CommandInterprinter, rpc_client::RpcClient},
    settings::Settings,
    utils::console::console_read,
};

pub async fn engine(command_interprinter: &mut CommandInterprinter, params: Vec<String>) {
    let default_operation: &String = &Settings::DEFAULT_OPERATION.to_string();
    let default_connection_type: &String = &Settings::DEFAULT_CONNECTION_TYPE.to_string();

    let operation = params.get(0).unwrap_or(default_operation);

    if operation.eq("establish") {
        let connection_type = params.get(1).unwrap_or(&default_connection_type);
        command_interprinter.rpc_client = establish_rpc(&connection_type).await;
    } else if operation.eq("stop") {
        stop(&command_interprinter.rpc_client).await;
    } else if operation.eq("status") {
        status(&command_interprinter.rpc_client).await;
    } else if operation.eq("help") {
        help(false);
    } else {
        help(true);
    }
}

fn help(undefined_command_flag: bool) {
    if undefined_command_flag {
        println!("Please provide a operation.");
    }

    println!(
        r#"
        === Command List ===
        Engine:
            * 'engine establish auto'
                » Tries to find locale running Reprox Engine and establish connection to engine AUTOMATICALLY 
                » Recommended for new beginners
                » Recommended for locale hosted Reprox Engine
            * engine establish manual
                » Tries to connect Reprox Engine by user given inputs.
                » You need to provide Ip Address, Port and Security Key to connect.
                » Recommended for connecting non-locale hosted Reprox Engine
            * engine status
                » Shows stats & status about Reprox Engine 
            * engine stop
                » Stops currently established Reprox Engine 
    "#
    );
}

async fn status(rpc_client: &Option<RpcClient>) {
    match rpc_client {
        Some(rpc) => {
            let result = rpc.send_rpc("status".to_string(), None).await;
            if result.is_ok() {
                println!("@todo -- status here --");
            } else {
                println!("Error: An error occured while communicating with Reprox engine")
            }
        }
        None => {
            println!("Error: Reprox engine is not established.\nPlease establish an engine connection using 'Engine establish' first")
        }
    }
}

async fn stop(rpc_client: &Option<RpcClient>) {
    match rpc_client {
        Some(rpc) => {
            let result = rpc.send_rpc("stop".to_string(), None).await;
            if result.is_ok() {
                println!("@todo -- show shutdown message here --");
            } else {
                println!("Error: An error occured while communicating with Reprox engine")
            }
        }
        None => {
            println!("Error: Reprox engine is not established.\nPlease establish an engine connection using 'Engine establish' first")
        }
    }
}

async fn establish_rpc(connection_type: &String) -> Option<RpcClient> {
    let automatic_connection_type = "automatic".to_string();
    let manual_connection_type = "manual".to_string();

    if connection_type.eq(&manual_connection_type) {
        let rpc_client = connect_rpc_automatically().await;
        return rpc_client;
    } else if connection_type.eq(&automatic_connection_type) {
        let ip_addr: String = console_read("Ip Address:");
        let port: String = console_read("Port:");
        let security_hash: String = console_read("Security Hash:");

        let rpc_client = connect_rpc_manually(&ip_addr, &port, &security_hash).await;
        return rpc_client;
    }

    None
}

async fn connect_rpc_automatically() -> Option<RpcClient> {
    match RpcClient::init_session() {
        Ok(rpc_client) => {
            let result = rpc_client.send_rpc("test".to_string(), None).await;
            if result.is_ok() {
                return Some(rpc_client);
            } else {
                return None;
            }
        }
        Err(_) => {
            println!("Failed to connect Reprox Engine automatically. Please be sure thar Reprox Engine is running");
            return None;
        }
    }
}

async fn connect_rpc_manually(
    ip_addr: &String,
    port: &String,
    security_hash: &String,
) -> Option<RpcClient> {
    if ip_addr.len() < 1 || port.len() < 1 || security_hash.len() < 1 {
        println!("Please provide valid Ip Address, Port Number and Security Hash");
        return None;
    }

    match RpcClient::init_manual() {
        Ok(rpc_client) => {
            let result = rpc_client.send_rpc("test".to_string(), None).await;
            if result.is_ok() {
                return Some(rpc_client);
            } else {
                return None;
            }
        }
        Err(_) => {
            println!("Failed to connect Reprox Engine automatically. Please be sure thar Reprox Engine is running");
            return None;
        }
    }
}
