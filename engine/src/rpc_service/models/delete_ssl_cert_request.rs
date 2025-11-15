use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Value};

pub struct DeleteSSlCertRequest {
    domain: String,
}

impl DeleteSSlCertRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let domain: Option<String> = RpcParameter::extract_string("domain", &params);

        if domain.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Please speficy an existing domain".into(),
                data: None,
            });
        }

        Ok(Self {
            domain: domain.unwrap(),
        })
    }

    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }
}
