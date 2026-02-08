use serde::{Deserialize, Serialize};

use crate::{
    core::shared_memory::RUNTIME_BOOT_INFO,
    models::{ssl_context::SslContext, traffic_policy::scope_traffic_policy::ScopeTrafficPolicy},
};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpsRoute {
    pub target: String,
    pub ssl_context: SslContext,

    #[serde(default = "inherit_from_global")]
    #[serde(skip_serializing_if = "is_inherited_from_global")]
    pub traffic_policy: ScopeTrafficPolicy,
}

fn inherit_from_global() -> ScopeTrafficPolicy {
    let runtime_boot_info = RUNTIME_BOOT_INFO.read().unwrap();
    let inherited_traffic_policy = runtime_boot_info.config.https_server.traffic_policy.clone();

    inherited_traffic_policy
}

fn is_inherited_from_global(val: &ScopeTrafficPolicy) -> bool {
    *val == inherit_from_global()
}
