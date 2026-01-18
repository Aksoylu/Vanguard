use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        delete_iws_route_request::DeleteIwsRouteRequest,
        delete_iws_route_response::DeleteIwsRouteResponse,
    },
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct DeleteIwsRouteArgs {
    pub source: String,
}

pub async fn delete_iws_route(args: DeleteIwsRouteArgs) {
    let delete_iws_route_request = DeleteIwsRouteRequest {
        source: args.source.clone(),
    };

    let result = execute(delete_iws_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let response = result.unwrap();

    if response.code == StatusCode::OK.as_u16() {
        log_info!("IWS route deleted successfully");
    } else {
        log_error!(
            "Error while deleting IWS route: {}. Details: {}",
            args.source,
            response.message
        )
    }
}

async fn execute(input: DeleteIwsRouteRequest) -> Result<DeleteIwsRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client
            .call("delete_iws_route", serialized_input)
            .await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(DeleteIwsRouteResponse {
            code: code.to_owned() as u16,
            message: message.to_owned(),
        })
    }?;

    Ok(lock)
}
