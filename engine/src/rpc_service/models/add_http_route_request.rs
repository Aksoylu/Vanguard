use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;

pub struct AddHttpRouteRequest {
    source: String,
    target: String,
}

impl AddHttpRouteRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let source: Option<String> = RpcParameter::extract_string("source", params.clone());
        let target = RpcParameter::extract_string("target", params.clone());

        if source.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Route name is not valid".into(),
                data: None,
            });
        }

        if target.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Route name is not valid".into(),
                data: None,
            });
        }

        Ok(Self {
            source: source.unwrap(),
            target: target.unwrap(),
        })
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }
}
