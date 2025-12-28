use crate::models::entity::engine_config::EngineConfig;
use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Serialize, Deserialize)]
pub struct GetStatusResponse {
    pub code: u16,
    pub config: EngineConfig,
    pub runtime_path: String,
    pub config_path: String,
    pub rpc_session_path: String,
    pub route_path: String,

    pub is_config_loaded_successfully: bool,
    pub is_router_loaded_successfully: bool,

    pub http_route_count: usize,
    pub https_route_count: usize,
    pub iws_route_count: usize,
    pub secure_iws_route_count: usize
} 