use jsonrpc_core::{Error, ErrorCode, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::models::echo_model::{EchoRequest, EchoResponse};
use crate::runtime::Runtime;

pub fn echo(_runtime: Arc<Mutex<Runtime>>, payload: Value) -> Result<Value, Error> {
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
