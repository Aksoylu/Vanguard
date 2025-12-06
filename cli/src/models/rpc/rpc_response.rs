use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::errors::rpc_base_error::RPCBaseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub result: Value,
    pub id: i64,
}

impl RPCResponse {
    pub fn build(input: String) -> Result<RPCResponse, RPCBaseError> {
        let parsed_input: Value = serde_json::from_str(&input)
            .map_err(|_| RPCBaseError::build("RPC response can not parsed"))?;

        let jsonrpc_version = &parsed_input["jsonrpc"];
        let jsonrpc = jsonrpc_version.as_str().unwrap_or_default().to_string();

        let result = &parsed_input["result"];

        let json_id = &parsed_input["id"];
        let id = json_id.as_i64().unwrap_or_default();

        Ok(RPCResponse {
            jsonrpc,
            result: result.to_owned(),
            id,
        })
    }
}
