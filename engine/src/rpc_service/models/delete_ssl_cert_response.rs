use jsonrpc_core::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteSSlCertResponse {
    code: i32,
    message: String,
    data: Vec<String>,
}

impl DeleteSSlCertResponse {
    pub fn build(data: Vec<String>) -> jsonrpc_core::Value {
        let response = DeleteSSlCertResponse {
            code: 200,
            message: "OK".into(),
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
