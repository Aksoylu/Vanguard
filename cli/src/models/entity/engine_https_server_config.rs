use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngineHttpsServerConfig {
    pub is_active: bool,
    pub ip_address: String,
    pub port: u16,
}

impl Default for EngineHttpsServerConfig {
    fn default() -> Self {
        Self {
            is_active: Constants::DEFUALT_HTTPS_IS_ACTIVE,
            ip_address: Constants::DEFAULT_HTTPS_IP.to_string(),
            port: Constants::DEFAULT_HTTPS_PORT,
        }
    }
}

impl EngineHttpsServerConfig {
    pub fn get_endpoint(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }
}
