use serde::{Deserialize, Serialize};

use crate::{constants::Constants, models::traffic_policy::TrafficPolicy};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HttpsServerConfig {
    pub is_active: bool,
    pub ip_address: String,
    pub port: u16,
    #[serde(default = "default_traffic_policy")]
    pub traffic_policy: TrafficPolicy,
}

impl Default for HttpsServerConfig {
    fn default() -> Self {
        Self {
            is_active: Constants::DEFUALT_HTTPS_IS_ACTIVE,
            ip_address: Constants::DEFAULT_HTTPS_IP.to_string(),
            port: Constants::DEFAULT_HTTPS_PORT,
            traffic_policy: TrafficPolicy::default(),
        }
    }
}

impl HttpsServerConfig {
    pub fn get_endpoint(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }
}

fn default_traffic_policy() -> TrafficPolicy {
    TrafficPolicy::default()
}
