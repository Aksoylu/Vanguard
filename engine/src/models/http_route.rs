use serde::{Deserialize, Serialize};

use crate::{core::shared_memory::RUNTIME_BOOT_INFO, models::traffic_policy::TrafficPolicy};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpRoute {
    pub target: String,
    #[serde(default = "inherit_traffic_policy_from_parent")]
    pub traffic_policy: TrafficPolicy,
}

fn inherit_traffic_policy_from_parent() -> TrafficPolicy {
    let runtime_boot_info = RUNTIME_BOOT_INFO.read().unwrap();
    let inherited_traffic_policy = runtime_boot_info
        .config
        .http_server
        .traffic_policy
        .clone();

    inherited_traffic_policy
}
