use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

use crate::rpc_service::rpc_status_message::RpcStatusMessage;

#[derive(Serialize, Deserialize)]
pub struct AddSecureIwsResponse {
    code: u16,
    message: String,
}

impl AddSecureIwsResponse {
    pub fn build() -> jsonrpc_core::Value {
        let response = AddSecureIwsResponse {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
