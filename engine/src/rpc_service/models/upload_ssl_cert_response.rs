use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct UploadSslCertResponse {
    code: u16,
    message: String,
    data: Option<Value>,
}

impl UploadSslCertResponse {
    pub fn build(message: String, data: Option<Value>) -> jsonrpc_core::Value {
        let response = UploadSslCertResponse {
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
