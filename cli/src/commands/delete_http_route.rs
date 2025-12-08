use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        delete_http_route_request::DeleteHttpRouteRequest, delete_http_route_response::DeleteHttpRouteResponse,
    },
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct DeleteHttpRouteArgs {
    pub source: String,
}

pub async fn delete_http_route(args: DeleteHttpRouteArgs) {
    let delete_http_route_request: DeleteHttpRouteRequest = DeleteHttpRouteRequest {
        source: args.source.clone(),
    };

    let result = execute(delete_http_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let delete_http_route_response = result.unwrap();

    if delete_http_route_response.code == StatusCode::OK.as_u16() {
        log_info!("Http route deleted successfully");
    } else {
        log_error!(
            "Error while deleting http route: {}. Details: {}",
            args.source,
            delete_http_route_response.message
        )
    }
}

async fn execute(input: DeleteHttpRouteRequest) -> Result<DeleteHttpRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("delete_http_route", serialized_input).await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(DeleteHttpRouteResponse {
            code: code.to_owned() as u16,
            message: message.to_owned(),
        })
    }?;

    Ok(lock)
}
