use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GlobalSettings {
    // Only global scalability & performance settings
    #[serde(default = "default_http1_header_read_timeout")]
    pub http1_header_read_timeout: u64,

    #[serde(default = "default_maximum_total_connections")]
    pub maximum_total_connections: u64,

    // All Scalability & Performance Settings
    #[serde(default = "default_server_read_timeout")]
    pub server_read_timeout: u64,

    #[serde(default = "default_http_client_timeout")]
    pub http_client_timeout: u64,

    #[serde(default = "default_pool_idle_timeout")]
    pub pool_idle_timeout: u64,

    #[serde(default = "default_max_idle_conns_per_host")]
    pub max_idle_conns_per_host: usize,

    #[serde(default = "default_server_write_timeout")]
    pub server_write_timeout: u64,

    #[serde(default = "default_max_request_body_size")]
    pub max_request_body_size: u64,

    #[serde(default = "default_max_requests_per_minute")]
    pub max_requests_per_minute: u32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            http1_header_read_timeout: Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT,
            maximum_total_connections: Constants::DEFAULT_MAXIMUM_TOTAL_CONNECTIONS,
            server_read_timeout: Constants::DEFAULT_SERVER_READ_TIMEOUT,

            http_client_timeout: Constants::DEFAULT_HTTP_CLIENT_TIMEOUT,
            pool_idle_timeout: Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            max_idle_conns_per_host: Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST,
            server_write_timeout: Constants::DEFAULT_SERVER_WRITE_TIMEOUT,
            max_request_body_size: Constants::DEFAULT_MAX_REQUEST_BODY_SIZE,
            max_requests_per_minute: Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE,
        }
    }
}

fn default_max_requests_per_minute() -> u32 {
    Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE
}

fn default_max_request_body_size() -> u64 {
    Constants::DEFAULT_MAX_REQUEST_BODY_SIZE
}

fn default_server_write_timeout() -> u64 {
    Constants::DEFAULT_SERVER_WRITE_TIMEOUT
}

fn default_max_idle_conns_per_host() -> usize {
    Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST
}

fn default_pool_idle_timeout() -> u64 {
    Constants::DEFAULT_POOL_IDLE_TIMEOUT
}

fn default_http_client_timeout() -> u64 {
    Constants::DEFAULT_HTTP_CLIENT_TIMEOUT
}

fn default_server_read_timeout() -> u64 {
    Constants::DEFAULT_SERVER_READ_TIMEOUT
}

fn default_maximum_total_connections() -> u64 {
    Constants::DEFAULT_MAXIMUM_TOTAL_CONNECTIONS
}

fn default_http1_header_read_timeout() -> u64 {
    Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT
}
