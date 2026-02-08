use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Http1ProtocolSettings {
    #[serde(default = "default_http1_header_read_timeout")]
    #[serde(skip_serializing_if = "is_http1_header_read_timeout_default")]
    pub http1_header_read_timeout: u64,
}

impl Default for Http1ProtocolSettings {
    fn default() -> Self {
        Self {
            http1_header_read_timeout: Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT,
        }
    }
}

fn default_http1_header_read_timeout() -> u64 {
    Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT
}

fn is_http1_header_read_timeout_default(val: &u64) -> bool {
    *val == default_http1_header_read_timeout()
}
