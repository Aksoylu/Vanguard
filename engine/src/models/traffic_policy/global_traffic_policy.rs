use serde::{Deserialize, Serialize};

use crate::models::settings::{
    http1_protocol_settings::Http1ProtocolSettings, http2_protocol_settings::Http2ProtocolSettings,
    server_settings::ServerSettings, upstream_settings::UpstreamSettings,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct GlobalTrafficPolicy {
    #[serde(default)]
    pub server: ServerSettings,

    #[serde(default)]
    pub http1_protocol_settings: Http1ProtocolSettings,

    #[serde(default)]
    pub http2_protocol_settings: Http2ProtocolSettings,

    #[serde(default)]
    pub upstream_settings: UpstreamSettings,
}

impl GlobalTrafficPolicy {
    /// Creates a new instance with all values set to their defaults.
    pub fn global() -> Self {
        Self {
            server: ServerSettings::default(),
            http1_protocol_settings: Http1ProtocolSettings::global(),
            http2_protocol_settings: Http2ProtocolSettings::global(),
            upstream_settings: UpstreamSettings::global(),
        }
    }
}
