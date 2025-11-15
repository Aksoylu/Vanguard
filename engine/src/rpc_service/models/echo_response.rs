use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use rustls::internal::msgs::message;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct EchoResponse {
    code: i32,
    message: String,
}

impl EchoResponse {
    pub fn build(message: String) -> jsonrpc_core::Value {
        let response = EchoResponse {
            code: 200,
            message: message,
        };

        let serialized_json: String = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
