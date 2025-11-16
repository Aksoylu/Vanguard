use hyper::StatusCode;
use jsonrpc_core::{Error, Params, Value};

pub struct RPCError;

impl RPCError {
    pub fn badrequest(message: &str) -> Error {
        Self::build(&StatusCode::BAD_REQUEST, message)
    }

    pub fn unauthorized(message: &str) -> Error {
        Self::build(&StatusCode::UNAUTHORIZED, message)
    }

    pub fn build(code: &StatusCode, message: &str) -> Error {
        Error {
            code: jsonrpc_core::ErrorCode::ServerError(code.as_u16() as i64),
            message: message.into(),
            data: None,
        }
    }
}
