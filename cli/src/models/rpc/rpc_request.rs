use serde::{Deserialize, Serialize};

use crate::{core::errors::rpc_base_error::RPCBaseError, models::rpc::rpc_params::RPCParams};

/// Outermost data layer of wrapped RPC request 
#[derive(Debug, Serialize, Deserialize)]
pub struct RPCRequest {
    pub id: u64, 
    pub jsonrpc: String,
    pub method: String,
    pub params: RPCParams
}

impl Default for RPCRequest{
    fn default() -> Self {
        Self {
            id: 1,
            jsonrpc: "2.0".into(),
            method: "".into(),
            params: RPCParams::default()
        }
    }
}

impl RPCRequest{
    pub fn new(method: &str, params: RPCParams) -> Self {

         Self {
            id: 1,
            jsonrpc: "2.0".into(),
            method: method.into(),
            params
        }
    }
}