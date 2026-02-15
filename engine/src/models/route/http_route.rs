use serde::{Deserialize, Serialize};

use crate::models::traffic_policy::{
    scope_traffic_policy::ScopeTrafficPolicy, path_traffic_policy::PathTrafficPolicy,
};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpRoute {
    pub target: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<ScopeTrafficPolicy>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_policy: Option<PathTrafficPolicy>,
}
