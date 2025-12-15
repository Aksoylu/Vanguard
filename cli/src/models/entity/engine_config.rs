use serde::{Deserialize, Serialize};

use crate::models::entity::{
    engine_http_server_config::EngineHttpServerConfig, engine_logger_config::EngineLoggerConfig,
    engine_rpc_server_config::EngineRpcServerConfig,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct EngineConfig {
    pub http_server: EngineHttpServerConfig,
    pub https_server: EngineHttpServerConfig,
    pub rpc_server: EngineRpcServerConfig,
    pub logger: EngineLoggerConfig,
}


impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            http_server: Default::default(),
            https_server: Default::default(),
            rpc_server: Default::default(),
            logger: Default::default()
        }
    }
}
