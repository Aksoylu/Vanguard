use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PathTrafficPolicy {
    pub http_client_timeout: u64,

    pub max_request_body_size: u64,

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
