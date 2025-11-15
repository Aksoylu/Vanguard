use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Value};

pub struct DeleteHttpRouteRequest {
    source: String,
}

impl DeleteHttpRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);

        if source.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Provide at least 'source' parameter".into(),
                data: None,
            });
        }

        Ok(Self {
            source: source.unwrap(),
        })
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}
