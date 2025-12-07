use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::{
        commands::{add_http_route_request::AddHttpRouteRequest, add_http_route_response::AddHttpRouteResponse, echo_request::EchoRequest, echo_response::EchoResponse},
        rpc::{rpc_params::RPCParams, rpc_payload::RPCPayload, rpc_request::RPCRequest},
    },
};
use clap::Args;

#[derive(Debug, Args)]
pub struct AddHttpRouteArgs {
    pub source: String,
    pub target: String
}

pub async fn add_http_route(args: AddHttpRouteArgs) {
    let add_http_route_request = AddHttpRouteRequest {
        source: args.source,
        target: args.target,
    };

    let result = execute(add_http_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let echo_response = result.unwrap();

    log_info!(
        "Echo answer from Vanguard Engine: {}",
        echo_response.message
    );
}

// todo here
async fn execute(input: AddHttpRouteRequest) -> Result<AddHttpRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("add_http_route", serialized_input).await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(AddHttpRouteResponse {
            code: code.to_owned(),
            message: message.to_owned(),
        })
    }?;

    Ok(lock)
}
