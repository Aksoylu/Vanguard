use serde::{Deserialize, Deserializer, Serialize};

use crate::constants::Constants;
use crate::utils::text_utility::parse_str_as_size;

fn default_max_request_body_size() -> u64 {
    Constants::DEFAULT_MAX_REQUEST_BODY_SIZE
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrafficPolicy {
    #[serde(default = "default_http_client_timeout")]
    pub http_client_timeout: u64,

    #[serde(default = "default_pool_idle_timeout")]
    pub pool_idle_timeout: u64,

    #[serde(default = "default_max_idle_conns_per_host")]
    pub max_idle_conns_per_host: usize,

    #[serde(default = "default_server_read_timeout")]
    pub server_read_timeout: u64,

    #[serde(default = "default_server_write_timeout")]
    pub server_write_timeout: u64,

    #[serde(default = "default_max_requests_per_minute")]
    pub max_requests_per_minute: u32,

    #[serde(default = "default_max_request_body_size")]
    #[serde(deserialize_with = "deserialize_size")]
    pub max_request_body_size: u64,
}

impl Default for TrafficPolicy {
    fn default() -> Self {
        Self {
            http_client_timeout: Constants::DEFAULT_HTTP_CLIENT_TIMEOUT,
            pool_idle_timeout: Constants::DEFAULT_POOL_IDLE_TIMEOUT,
            max_idle_conns_per_host: Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST,
            server_read_timeout: Constants::DEFAULT_SERVER_READ_TIMEOUT,
            server_write_timeout: Constants::DEFAULT_SERVER_WRITE_TIMEOUT,
            max_request_body_size: Constants::DEFAULT_MAX_REQUEST_BODY_SIZE,
            max_requests_per_minute: Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE,
        }
    }
}

fn default_http_client_timeout() -> u64 {
    Constants::DEFAULT_HTTP_CLIENT_TIMEOUT
}

fn default_pool_idle_timeout() -> u64 {
    Constants::DEFAULT_POOL_IDLE_TIMEOUT
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

fn default_max_idle_conns_per_host() -> usize {
    Constants::DEFAULT_MAX_IDLE_CONNS_PER_HOST
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Number(n) => n
            .as_u64()
            .ok_or_else(|| Error::custom("Invalid number for size")),
        Value::String(s) => parse_str_as_size(&s).map_err(Error::custom),
        _ => Err(Error::custom(
            "Expected a number or string for max_request_body_size",
        )),
    }
}
