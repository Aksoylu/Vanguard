use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        delete_secure_iws_route_request::DeleteSecureIwsRouteRequest,
        delete_secure_iws_route_response::DeleteSecureIwsRouteResponse,
    },
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct DeleteSecureIwsRouteArgs {
    pub source: String,
}

pub async fn delete_secure_iws_route(args: DeleteSecureIwsRouteArgs) {
    let delete_secure_iws_route_request = DeleteSecureIwsRouteRequest {
        source: args.source.clone(),
    };

    let result = execute(delete_secure_iws_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let response = result.unwrap();

    if response.code == StatusCode::OK.as_u16() {
        log_info!("Secure IWS route deleted successfully");
    } else {
        log_error!(
            "Error while deleting Secure IWS route: {}. Details: {}",
            args.source,
            response.message
        )
    }
}

async fn execute(
    input: DeleteSecureIwsRouteRequest,
) -> Result<DeleteSecureIwsRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client
            .call("delete_secure_iws_route", serialized_input)
            .await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(DeleteSecureIwsRouteResponse {
            code: code.to_owned() as u16,
            message: message.to_owned(),
        })
    }?;

    Ok(lock)
}
