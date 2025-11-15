use jsonrpc_core::{Error, ErrorCode, Value};
use crate::rpc_service::models::echo_request::EchoRequest;
use crate::rpc_service::models::echo_response::EchoResponse;

pub fn echo(payload: Value) -> Result<Value, Error> {
    let request = match EchoRequest::new(payload) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: echo".into(),
                data: None,
            });
        }
    };

    Ok(EchoResponse::build(request.get_message()))
}
