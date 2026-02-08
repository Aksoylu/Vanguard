use serde::{Deserialize, Serialize};

use crate::models::settings::http1_protocol_settings::Http1ProtocolSettings;
use crate::models::settings::http2_protocol_settings::Http2ProtocolSettings;
use crate::models::settings::upstream_settings::UpstreamSettings;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ScopeTrafficPolicy {
    #[serde(default = "default_http1_protocol_settings")]
    pub http1_protocol_settings: Http1ProtocolSettings,

    #[serde(default = "default_http2_protocol_settings")]
    pub http2_protocol_settings: Http2ProtocolSettings,

    #[serde(default = "default_upstream_settings")]
    pub upstream_settings: UpstreamSettings,
}

impl Default for ScopeTrafficPolicy {
    fn default() -> Self {
        Self {
            http1_protocol_settings: Http1ProtocolSettings::default(),
            http2_protocol_settings: Http2ProtocolSettings::default(),
            upstream_settings: UpstreamSettings::default(),
        }
    }
}

// todo: inherit from parent (global) traffic policy
fn default_http1_protocol_settings() -> Http1ProtocolSettings {
    Http1ProtocolSettings::default()
}

fn default_http2_protocol_settings() -> Http2ProtocolSettings {
    Http2ProtocolSettings::default()
}

fn default_upstream_settings() -> UpstreamSettings {
    UpstreamSettings::default()
}
