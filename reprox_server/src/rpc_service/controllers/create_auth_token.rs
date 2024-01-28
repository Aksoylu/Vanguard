use hyper::{Body, Request, Response};
use jsonrpc_core::Params;
use serde_json::Value;

pub fn create_auth_token_controller(_params: Params) -> Result<Value, jsonrpc_core::Error> {
    Ok(Value::String("Hello, JSON-RPC!".to_string()))
}
