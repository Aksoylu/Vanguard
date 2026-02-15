use serde::{Deserialize, Serialize};

use crate::models::traffic_policy::scope_traffic_policy::ScopeTrafficPolicy;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct IwsRoute {
    pub serving_path: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<ScopeTrafficPolicy>,
}
