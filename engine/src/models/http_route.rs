use serde::{Deserialize, Serialize};

use crate::{core::shared_memory::RUNTIME_BOOT_INFO, models::upstream_settings::UpstreamSettings};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpRoute {
    pub target: String,
    #[serde(default = "use_global_upstream_settings")]
    pub upstream_settings: UpstreamSettings,
}

fn use_global_upstream_settings() -> UpstreamSettings {
    let runtime_boot_info = RUNTIME_BOOT_INFO.read().unwrap();
    let upstream_settins = runtime_boot_info
        .config
        .http_server
        .upstream_settings
        .clone();

    upstream_settins
}
