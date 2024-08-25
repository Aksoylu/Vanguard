use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::{Deserialize, Serialize};

pub struct DeleteSSlCertRequest {
    domain: String,
}

impl DeleteSSlCertRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let domain: Option<String> = RpcParameter::extract_string("domain", params.clone());

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

#[derive(Serialize, Deserialize)]
pub struct DeleteSSlCertResponse {
    code: i32,
    message: String,
    data: Vec<String>,
}

impl DeleteSSlCertResponse {
    pub fn build(data: Vec<String>) -> jsonrpc_core::Value {
        let response = DeleteSSlCertResponse {
            code: 200,
            message: "OK".into(),
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
