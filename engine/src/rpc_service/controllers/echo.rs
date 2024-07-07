use jsonrpc_core::{Error, Params, Value};

pub fn echo_controller(params: Params) -> Result<Value, Error> {
    // Example implementation
    Ok(Value::String(format!("Hello, {:?}", params)))
}