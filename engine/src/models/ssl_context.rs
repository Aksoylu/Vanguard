use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct SslContext {
    pub cert: String,
    pub private_key: String,
}
