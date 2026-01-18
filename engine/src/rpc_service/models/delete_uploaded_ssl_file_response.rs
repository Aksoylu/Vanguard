use crate::rpc_service::rpc_status_message::RpcStatusMessage;
use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteUploadedSslFileResponse {
    pub code: u16,
    pub message: String,
    pub is_success: bool,
}

impl DeleteUploadedSslFileResponse {
    pub fn new() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
            is_success: false,
        }
    }

    pub fn set_success(&mut self, is_success: bool) {
        self.is_success = is_success;
    }

    pub fn build(&self) -> jsonrpc_core::Value {
        let serialized_json = match serde_json::to_string(&self) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
