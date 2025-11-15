use crate::utils::rpc_utility::RpcParameter;
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
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Route name (source) is not valid".into(),
                data: None,
            });
        }

        if serving_path.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Serving path (serving_path) is not exist on server".into(),
                data: None,
            });
        }

        Ok(Self {
            source: source.unwrap(),
            serving_path: serving_path.unwrap(),
        })
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_serving_path(&self) -> String {
        self.serving_path.clone()
    }
}
