use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub result: String,
    pub id: i32,
}
