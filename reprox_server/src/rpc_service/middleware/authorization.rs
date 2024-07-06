use jsonrpc_core::{Error, Params, Value};

use crate::utils::text_utility::clear_punctation;

/// Public: This function is an middleware for providing Bearer token authentication to RPC service.
/// Remember that authorization token will be written to runtime/.session.json file.
///
/// # Arguments
/// * `authorization_token` - Private key value that specified in `settings.json` file. (`&str`)
/// (params: Params) -> Result<Value, jsonrpc_core::Error>
pub fn authorization(
    authorization_token: String,
    handler: impl Fn(Params) -> Result<Value, Error> + 'static,
) -> impl Fn(Params) -> Result<Value, Error> + 'static {
    move |params: Params| {
        let auth_token = params
            .clone()
            .parse::<Value>()
            .ok()
            .and_then(|v| v.get("token").map(|t| t.to_string()));

        if let Some(auth_token) = auth_token {
            let cleared_auth_token = clear_punctation(auth_token);
            println!("{}-{}", cleared_auth_token, authorization_token);

            if cleared_auth_token == authorization_token {
                return handler(params);
            }
        }

        Err(Error {
            code: jsonrpc_core::ErrorCode::ServerError(401),
            message: "Unauthorized".into(),
            data: None,
        })
    }
}
