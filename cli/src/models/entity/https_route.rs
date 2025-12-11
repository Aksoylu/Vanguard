use serde::{Deserialize, Serialize};

use crate::models::entity::ssl_context::SslContext;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpsRoute {
    pub target: String,
    pub ssl_context: SslContext,
}