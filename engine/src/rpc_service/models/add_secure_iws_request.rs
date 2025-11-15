use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;

pub struct AddSecureIwsRequest {
    source: String,
    serving_path: String,
    use_ssl: bool,
}

impl AddSecureIwsRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let serving_path = RpcParameter::extract_string("serving_path", &params);

        let use_ssl = RpcParameter::extract_boolean("use_ssl", params.clone());

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
            use_ssl,
        })
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_serving_path(&self) -> String {
        self.serving_path.clone()
    }

    pub fn get_use_ssl(&self) -> bool {
        self.use_ssl.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddSecureIwsResponse {
    code: i32,
    message: String,
}

impl AddSecureIwsResponse {
    pub fn build(message: String) -> jsonrpc_core::Value {
        let response = AddSecureIwsResponse { code: 200, message };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
