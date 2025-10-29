use crate::models::route::JsonRoute;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct GetHttpRouteListResponse {
    code: i32,
    message: String,
    data: Vec<JsonRoute>,
}

impl GetHttpRouteListResponse {
    pub fn build(data: Vec<JsonRoute>) -> jsonrpc_core::Value {
        let response = GetHttpRouteListResponse {
            code: 200,
            message: "ok".to_owned(),
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
