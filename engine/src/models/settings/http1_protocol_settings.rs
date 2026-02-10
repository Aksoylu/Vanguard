use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Http1ProtocolSettings {
    #[serde(default = "default_http1_header_read_timeout")]
    #[serde(skip_serializing_if = "is_http1_header_read_timeout_default")]
    pub http1_header_read_timeout: u64,

    #[serde(default = "default_tcp_nodelay")]
    pub tcp_nodelay: bool,

    #[serde(default = "default_http1_keepalive")]
    pub http1_keepalive: bool,

    #[serde(default = "default_tcp_keepalive")]
    pub tcp_keepalive: u64,

    #[serde(default = "default_http1_max_buf_size")]
    pub http1_max_buf_size: usize,

    #[serde(default = "default_http1_only")]
    pub http1_only: bool,
}

impl Default for Http1ProtocolSettings {
    fn default() -> Self {
        Self {
            http1_header_read_timeout: Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT,
            tcp_nodelay: true,
            http1_keepalive: true,
            tcp_keepalive: Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            http1_max_buf_size: Constants::DEFAULT_MAX_REQUEST_BODY_SIZE as usize,
            http1_only: true,
        }
    }
}

fn default_http1_header_read_timeout() -> u64 {
    Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT
}

fn default_tcp_nodelay() -> bool {
    true
}

fn default_http1_keepalive() -> bool {
    true
}

fn default_tcp_keepalive() -> u64 {
    Constants::DEFAULT_POOL_IDLE_TIMEOUT
}

fn default_http1_max_buf_size() -> usize {
    Constants::DEFAULT_MAX_REQUEST_BODY_SIZE as usize
}

fn default_http1_only() -> bool {
    true
}

fn is_http1_header_read_timeout_default(val: &u64) -> bool {
    *val == default_http1_header_read_timeout()
}
