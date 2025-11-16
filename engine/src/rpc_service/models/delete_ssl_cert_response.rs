use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::{Deserialize, Serialize};

use crate::rpc_service::rpc_status_message::RpcStatusMessage;

#[derive(Serialize, Deserialize)]
pub struct DeleteSSlCertResponse {
    code: u16,
    message: String,
    data: Vec<String>,
}

impl DeleteSSlCertResponse {
    pub fn build(data: Vec<String>) -> jsonrpc_core::Value {
        let response = DeleteSSlCertResponse {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
            data
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
