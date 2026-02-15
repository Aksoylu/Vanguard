use serde::{Deserialize, Serialize};

use crate::constants::Constants;


#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UpstreamSettings {
    #[serde(default = "default_http_client_timeout")]
    #[serde(skip_serializing_if = "is_http_client_timeout_default")]
    pub http_client_timeout: u64,

    #[serde(default = "default_pool_idle_timeout")]
    #[serde(skip_serializing_if = "is_pool_idle_timeout_default")]
    pub pool_idle_timeout: u64,

    #[serde(default = "default_max_idle_conns_per_host")]
    #[serde(skip_serializing_if = "is_max_idle_conns_per_host_default")]
    pub max_idle_conns_per_host: usize,

    #[serde(default = "default_max_request_body_size")]
    #[serde(skip_serializing_if = "is_max_request_body_size_default")]
    pub max_request_body_size: u64,
}

impl Default for UpstreamSettings {
    fn default() -> Self {
        Self {
            http_client_timeout: Constants::DEFAULT_HTTP_CLIENT_TIMEOUT,
            pool_idle_timeout: Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            max_idle_conns_per_host: Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST,
            max_request_body_size: Constants::DEFAULT_MAX_REQUEST_BODY_SIZE,
        }
    }
}

// Default functions for serde
fn default_http_client_timeout() -> u64 {
    Constants::DEFAULT_HTTP_CLIENT_TIMEOUT
}
fn default_pool_idle_timeout() -> u64 {
    Constants::DEFAULT_POOL_IDLE_TIMEOUT
}
fn default_max_idle_conns_per_host() -> usize {
    Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST
}
fn default_max_request_body_size() -> u64 {
    Constants::DEFAULT_MAX_REQUEST_BODY_SIZE
}

fn is_http_client_timeout_default(val: &u64) -> bool {
    *val == default_http_client_timeout()
}

fn is_pool_idle_timeout_default(val: &u64) -> bool {
    *val == default_pool_idle_timeout()
}

fn is_max_idle_conns_per_host_default(val: &usize) -> bool {
    *val == default_max_idle_conns_per_host()
}

fn is_max_request_body_size_default(val: &u64) -> bool {
    *val == default_max_request_body_size()
}