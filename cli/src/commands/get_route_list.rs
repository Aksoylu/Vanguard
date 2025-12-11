use std::str::FromStr;

use crate::{
    common::enums::route_type::RouteType, core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT}, log_error, log_info, models::commands::{
        get_route_list_request::GetRouteListRequest, get_route_list_response::GetRouteListResponse,
    }
};
use clap::Args;
use hyper::StatusCode;

#[derive(Debug, Args, Clone)]
pub struct GetRouteListArgs {
    pub route_type: String,
}

pub async fn get_route_list(args: GetRouteListArgs) {
    let parsed_route_type = RouteType::from_str(&args.route_type);
    if parsed_route_type.is_err(){
        log_error!("{}", parsed_route_type.err().unwrap_or_default());
        return;
    }

    let get_http_route_list_request = GetRouteListRequest {
        route_type: parsed_route_type.unwrap()
    };

    println!("req >> {:?}", &get_http_route_list_request);

    let result = execute(get_http_route_list_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let get_http_route_list_response = result.unwrap();

    if get_http_route_list_response.code == StatusCode::OK.as_u16() {
        log_info!(
            "Http route list: {:?}",
            get_http_route_list_response.http_routes
        );
    } else {
        log_error!(
            "Error while listing http route list. Details: {}",
            get_http_route_list_response.message
        )
    }
}

async fn execute(input: GetRouteListRequest) -> Result<GetRouteListResponse, RPCBaseError> {
    let serialized_input = serde_json::to_value(input)
        .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client
            .call("get_http_route_list", serialized_input)
            .await?;
        let result = rpc_call_response.result;

        let json_string = result
            .as_str()
            .ok_or_else(|| RPCBaseError::build("Yanıt dize formatında değil"))?;

        let response: GetRouteListResponse = serde_json::from_str(json_string)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;

        Ok(response)
    }?;

    Ok(lock)
}
