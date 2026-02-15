use serde::{Deserialize, Serialize};

use crate::models::{
    ssl_context::SslContext, traffic_policy::scope_traffic_policy::ScopeTrafficPolicy,
};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct SecureIwsRoute {
    pub serving_path: String,
    pub ssl_context: SslContext,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<ScopeTrafficPolicy>,
}
