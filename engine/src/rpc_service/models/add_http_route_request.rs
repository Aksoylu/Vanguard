use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct AddHttpRouteRequest {
    source: String,
    target: String,
}

impl AddHttpRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let target = RpcParameter::extract_string("target", &params);

        if source.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'Source' parameter",
            ));
        }

        if target.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'Target' parameter",
            ));
        }

        Ok(Self {
            source: source.unwrap(),
            target: target.unwrap(),
        })
    }

    // getters
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }
}
