use crate::models::route::JsonRoute;
use crate::rpc_service::rpc_status_message::RpcStatusMessage;
use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct GetHttpRouteListResponse {
    code: u16,
    message: String,
    data: Vec<JsonRoute>,
}

impl GetHttpRouteListResponse {
    pub fn build(data: Vec<JsonRoute>) -> jsonrpc_core::Value {
        let response = GetHttpRouteListResponse {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
