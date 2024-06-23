use std::sync::Arc;

use jsonrpc_core::{params, Error, ErrorCode, Params, Result as RpcResult, Value};

use crate::rpc_service::RpcHandler;

/// Public: This function is an middleware for providing Bearer token authentication to RPC service. 
/// Remember that authorization token will be written to runtime/.session.json file.
/// 
/// # Arguments
/// * `authorization_token` - Private key value that specified in `settings.json` file. (`&str`)
/// (params: Params) -> Result<Value, jsonrpc_core::Error> 
pub fn authorization(
    authorization_token: String,
    handler: impl Fn(Params) -> Result<Value, Error> + 'static
) -> impl Fn(Params) -> Result<Value, Error> + 'static {
    move |params: Params| {
        // Simplified authorization logic
        let auth_token = params.clone().parse::<Value>().ok().and_then(|v| v.get("token").map(|t| t.to_string()));

        if let Some(auth_token) = auth_token {
            if auth_token == authorization_token {
                // Token is valid, call the actual handler
                return handler(params);
            }
        }
        
        // If token is invalid or missing, return an error
        Err(Error {
            code: jsonrpc_core::ErrorCode::ServerError(401),
            message: "Unauthorized".into(),
            data: None,
        })
    }
}
