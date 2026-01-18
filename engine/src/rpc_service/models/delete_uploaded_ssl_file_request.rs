use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

use crate::rpc_service::rpc_error::RPCError;
use crate::utils::rpc_utility::RpcParameter;

pub struct DeleteUploadedSslFileRequest {
    file_name: String,
}

impl DeleteUploadedSslFileRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let file_name = RpcParameter::extract_string("file_name", &params);

        if file_name.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'file_name' parameter",
            ));
        }

        Ok(Self {
            file_name: file_name.unwrap(),
        })
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
}
