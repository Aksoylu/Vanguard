use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct AddIwsRouteRequest {
    source: String,
    serving_path: String,
}

impl AddIwsRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let serving_path = RpcParameter::extract_string("serving_path", &params);

        if source.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide Source (Route Name)",
            ));
        }

        if serving_path.is_none() {
            return Err(RPCError::build(
                &StatusCode::NOT_FOUND,
                "Please provide Serving Path (serving_path)",
            ));
        }

        Ok(Self {
            source: source.unwrap(),
            serving_path: serving_path.unwrap(),
        })
    }

    // getters
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_serving_path(&self) -> String {
        self.serving_path.clone()
    }
}
