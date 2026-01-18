use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct AddSecureIwsRouteRequest {
    source: String,
    serving_path: String,
    ssl_cert_path: String,
    ssl_private_key_path: String,
}

impl AddSecureIwsRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let serving_path = RpcParameter::extract_string("serving_path", &params);
        let ssl_cert_path = RpcParameter::extract_string("ssl_cert_path", &params);
        let ssl_private_key_path = RpcParameter::extract_string("ssl_private_key_path", &params);

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

        if ssl_cert_path.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'ssl_cert_path' parameter",
            ));
        }

        if ssl_private_key_path.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'ssl_private_key_path' parameter",
            ));
        }

        Ok(Self {
            source: source.unwrap(),
            serving_path: serving_path.unwrap(),
            ssl_cert_path: ssl_cert_path.unwrap(),
            ssl_private_key_path: ssl_private_key_path.unwrap(),
        })
    }

    // getters
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_serving_path(&self) -> String {
        self.serving_path.clone()
    }

    pub fn get_ssl_cert_path(&self) -> String {
        self.ssl_cert_path.clone()
    }

    pub fn get_ssl_private_key_path(&self) -> String {
        self.ssl_private_key_path.clone()
    }
}
