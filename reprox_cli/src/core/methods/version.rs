use serde::{de::value::Error, Deserialize, Serialize};
use serde_json::from_str;

use crate::{build::Build, core::rpc_client::RpcClient};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EngineVersionResponse {
    pub version_name: String,
    pub version_number: f32,
}

pub async fn version(rpc_client: Option<&RpcClient>) {
    print_cli_version();
    print_engine_version(rpc_client);
}

pub fn print_cli_version() {
    println!(
        "\nReprox CLI Version: {} ({})",
        Build::VERSION_NUMBER,
        Build::VERSION_NAME
    );
}

pub async fn print_engine_version(rpc_client: Option<&RpcClient>) {
    if rpc_client.is_none() {
        println!("Please run 'Connect' for  connect Reprox Engine manually. ");
        return;
    }

    let version_info = rpc_version(rpc_client.unwrap()).await;

    match version_info {
        Some(engine_version) => {
            println!(
                "\nReprox Engine Version: {} ({})",
                engine_version.version_number, engine_version.version_name,
            );
        }
        None => {
            println!("\nCould not fetch Reprox Engine Version. Please check your connection");
        }
    }
}

async fn rpc_version(rpc_client: &RpcClient) -> Option<EngineVersionResponse> {
    match rpc_client.send_rpc("echo".to_string(), None).await {
        Ok(output) => {
            let session: EngineVersionResponse = from_str(&output).unwrap_or_default();
            Some(session)
        }
        Err(_) => None,
    }
}
