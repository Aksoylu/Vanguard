use crate::core::rpc_client::RPCClient;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

pub static RPC_CLIENT: Lazy<Arc<RwLock<RPCClient>>> =
    Lazy::new(|| Arc::new(RwLock::new(RPCClient::default())));
