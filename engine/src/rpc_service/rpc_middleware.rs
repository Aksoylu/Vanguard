use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::runtime::Runtime;
use crate::utils::crypt_utility::decrypt_aes256;

pub type RpcHandler =
    Arc<dyn Fn(Arc<Mutex<Runtime>>, Params) -> Result<Value, Error> + Send + Sync>;

pub struct RpcMiddleware {
    runtime: Arc<Mutex<Runtime>>,
}

impl RpcMiddleware {
    pub fn bind(
        controller_delegate: RpcHandler,
        function_runtime: Arc<Mutex<Runtime>>,
        function_authorization_code: String,
    ) -> impl Fn(Params) -> Result<Value, Error> + Send + Sync + 'static {
        move |params: Params| {
            let decrypted_params = match Self::try_decrypt_params(&function_authorization_code, &params) {
                Ok(p) => p,
                Err(err_str) => {
                    return Err(Error {
                        code: jsonrpc_core::ErrorCode::ServerError(400),
                        message: format!(
                            "Vanguard JRPC Security Warning: Wrong cryption key or nonce: {}",
                            err_str
                        ),
                        data: None,
                    });
                }
            };

            // 2. Authorization check
            let auth_result: Result<(), Error> = Ok(()); // simülasyon
            if auth_result.is_ok() {
                controller_delegate(function_runtime.clone(), decrypted_params)
            } else {
                Err(Error {
                    code: jsonrpc_core::ErrorCode::ServerError(401),
                    message: "Unauthorized".into(),
                    data: None,
                })
            }
        }
    }

    // Tries to decrypt the params using AES-256-GCM
    /// Expects params to be a JSON object with "nonce" and "payload" fields
    pub fn try_decrypt_params(auth_token: &str, params: &Params) -> Result<Params, String> {
        let raw = serde_json::to_value(params).map_err(|_| "invalid params")?;

        let nonce = raw
            .get("nonce")
            .and_then(|v| v.as_str())
            .ok_or("missing nonce")?;

        let crypted_payload = raw
            .get("payload")
            .and_then(|v| v.as_str())
            .ok_or("missing payload")?;

        let decrypted_str = decrypt_aes256(&auth_token, crypted_payload, nonce);
        if decrypted_str.is_none() {
            return Err("decryption failed".into());
        }

        let json = serde_json::from_str(&decrypted_str.unwrap_or_default())
            .map_err(|_| "invalid decrypted json")?;

        Ok(Params::Map(serde_json::from_value(json).unwrap()))
    }
}
