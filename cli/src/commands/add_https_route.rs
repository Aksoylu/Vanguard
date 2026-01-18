use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        add_https_route_request::AddHttpsRouteRequest,
        add_https_route_response::AddHttpsRouteResponse,
    },
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct AddHttpsRouteArgs {
    pub source: String,
    pub target: String,
    pub ssl_cert_path: String,
    pub ssl_private_key_path: String,
}

pub async fn add_https_route(args: AddHttpsRouteArgs) {
    let add_https_route_request = AddHttpsRouteRequest {
        source: args.source,
        target: args.target,
        ssl_cert_path: args.ssl_cert_path,
        ssl_private_key_path: args.ssl_private_key_path,
    };

    let result = execute(add_https_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let response = result.unwrap();

    if response.code == StatusCode::OK.as_u16() {
        log_info!("New https route added successfully");
    } else {
        log_error!("Error while adding new https route: {}", response.message)
    }
}

async fn execute(input: AddHttpsRouteRequest) -> Result<AddHttpsRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("add_https_route", serialized_input).await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(AddHttpsRouteResponse {
            code: code.to_owned() as u16,
            message: message.to_owned(),
            data: result.get("data").cloned(),
        })
    }?;

    Ok(lock)
}
