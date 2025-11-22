use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::{Deserialize, Serialize};
use crate::{models::ssl_entity::SslEntity, rpc_service::rpc_status_message::RpcStatusMessage};


#[derive(Serialize, Deserialize)]
pub struct GetUplodadedSslEntityListResponse {
    code: u16,
    message: String,
    data: Option<Value>,
}

impl GetUplodadedSslEntityListResponse {
    pub fn build(ssl_entity_list: Vec<SslEntity>) -> jsonrpc_core::Value {

        let data = serde_json::to_value(&ssl_entity_list).ok();

        let response = GetUplodadedSslEntityListResponse {
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
