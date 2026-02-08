use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServerSettings {
    #[serde(default = "default_maximum_total_connections")]
    pub maximum_total_connections: u64,

    #[serde(default = "default_server_read_timeout")]
    pub server_read_timeout: u64,

    #[serde(default = "default_server_write_timeout")]
    pub server_write_timeout: u64,

    #[serde(default = "default_max_requests_per_minute")]
    pub max_requests_per_minute: u32,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            maximum_total_connections: Constants::DEFAULT_MAXIMUM_TOTAL_CONNECTIONS,
            server_read_timeout: Constants::DEFAULT_SERVER_READ_TIMEOUT,
            server_write_timeout: Constants::DEFAULT_SERVER_WRITE_TIMEOUT,
            max_requests_per_minute: Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE,
        }
    }
}

fn default_maximum_total_connections() -> u64 {
    Constants::DEFAULT_MAXIMUM_TOTAL_CONNECTIONS
}

fn default_server_read_timeout() -> u64 {
    Constants::DEFAULT_SERVER_READ_TIMEOUT
}

fn default_server_write_timeout() -> u64 {
    Constants::DEFAULT_SERVER_WRITE_TIMEOUT
}

fn default_max_requests_per_minute() -> u32 {
    Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE
}
