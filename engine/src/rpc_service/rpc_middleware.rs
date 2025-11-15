use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::rpc_error::RPCError;
use crate::utils::crypt_utility::decrypt_aes256_hex;

pub type RpcHandler = Arc<dyn Fn(Value) -> Result<Value, Error> + Send + Sync>;

pub struct RpcMiddleware {}

impl RpcMiddleware {
    pub fn bind(
        controller_delegate: RpcHandler,
        decryption_key: String,
        authorization_token: String,
    ) -> impl Fn(Params) -> Result<Value, Error> + Send + Sync + 'static {
        move |raw_params: Params| {
            // 1. Decrypt params
            let decrypt_payload_operation = Self::decrypt_payload(&decryption_key, &raw_params);
            if decrypt_payload_operation.is_err() {
                return Err(RPCError::badrequest(format!(
                    "Vanguard JRPC Security Warning: Failed to decrypt parameters. Details: {}",
                    decrypt_payload_operation.err().unwrap_or_default()
                )));
            }

            // 2. Extract decrypted params as payload
            let decrypted_payload = decrypt_payload_operation.unwrap();

            // 3. Check authorization token
            let check_auth_token_operation =
                Self::check_auth_token(&authorization_token, &decrypted_payload);
            if !check_auth_token_operation {
                return Err(RPCError::unauthorized(format!(
                    "Vanguard JRPC Security Warning: Wrong authorization token.",
                )));
            }

            // 4. Extract request input data from decrypted payload
            let parse_request_data_operation = serde_json::to_value(&decrypted_payload.get("data"))
                .map_err(|_| "invalid request data");

            if parse_request_data_operation.is_err() {
                return Err(RPCError::badrequest(format!(
                    "Vanguard JRPC Warning: Failed to parse request data. Details: {}",
                    parse_request_data_operation.err().unwrap_or_default()
                )));
            }

            let request_data = parse_request_data_operation.unwrap();

            controller_delegate(request_data)
        }
    }

    // Tries to decrypt the params using AES-256-GCM
    /// Expects params to be a JSON object with "nonce" and "payload" fields
    pub fn decrypt_payload(auth_token: &str, params: &Params) -> Result<Value, String> {
        let raw = serde_json::to_value(params).map_err(|_| "invalid params")?;

        let nonce = raw
            .get("nonce")
            .and_then(|v| v.as_str())
            .ok_or("missing nonce")?;

        let crypted_payload = raw
            .get("payload")
            .and_then(|v| v.as_str())
            .ok_or("missing payload")?;

        println!(
            "Decrypting with nonce: {}, payload: {}",
            nonce, crypted_payload
        );

        let decrypted_payload_as_str = decrypt_aes256_hex(&auth_token, crypted_payload, nonce);
        if decrypted_payload_as_str.is_none() {
            return Err("decryption failed".into());
        }

        let decrypted_payload_as_json: Value =
            serde_json::from_str(&decrypted_payload_as_str.unwrap_or_default())
                .map_err(|_| "incompatible payload json")?;

        Ok(decrypted_payload_as_json)
    }

    pub fn check_auth_token(valid_auth_token: &str, params: &Value) -> bool {
        let auth_token = params.get("token").and_then(|v| v.as_str());

        if auth_token.is_none() {
            return false;
        }

        let is_auth_token_correct = auth_token.unwrap_or_default() == valid_auth_token;

        is_auth_token_correct
    }
}
