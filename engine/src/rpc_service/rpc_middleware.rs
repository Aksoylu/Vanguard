use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;
use jsonrpc_core::{Params, Value};


use crate::runtime::Runtime;

pub type RpcHandler =
    Arc<dyn Fn(Arc<Mutex<Runtime>>, Params) -> Result<Value, Error> + Send + Sync>;

pub struct RpcMiddleware {
    runtime: Arc<Mutex<Runtime>>,
}

pub fn bind(
    controller_delegate: RpcHandler,
    function_runtime: Arc<Mutex<Runtime>>,
    function_authorization_code: String,
) -> impl Fn(Params) -> Result<Value, Error> + Send + Sync + 'static {
    move |params: Params| {
        // 1. Decrypt incoming payload
        let decrypted_params = match try_decrypt_params(&function_authorization_code, &params) {
            Ok(p) => p,
            Err(e) => {
                return Err(Error {
                    code: jsonrpc_core::ErrorCode::ServerError(400),
                    message: format!(
                        "Vanguard JRPC Security Warning: Wrong cryption key or nonce: {}",
                        e
                    ),
                    data: None,
                });
            }
        };

        // 2. Authorization check
        let auth_result: Result<(), Error> = Ok(()); // simülasyon
        if auth_result.is_ok() {
            // Remove the 'return' and just call the delegate
            controller_delegate(function_runtime.clone(), decrypted_params)
        } else {
            Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(401),
                message: "Unauthorized".into(),
                data: None,
            })
        }
    }
        
    pub fn try_decrypt_params(auth_token: &str, params: &Params) -> Result<Params, String> {
        let raw: JsonValue = serde_json::to_value(params).map_err(|_| "invalid params")?;

        // params: { "nonce": "...", "ciphertext": "..." }
        let nonce = raw
            .get("nonce")
            .and_then(|v| v.as_str())
            .ok_or("missing nonce")?;
        let ciphertext = raw
            .get("ciphertext")
            .and_then(|v| v.as_str())
            .ok_or("missing ciphertext")?;

        // derive 32-byte key from auth_token
        let mut key_bytes = [0u8; 32];
        let token_bytes = auth_token.as_bytes();
        for (i, b) in token_bytes.iter().enumerate().take(32) {
            key_bytes[i] = *b;
        }

        let decrypted_str = decrypt_payload(&key_bytes, ciphertext, nonce)?;
        let json: JsonValue =
            serde_json::from_str(&decrypted_str).map_err(|_| "invalid decrypted json")?;

        Ok(Params::Map(serde_json::from_value(json).unwrap()))
    }

}
