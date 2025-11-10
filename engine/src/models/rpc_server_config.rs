use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RpcServerConfig {
    pub ip_address: String,
    pub port: u16,
    pub private_secret_key: String,
}

impl Default for RpcServerConfig {
    fn default() -> Self {
        Self {
            ip_address: Constants::DEFAULT_RPC_IP.to_string(),
            port: Constants::DEFAULT_RPC_PORT,
            private_secret_key: Constants::DEFAULT_PRIVATE_SECRET_KEY.to_string(),
        }
    }
}

impl RpcServerConfig {
    pub fn get_endpoint(&self) -> String{
        format!("{}:{}", self.ip_address, self.port)
    }
}