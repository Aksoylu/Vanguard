use jsonrpc_core::{Error, Value};
use serde::Deserialize;
use serde::Serialize;

use crate::utils::rpc_utility::RpcParameter;

#[derive(Serialize, Deserialize)]
pub struct EchoRequest {
    message: String,
}

impl EchoRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let message = RpcParameter::extract_string("message", &params);

        if message.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Please add a message".into(),
                data: None,
            });
        }

        Ok(Self {
            message: message.unwrap(),
        })
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
