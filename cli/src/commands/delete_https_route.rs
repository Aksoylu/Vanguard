use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        delete_https_route_request::DeleteHttpsRouteRequest,
        delete_https_route_response::DeleteHttpsRouteResponse,
    },
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct DeleteHttpsRouteArgs {
    pub source: String,
}

pub async fn delete_https_route(args: DeleteHttpsRouteArgs) {
    let delete_https_route_request: DeleteHttpsRouteRequest = DeleteHttpsRouteRequest {
        source: args.source.clone(),
    };

    let result = execute(delete_https_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let response = result.unwrap();
    println!("response: {:?}", response);

    if response.code == StatusCode::OK.as_u16() {
        log_info!("Https route deleted successfully");
    } else {
        log_error!(
            "Error while deleting https route: {}. Details: {}",
            args.source,
            response.message
        )
    }
}

async fn execute(input: DeleteHttpsRouteRequest) -> Result<DeleteHttpsRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client
            .call("delete_https_route", serialized_input)
            .await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(DeleteHttpsRouteResponse {
            code: code.to_owned() as u16,
            message: message.to_owned(),
        })
    }?;

    Ok(lock)
}
