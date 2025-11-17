use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct DeleteHttpRouteRequest {
    source: String,
}

impl DeleteHttpRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);

        if source.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'source' parameter",
            ));
        }

        Ok(Self {
            source: source.unwrap(),
        })
    }

    // getters
    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}
