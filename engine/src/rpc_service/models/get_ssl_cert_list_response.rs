use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::tls_utility::SSlFileType;

#[derive(Serialize, Deserialize, Clone)]
pub struct SslCertEntity {
    pub domain: String,
    pub files: Vec<SSlFileType>,
}

#[derive(Serialize, Deserialize)]
pub struct ListSslCertResponse {
    code: u16,
    message: String,
    data: Vec<SslCertEntity>,
}

impl ListSslCertResponse {
    pub fn build(data: Vec<SslCertEntity>) -> jsonrpc_core::Value {
        let response = ListSslCertResponse {
            code: StatusCode::OK.as_u16(),
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
