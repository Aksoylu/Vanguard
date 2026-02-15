use hyper::StatusCode;
use jsonrpc_core::{Error, ErrorCode, Value};
use serde::{Deserialize, Serialize};

use crate::{
    core::router::Router,
    models::{boot_result::BootResult, config::Config},
};

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

    pub http_route_count: usize,
    pub https_route_count: usize,
    pub iws_route_count: usize,
    pub secure_iws_route_count: usize,
}

impl GetStatusResponse {
    pub fn build(boot_info: BootResult, router: Router) -> Result<Value, Error> {
        let response = GetStatusResponse {
            code: StatusCode::OK.as_u16(),
            config: boot_info.config.clone(),
            runtime_path: boot_info.runtime_path.to_string_lossy().to_string(),
            config_path: boot_info.config_path.to_string_lossy().to_string(),
            rpc_session_path: boot_info.rpc_session_path.to_string_lossy().to_string(),
            route_path: boot_info.route_path.to_string_lossy().to_string(),
            is_config_loaded_successfully: boot_info.is_config_loaded_successfully,
            is_router_loaded_successfully: boot_info.is_router_loaded_successfully,

            http_route_count: router.get_http_routes().keys().count(),
            https_route_count: router.get_https_routes().keys().count(),
            iws_route_count: router.get_iws_routes().keys().count(),
            secure_iws_route_count: router.get_secure_iws_routes().keys().count(),
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
