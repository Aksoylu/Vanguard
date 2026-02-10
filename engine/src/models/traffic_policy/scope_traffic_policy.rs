use serde::{Deserialize, Serialize};

use crate::models::settings::http1_protocol_settings::Http1ProtocolSettings;
use crate::models::settings::http2_protocol_settings::Http2ProtocolSettings;
use crate::models::settings::upstream_settings::UpstreamSettings;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ScopeTrafficPolicy {
    pub http1_protocol_settings: Http1ProtocolSettings,

    pub http2_protocol_settings: Http2ProtocolSettings,

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
