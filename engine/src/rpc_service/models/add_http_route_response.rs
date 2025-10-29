use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddHttpRouteResponse {
    code: i32,
    message: String,
    data: Option<Value>,
}

impl AddHttpRouteResponse {
    pub fn build(message: String, data: Option<Value>) -> jsonrpc_core::Value {
        let response = AddHttpRouteResponse {
            code: 200,
            message,
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
