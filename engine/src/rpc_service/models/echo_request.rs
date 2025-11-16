use hyper::StatusCode;
use jsonrpc_core::{Error, Value};
use serde::Deserialize;
use serde::Serialize;

use crate::rpc_service::rpc_error::RPCError;
use crate::utils::rpc_utility::RpcParameter;

#[derive(Serialize, Deserialize)]
pub struct EchoRequest {
    message: String,
}

impl EchoRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let message = RpcParameter::extract_string("message", &params);

        if message.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'message' parameter",
            ));
        }

        Ok(Self {
            message: message.unwrap(),
        })
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
