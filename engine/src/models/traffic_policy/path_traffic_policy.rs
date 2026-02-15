use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PathTrafficPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_client_timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_request_body_size: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_requests_per_minute: Option<u32>,
}

impl PathTrafficPolicy {
    /// Creates a new instance with all values set to their defaults.
    pub fn global() -> Self {
        Self {
            http_client_timeout: Some(Constants::DEFAULT_HTTP_CLIENT_TIMEOUT),
            max_request_body_size: Some(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE),
            max_requests_per_minute: Some(Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE),
        }
    }

    /// Merges another policy into this one. Values in `other` take precedence.
    pub fn merge(&mut self, other: &Self) {
        if other.http_client_timeout.is_some() {
            self.http_client_timeout = other.http_client_timeout;
        }
        if other.max_request_body_size.is_some() {
            self.max_request_body_size = other.max_request_body_size;
        }
        if other.max_requests_per_minute.is_some() {
            self.max_requests_per_minute = other.max_requests_per_minute;
        }
    }

    pub fn get_http_client_timeout(&self) -> u64 {
        self.http_client_timeout
            .unwrap_or(Constants::DEFAULT_HTTP_CLIENT_TIMEOUT)
    }

    pub fn get_max_request_body_size(&self) -> u64 {
        self.max_request_body_size
            .unwrap_or(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE)
    }

    pub fn get_max_requests_per_minute(&self) -> u32 {
        self.max_requests_per_minute
            .unwrap_or(Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE)
    }
}
