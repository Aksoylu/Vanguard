use hyper::StatusCode;
use jsonrpc_core::{ Value};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DeleteHttpsRouteResponse {
    code: u16,
    message: String
}

impl DeleteHttpsRouteResponse {
    pub fn build() -> jsonrpc_core::Value {
        let response = DeleteHttpsRouteResponse {
            code: StatusCode::OK.as_u16(),
            message: "ok".into()
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
