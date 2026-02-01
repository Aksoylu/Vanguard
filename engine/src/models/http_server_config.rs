use serde::{Deserialize, Serialize};

use crate::{constants::Constants, models::upstream_settings::UpstreamSettings};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HttpServerConfig {
    pub is_active: bool,
    pub ip_address: String,
    pub port: u16,
    #[serde(default = "default_upstream_settings")]
    pub upstream_settings: UpstreamSettings,
}

impl Default for HttpServerConfig {
    fn default() -> Self {
        Self {
            is_active: Constants::DEFUALT_HTTP_IS_ACTIVE,
            ip_address: Constants::DEFAULT_HTTP_IP.to_string(),
            port: Constants::DEFAULT_HTTP_PORT,
            upstream_settings: UpstreamSettings::default(),
        }
    }

}

impl HttpServerConfig{
    pub fn get_endpoint(&self) -> String{
        format!("{}:{}", self.ip_address, self.port)
    }
}

fn default_upstream_settings() -> UpstreamSettings {
    UpstreamSettings::default()
}