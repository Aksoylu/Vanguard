use serde::{Deserialize, Serialize};
use crate::{core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT}, models::rpc::rpc_payload::RPCPayload, utils::crypt_utility::{encrypt_aes256_hex, generate_nonce_hex}};

/// Middle data layer of wrapped RPC request 
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RPCParams {
    pub payload: String,
    pub nonce: String,
}

impl RPCParams{
    pub fn new(payload: RPCPayload) -> Result<Self, RPCBaseError>{

        let serialized_json = serde_json::to_string(&payload).map_err(|_| {
            RPCBaseError::build("Can not serialized RPC payload")
        })?;

        let rpc_client = RPC_CLIENT.read().unwrap();
        let aes_encryption_key = rpc_client
            .boot_data.as_ref()
            .and_then(|boot_data| boot_data.rpc_session.as_ref()) 
            .map(|rpc_session_info| rpc_session_info.aes_encryption_key.clone()); 

        if aes_encryption_key.is_none(){
            return Err(RPCBaseError::build("Encryption key can not found in session data"));
        }

        let nonce = generate_nonce_hex();

        let encrypt_payload = encrypt_aes256_hex(
            aes_encryption_key.unwrap().as_str(),
            serialized_json.as_str(),
        &nonce.as_str());

        if encrypt_payload.is_none(){
            return Err(RPCBaseError::build("Encryption failed, please check session information"));
        }

        let encrypted_payload = encrypt_payload.unwrap();

        Ok(Self { 
            payload: encrypted_payload, 
            nonce
        })
    }
}