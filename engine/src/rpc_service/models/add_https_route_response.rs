use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;
use hyper::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct AddHttpsRouteResponse {
    code: u16,
    message: String,
    data: Option<Value>,
}

impl AddHttpsRouteResponse {
    pub fn build(message: String, data: Option<Value>) -> jsonrpc_core::Value {
        let response = AddHttpsRouteResponse {
            code: StatusCode::OK.as_u16(),
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
