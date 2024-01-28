use hyper::{Body, Request, Response};
use jsonrpc_core::Params;
use serde_json::Value;

pub fn echo_controller(params: Params) -> Result<Value, jsonrpc_core::Error> {
    Ok(Value::String("Hello, JSON-RPC!".to_string()))
}
