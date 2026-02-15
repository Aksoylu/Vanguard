use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UpstreamSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_client_timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_idle_timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idle_conns_per_host: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_request_body_size: Option<u64>,
}

impl Default for UpstreamSettings {
    fn default() -> Self {
        Self {
            http_client_timeout: None,
            pool_idle_timeout: None,
            max_idle_conns_per_host: None,
            max_request_body_size: None,
        }
    }
}

impl UpstreamSettings {
    pub fn global() -> Self {
        Self {
            http_client_timeout: Some(Constants::DEFAULT_HTTP_CLIENT_TIMEOUT),
            pool_idle_timeout: Some(Constants::DEFAULT_POOL_IDLE_TIMEOUT),
            max_idle_conns_per_host: Some(Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST),
            max_request_body_size: Some(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if other.http_client_timeout.is_some() {
            self.http_client_timeout = other.http_client_timeout;
        }
        if other.pool_idle_timeout.is_some() {
            self.pool_idle_timeout = other.pool_idle_timeout;
        }
        if other.max_idle_conns_per_host.is_some() {
            self.max_idle_conns_per_host = other.max_idle_conns_per_host;
        }
        if other.max_request_body_size.is_some() {
            self.max_request_body_size = other.max_request_body_size;
        }
    }

    // Getters
    pub fn get_http_client_timeout(&self) -> u64 {
        self.http_client_timeout
            .unwrap_or(Constants::DEFAULT_HTTP_CLIENT_TIMEOUT)
    }

    pub fn get_pool_idle_timeout(&self) -> u64 {
        self.pool_idle_timeout
            .unwrap_or(Constants::DEFAULT_POOL_IDLE_TIMEOUT)
    }

    pub fn get_max_idle_conns_per_host(&self) -> usize {
        self.max_idle_conns_per_host
            .unwrap_or(Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST)
    }

    pub fn get_max_request_body_size(&self) -> u64 {
        self.max_request_body_size
            .unwrap_or(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE)
    }
}
