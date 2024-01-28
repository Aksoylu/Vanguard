use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RpcSession
{
    pub ip_addr: String,
    pub port: String,
    pub hash: String,
    pub created_at: i64
}
