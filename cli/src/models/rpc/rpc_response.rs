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
        let serialized_input: Value = serde_json::to_value(input)
            .map_err(|_| RPCBaseError::build("Object can not serialized"))?;

        println!("rpcresponse serialized_input >> {:?}", &serialized_input);

        let jsonrpc_version = &serialized_input["jsonrpc"];
        let jsonrpc = jsonrpc_version.as_str().unwrap_or_default().to_string();

        let result = &serialized_input["result"];

        let json_id = &serialized_input["id"];
        let id = json_id.as_i64().unwrap_or_default();

        Ok(RPCResponse {
            jsonrpc,
            result: result.to_owned(),
            id,
        })
    }
}
