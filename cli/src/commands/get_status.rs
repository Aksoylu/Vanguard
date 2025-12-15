use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error,
    models::commands::get_status_response::GetStatusResponse,
    utils::{console::separator, json_utility::create_empty_json_object},
};
use hyper::StatusCode;

pub async fn get_status() {
    let result = execute().await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let get_status_response = result.unwrap();

    if get_status_response.code != StatusCode::OK.as_u16() {
        log_error!("An error occured while getting status of Vanguard Engine.");
        return;
    }

    // @todo: Beautify result
    separator(36);
    println!("{:?}", get_status_response);
    separator(36);
}

async fn execute() -> Result<GetStatusResponse, RPCBaseError> {
    let request = create_empty_json_object();

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("get_status", request).await?;
        let result = rpc_call_response.result;
    
        let response: GetStatusResponse = serde_json::from_value(result)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;

        Ok(response)
    }?;

    Ok(lock)
}
