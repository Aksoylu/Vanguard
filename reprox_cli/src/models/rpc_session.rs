use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct RpcSession {
    pub ip_addr: String,
    pub port: String,
    pub hash: String,
    pub created_at: i64,
}

impl RpcSession {
    pub fn create() -> Self {
        Self {
            ip_addr: todo!(),
            port: todo!(),
            hash: todo!(),
            created_at: todo!(),
        }
    }
}
