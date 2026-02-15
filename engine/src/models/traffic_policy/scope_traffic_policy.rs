use serde::{Deserialize, Serialize};

use crate::models::{
    settings::{
        http1_protocol_settings::Http1ProtocolSettings, http2_protocol_settings::Http2ProtocolSettings,
        upstream_settings::UpstreamSettings,
    },
    traffic_policy::path_traffic_policy::PathTrafficPolicy,
};

/// Represents the traffic policy for a specific protocol (e.g., Http, Https).
///
/// This struct holds the protocol-specific settings for HTTP/1 and HTTP/2, as well as
/// the upstream connection settings. It allows for fine-grained control over how traffic
/// is handled at the scope level.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ScopeTrafficPolicy {
    #[serde(default)]
    pub http1_protocol_settings: Http1ProtocolSettings,

    #[serde(default)]
    pub http2_protocol_settings: Http2ProtocolSettings,

    #[serde(default)]
    pub upstream_settings: UpstreamSettings,
}

impl ScopeTrafficPolicy {
    /// Creates a new instance with all values set to their defaults.
    pub fn global() -> Self {
        Self {
            http1_protocol_settings: Http1ProtocolSettings::global(),
            http2_protocol_settings: Http2ProtocolSettings::global(),
            upstream_settings: UpstreamSettings::global(),
        }
    }

    /// Merges another policy into this one. Values in `other` take precedence.
    pub fn merge(&mut self, other: &Self) {
        self.http1_protocol_settings.merge(&other.http1_protocol_settings);
        self.http2_protocol_settings.merge(&other.http2_protocol_settings);
        self.upstream_settings.merge(&other.upstream_settings);
    }

    /// Merges a path policy into this scope policy.
    pub fn merge_path_policy(&mut self, path_policy: &PathTrafficPolicy) {
        if path_policy.http_client_timeout.is_some() {
            self.upstream_settings.http_client_timeout = path_policy.http_client_timeout;
        }
        if path_policy.max_request_body_size.is_some() {
            self.upstream_settings.max_request_body_size = path_policy.max_request_body_size;
        }
        // max_requests_per_minute is handled at the connection manager level
    }
}
