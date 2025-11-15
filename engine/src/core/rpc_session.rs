use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::{
    constants::Constants,
    utils::{crypt_utility::generate_hash, time_utility::get_current_timestamp},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RpcSession {
    pub ip_addr: String,
    pub port: u16,
    pub created_at: i64,
    pub private_secret_key: String,
    pub aes_encryption_key: String,
    pub authorization_token: String,
}

impl Default for RpcSession {
    fn default() -> Self {
        RpcSession::init(
            Constants::DEFAULT_RPC_IP.to_string(),
            Constants::DEFAULT_RPC_PORT,
            Constants::DEFAULT_PRIVATE_SECRET_KEY.to_string(),
        )
    }
}

impl RpcSession {
    pub fn init(ip_addr: String, port: u16, private_secret_key: String) -> Self {
        let created_at = get_current_timestamp();

        let aes_encryption_key = generate_hash(private_secret_key.clone());

        let clean_authorization_token = format!("{}{}", aes_encryption_key, created_at);
        let authorization_token = generate_hash(clean_authorization_token);

        Self {
            ip_addr,
            port,
            created_at,
            private_secret_key,
            aes_encryption_key,
            authorization_token,
        }
    }
}
