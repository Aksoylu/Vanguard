use crate::{rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct DeleteSSlCertRequest {
    domain: String,
}

impl DeleteSSlCertRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let domain: Option<String> = RpcParameter::extract_string("domain", &params);

        if domain.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'domain' parameter",
            ));
        }

        Ok(Self {
            domain: domain.unwrap(),
        })
    }

    // getters
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }
}
