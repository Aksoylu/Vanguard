use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

use crate::core::rpc_client::RPCClient;

pub static RPC_CLIENT: Lazy<Arc<RwLock<RPCClient>>> = Lazy::new(|| Arc::new(RwLock::new(RPCClient::default())));