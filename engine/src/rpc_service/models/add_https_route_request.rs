use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct AddHttpsRouteRequest {
    source: String,
    target: String,
    ssl_cert_path: String,
    ssl_private_key_path: String,
}

impl AddHttpsRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let target = RpcParameter::extract_string("target", &params);
        let ssl_cert_path = RpcParameter::extract_string("ssl_cert_path", &params);
        let ssl_private_key_path = RpcParameter::extract_string("ssl_private_key_path", &params);

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
            target: target.unwrap(),
            ssl_cert_path: ssl_cert_path.unwrap(),
            ssl_private_key_path: ssl_private_key_path.unwrap(),
        })
    }

    // getters
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }

    pub fn get_ssl_cert_path(&self) -> String {
        self.ssl_cert_path.clone()
    }

    pub fn get_ssl_private_key_path(&self) -> String {
        self.ssl_private_key_path.clone()
    }
}
