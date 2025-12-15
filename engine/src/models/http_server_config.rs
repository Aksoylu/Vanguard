use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HttpServerConfig {
    pub is_active: bool,
    pub ip_address: String,
    pub port: u16,
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            is_active: Constants::DEFUALT_HTTP_IS_ACTIVE,
            ip_address: Constants::DEFAULT_HTTP_IP.to_string(),
            port: Constants::DEFAULT_HTTP_PORT,
        }
    }

}

impl HttpServerConfig{
    pub fn get_endpoint(&self) -> String{
        format!("{}:{}", self.ip_address, self.port)
    }
}