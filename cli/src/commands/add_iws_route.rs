use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        add_iws_route_request::AddIwsRouteRequest, add_iws_route_response::AddIwsRouteResponse,
    },
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args)]
pub struct AddIwsRouteArgs {
    pub source: String,
    pub serving_path: String,
}

pub async fn add_iws_route(args: AddIwsRouteArgs) {
    let add_iws_route_request = AddIwsRouteRequest {
        source: args.source,
        serving_path: args.serving_path,
    };

    let result = execute(add_iws_route_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let response = result.unwrap();

    if response.code == StatusCode::OK.as_u16() {
        log_info!("New IWS (Internal Web Server) route added successfully");
    } else {
        log_error!(
            "An error occured while adding a new IWS (Internal Web Server) route: {}",
            response.message
        )
    }
}

async fn execute(input: AddIwsRouteRequest) -> Result<AddIwsRouteResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;
    
    println!("{:?}", serialized_input);

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("add_iws_route", serialized_input).await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();
        let message = &result["message"].as_str().unwrap_or_default().to_string();

        Ok(AddIwsRouteResponse {
            code: code.to_owned() as u16,
            message: message.to_owned(),
        })
    }?;

    Ok(lock)
}
