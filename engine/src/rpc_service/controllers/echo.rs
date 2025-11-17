use jsonrpc_core::{Error, Value};

use crate::rpc_service::models::{echo_request::EchoRequest, echo_response::EchoResponse};

pub fn echo(payload: Value) -> Result<Value, Error> {
    let request = EchoRequest::new(payload)?;

    Ok(EchoResponse::build(request.get_message()))
}
