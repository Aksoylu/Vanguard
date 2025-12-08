use hyper::StatusCode;
use jsonrpc_core::{Error, ErrorCode, Value};
use serde::{Deserialize, Serialize};

use crate::rpc_service::rpc_status_message::RpcStatusMessage;

#[derive(Serialize, Deserialize)]
pub struct AddHttpRouteResponse {
    code: u16,
    message: String
}

impl AddHttpRouteResponse {
    pub fn build() -> Result<Value, Error> {
        let response = AddHttpRouteResponse {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
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
