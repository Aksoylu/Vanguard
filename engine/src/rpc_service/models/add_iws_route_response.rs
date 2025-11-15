use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddIwsRouteResponse {
    code: i32,
    message: String
}

impl AddIwsRouteResponse {
    pub fn build(message: String) -> jsonrpc_core::Value {
        let response = AddIwsRouteResponse {
            code: 200,
            message
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
