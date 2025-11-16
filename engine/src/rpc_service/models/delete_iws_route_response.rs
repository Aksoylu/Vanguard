use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

use crate::rpc_service::rpc_status_message::RpcStatusMessage;

#[derive(Serialize, Deserialize)]
pub struct DeleteIwsRouteResponse {
    code: i32,
    message: String,
}

impl DeleteIwsRouteResponse {
    pub fn build() -> jsonrpc_core::Value {
        let response = DeleteIwsRouteResponse {
            code: 200,
            message: RpcStatusMessage::OK.to_string(),
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
