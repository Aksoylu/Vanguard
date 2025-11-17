use hyper::StatusCode;
use jsonrpc_core::Error;

pub struct RPCError;

impl RPCError {
    pub fn badrequest(message: String) -> Error {
        Self::build(&StatusCode::BAD_REQUEST, message.as_str())
    }

    pub fn unauthorized(message: String) -> Error {
        Self::build(&StatusCode::UNAUTHORIZED, message.as_str())
    }

    pub fn build(code: &StatusCode, message: &str) -> Error {
        Error {
            code: jsonrpc_core::ErrorCode::ServerError(code.as_u16() as i64),
            message: message.into(),
            data: None,
        }
    }
}
