use hyper::StatusCode;
use jsonrpc_core::{Error, ErrorCode, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetBuildVersionResponse {
    code: u16,
    build_version_number: f64,
    build_version_name: String
}

impl GetBuildVersionResponse {
    pub fn build(version_number: f64, version_name: String) -> Result<Value, Error> {
        let response = GetBuildVersionResponse {
            code: StatusCode::OK.as_u16(),
            build_version_number: version_number,
            build_version_name: version_name,
        };

        let response_as_json = serde_json::to_value(response).map_err(|error_details| {
            Error {
                code: ErrorCode::InternalError,
                message: error_details.to_string(),
                data: None,
            }
        })?;

        Ok(response_as_json)
    }
}
