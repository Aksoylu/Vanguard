use crate::{
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::commands::{
        get_http_route_list_request::GetHttpRouteListRequest,
        get_http_route_list_response::GetHttpRouteListResponse,
    },
};
use hyper::StatusCode;

pub async fn get_http_route_list() {
    let get_http_route_list_request = GetHttpRouteListRequest {};

    let result = execute(get_http_route_list_request).await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let get_http_route_list_response = result.unwrap();

    if get_http_route_list_response.code == StatusCode::OK.as_u16() {
        log_info!("Http route list: {:?}", get_http_route_list_response.data);
    } else {
        log_error!(
            "Error while listing http route list. Details: {}",
            get_http_route_list_response.message
        )
    }
}

async fn execute(input: GetHttpRouteListRequest) -> Result<GetHttpRouteListResponse, RPCBaseError> {
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
            .ok_or_else(|| RPCBaseError::build( "Yanıt dize formatında değil"))?;

        let response: GetHttpRouteListResponse = serde_json::from_str(json_string)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;

        Ok(response)
    }?;

    Ok(lock)
}
