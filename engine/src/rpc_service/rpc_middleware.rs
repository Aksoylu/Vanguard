use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::rpc_error::RPCError;
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
        decryption_key: String,
        authorization_token: String,
    ) -> impl Fn(Params) -> Result<Value, Error> + Send + Sync + 'static {
        move |params: Params| {
            // 1. Decrypt params
            let decrypt_params_operation = Self::decrypt_params(&decryption_key, &params);
            if decrypt_params_operation.is_err() {
                return Err(RPCError::badrequest(format!(
                    "Vanguard JRPC Security Warning: Failed to decrypt parameters. Details: {}",
                    decrypt_params_operation.err().unwrap_or_default()
                )));
            }

            let payload = decrypt_params_operation.unwrap();

            let check_auth_token_operation = Self::check_auth_token(&authorization_token, &payload);
            if !check_auth_token_operation {
                return Err(RPCError::unauthorized(format!(
                    "Vanguard JRPC Security Warning: Wrong cryption key or nonce. Details: {}",
                    decrypt_params_operation.err().unwrap_or_default()
                )));
            }

            let payload_auth_token = check_auth_token_operation.unwrap_or_default();

            let parsed_payload = Params::Map(
                serde_json::from_value(decrypted_payload_as_json)
                    .map_err(|_| "invalid params map")?,
            );

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
    pub fn decrypt_params(auth_token: &str, params: &Params) -> Result<Value, String> {
        let raw = serde_json::to_value(params).map_err(|_| "invalid params")?;

        let nonce = raw
            .get("nonce")
            .and_then(|v| v.as_str())
            .ok_or("missing nonce")?;

        let crypted_payload = raw
            .get("payload")
            .and_then(|v| v.as_str())
            .ok_or("missing payload")?;

        let decrypted_payload_as_str = decrypt_aes256(&auth_token, crypted_payload, nonce);
        if decrypted_payload_as_str.is_none() {
            return Err("decryption failed".into());
        }

        let decrypted_payload_as_json =
            serde_json::from_str(&decrypted_payload_as_str.unwrap_or_default())
                .map_err(|_| "incompatible payload json")?;

        Ok(decrypted_payload_as_json)
    }

    pub fn check_auth_token(valid_auth_token: &str, params: &Value) -> bool {
        let raw = serde_json::to_value(params).map_err(|_| "invalid params");

        let nonce = raw
            .get("nonce")
            .and_then(|v| v.as_str())
            .ok_or("missing nonce");

        if auth_token != expected_token {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(401),
                message: "Unauthorized".into(),
                data: None,
            });
        }
        Ok(())
    }
}
