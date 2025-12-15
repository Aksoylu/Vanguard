use std::{path::PathBuf, string};

use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use jsonrpc_core::{Error, ErrorCode, Value};

use crate::models::{boot_result::BootResult, config::Config};



#[derive(Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub code: u16,
    pub config: Config,
    pub runtime_path: String,
    pub config_path: String,
    pub rpc_session_path: String,
    pub route_path: String,

    pub is_config_loaded_successfully: bool,
    pub is_router_loaded_successfully: bool,
}

impl GetStatusResponse {
    pub fn build(status_data: BootResult) -> Result<Value, Error> {
        let response = GetStatusResponse {
            code: StatusCode::OK.as_u16(),
            config: status_data.config.clone(),
            runtime_path: status_data.runtime_path.to_string_lossy().to_string(),
            config_path: status_data.config_path.to_string_lossy().to_string(),
            rpc_session_path: status_data.rpc_session_path.to_string_lossy().to_string(),
            route_path: status_data.route_path.to_string_lossy().to_string(),
            is_config_loaded_successfully: status_data.is_config_loaded_successfully,
            is_router_loaded_successfully: status_data.is_router_loaded_successfully
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
