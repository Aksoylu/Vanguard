use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PathTrafficPolicy {
    #[serde(default = "default_http_client_timeout")]
    pub http_client_timeout: u64,

    #[serde(default = "default_max_request_body_size")]
    pub max_request_body_size: u64,

    #[serde(default = "default_max_requests_per_minute")]
    pub max_requests_per_minute: u32,
}

impl Default for PathTrafficPolicy {
    fn default() -> Self {
        Self {
            http_client_timeout: Constants::DEFAULT_HTTP_CLIENT_TIMEOUT,
            max_request_body_size: Constants::DEFAULT_MAX_REQUEST_BODY_SIZE,
            max_requests_per_minute: Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE,
        }
    }
}
// todo: inherit from parent (scope) traffic policy
fn default_http_client_timeout() -> u64 {
    Constants::DEFAULT_HTTP_CLIENT_TIMEOUT
}

fn default_max_request_body_size() -> u64 {
    Constants::DEFAULT_MAX_REQUEST_BODY_SIZE
}

fn default_max_requests_per_minute() -> u32 {
    Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE
}
