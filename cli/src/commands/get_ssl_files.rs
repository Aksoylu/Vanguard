use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error,
    models::{
        commands::get_uploaded_ssl_file_list_response::GetUploadedSslFileListResponse,
        entity::ssl_file::SSlFile,
    },
    utils::{console::{print_colored, separator}, json_utility::create_empty_json_object},
};
use clap::Args;
use crossterm::style::Color;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct GetSslFilesArgs {}

pub async fn get_ssl_files(_args: GetSslFilesArgs) {
    let result = execute().await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let response = result.unwrap();

    if response.code != StatusCode::OK.as_u16() {
        log_error!(
            "Error while fetching SSL file list. Details: {}",
            response.message
        );
        return;
    }

    print_ssl_files(response.ssl_file_list);
}

async fn execute() -> Result<GetUploadedSslFileListResponse, RPCBaseError> {
    let serialized_input = create_empty_json_object();

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        // Assuming the RPC method name is "get_uploaded_ssl_file_list"
        let rpc_call_response = rpc_client
            .call("get_uploaded_ssl_file_list", serialized_input)
            .await?;
        let result = rpc_call_response.result;

        // Handling potential different response structures (string vs object)
        // Similar to get_route_list, checking if it returns a string first
        if let Some(json_string) = result.as_str() {
             let response: GetUploadedSslFileListResponse = serde_json::from_str(json_string)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;
             Ok(response)
        } else {
            // If it's already an object, try to deserialize directly
             let response: GetUploadedSslFileListResponse = serde_json::from_value(result)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;
             Ok(response)
        }
    }?;

    Ok(lock)
}

fn print_ssl_files(ssl_files: Vec<SSlFile>) {
    separator(36);
    println!("\n--- Uploaded SSL Files ({}) ---", ssl_files.len());

    if ssl_files.is_empty() {
        println!("  (No SSL files found)");
        separator(36);
        return;
    }

    for (i, ssl_file) in ssl_files.iter().enumerate() {
        let index = format!("#{}", i + 1);
        print_colored(index.as_str(), Color::Yellow);
        println!("  Domain: {}", ssl_file.domain);
        println!("  Type: {:?}", ssl_file.file_type);
    }
    separator(36);
}
