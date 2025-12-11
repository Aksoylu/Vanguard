use serde::{Deserialize, Serialize};

use crate::models::entity::ssl_context::SslContext;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct SecureIwsRoute {
    pub serving_path: String,
    pub ssl_context: SslContext,
}
