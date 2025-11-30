use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT};

/// Inner data layer of wrapped RPC request 
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RPCPayload {
    data: Value,
    token: String
}

impl RPCPayload{
    pub fn new(data: Value) -> Result<Self, RPCBaseError> {

        let current_rpc_client = RPC_CLIENT.read().unwrap();

        let token= current_rpc_client
            .boot_data.as_ref()
            .and_then(|boot_data| boot_data.rpc_session.as_ref()) 
            .map(|rpc_session_info| rpc_session_info.authorization_token.clone()); 

        if token.is_none(){
            return Err(RPCBaseError::build("Authorization token can not found in session data"));
        }

        return Ok(RPCPayload { data, token: token.unwrap() })
    }
}