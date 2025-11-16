use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct AddSecureIwsRequest {
    source: String,
    serving_path: String,
}

impl AddSecureIwsRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let serving_path = RpcParameter::extract_string("serving_path", &params);

        if source.is_none() {
            return Err(RPCError::build(
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Please provide Route name (source)",
            ));
        }

        if serving_path.is_none() {
            return Err(RPCError::build(
                &StatusCode::NOT_FOUND,
                "Please provide Serving path (serving_path)",
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
