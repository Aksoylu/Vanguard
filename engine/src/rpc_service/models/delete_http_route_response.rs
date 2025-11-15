use jsonrpc_core::{ Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DeleteHttpRouteResponse {
    code: i32,
    message: String
}

impl DeleteHttpRouteResponse {
    pub fn build() -> jsonrpc_core::Value {
        let response = DeleteHttpRouteResponse {
            code: 200,
            message: "ok".to_owned()
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
