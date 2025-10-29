use hyper::StatusCode;
use jsonrpc_core::{Error, Params, Value};

pub struct RPCError;

impl RPCError {
    pub fn badrequest(message: String) -> Error {
        build(message, &StatusCode::BAD_REQUEST)
    }

    pub fn unauthorized(message: String) -> Error {
        build(message, &StatusCode::UNAUTHORIZED)
    }

    fn build(message: String, code: &StatusCode) -> Error {
        Error {
            code: jsonrpc_core::ErrorCode::ServerError(code.as_u16() as i64),
            message: message,
            data: None,
        }
    }
}
