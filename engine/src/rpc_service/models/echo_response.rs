use hyper::StatusCode;
use jsonrpc_core::{Error, ErrorCode, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EchoResponse {
    code: u16,
    message: String,
}

impl EchoResponse {
    pub fn build(message: String) -> Result<Value, Error> {
        let response = EchoResponse {
            code: StatusCode::OK.as_u16(),
            message: message,
        };

        let response_as_json = serde_json::to_value(response).map_err(|error_details| {
            return Error {
                code: ErrorCode::InternalError,
                message: error_details.to_string(),
                data: None,
            };
        })?;

        Ok(response_as_json)
    }
}
