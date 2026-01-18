use crate::{models::ssl_file::SSlFile, rpc_service::rpc_status_message::RpcStatusMessage};
use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetUploadedSslFileListResponse {
    pub code: u16,
    pub message: String,
    pub ssl_file_list: Vec<SSlFile>,
}

impl GetUploadedSslFileListResponse {
    pub fn build(data: Vec<SSlFile>) -> Value {
        let response = GetUploadedSslFileListResponse {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
            ssl_file_list: data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
