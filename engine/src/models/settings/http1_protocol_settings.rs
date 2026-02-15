use crate::constants::Constants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Http1ProtocolSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http1_header_read_timeout: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_nodelay: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http1_keepalive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_keepalive: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http1_max_buf_size: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub http1_only: Option<bool>,
}

impl Default for Http1ProtocolSettings {
    fn default() -> Self {
        Self {
            http1_header_read_timeout: None,
            tcp_nodelay: None,
            http1_keepalive: None,
            tcp_keepalive: None,
            http1_max_buf_size: None,
            http1_only: None,
        }
    }
}

impl Http1ProtocolSettings {
    pub fn global() -> Self {
        Self {
            http1_header_read_timeout: Some(Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT),
            tcp_nodelay: Some(true),
            http1_keepalive: Some(true),
            tcp_keepalive: Some(Constants::DEFAULT_POOL_IDLE_TIMEOUT),
            http1_max_buf_size: Some(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE as usize),
            http1_only: Some(true),
        }
    }

    pub fn merge(&mut self, other: &Self) {
        if other.http1_header_read_timeout.is_some() {
            self.http1_header_read_timeout = other.http1_header_read_timeout;
        }
        if other.tcp_nodelay.is_some() {
            self.tcp_nodelay = other.tcp_nodelay;
        }
        if other.http1_keepalive.is_some() {
            self.http1_keepalive = other.http1_keepalive;
        }
        if other.tcp_keepalive.is_some() {
            self.tcp_keepalive = other.tcp_keepalive;
        }
        if other.http1_max_buf_size.is_some() {
            self.http1_max_buf_size = other.http1_max_buf_size;
        }
        if other.http1_only.is_some() {
            self.http1_only = other.http1_only;
        }
    }

    // Getters
    pub fn get_http1_header_read_timeout(&self) -> u64 {
        self.http1_header_read_timeout
            .unwrap_or(Constants::DEFAULT_HTTP1_HEADER_READ_TIMEOUT)
    }

    pub fn get_tcp_nodelay(&self) -> bool {
        self.tcp_nodelay.unwrap_or(true)
    }

    pub fn get_http1_keepalive(&self) -> bool {
        self.http1_keepalive.unwrap_or(true)
    }

    pub fn get_tcp_keepalive(&self) -> u64 {
        self.tcp_keepalive
            .unwrap_or(Constants::DEFAULT_POOL_IDLE_TIMEOUT)
    }

    pub fn get_http1_max_buf_size(&self) -> usize {
        self.http1_max_buf_size
            .unwrap_or(Constants::DEFAULT_MAX_REQUEST_BODY_SIZE as usize)
    }

    pub fn get_http1_only(&self) -> bool {
        self.http1_only.unwrap_or(true)
    }
}
