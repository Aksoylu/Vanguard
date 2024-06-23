pub mod rpc_server;

mod controllers;
mod middleware;

use hyper::{Body, Request, Response};
use jsonrpc_core::{Error, Params, Value};
use lazy_static::lazy_static;
use middleware::authorization::authorization;
use std::{collections::HashMap, sync::Arc};

use crate::rpc_service::controllers::{
    create_auth_token::create_auth_token_controller,
    echo::echo_controller,
};

// Define a static secure key
pub static SECURE_KEY: &str = "super_secure_key";

// Define the type alias for RpcHandler
pub type RpcHandler = Arc<dyn Fn(Params) -> Result<Value, Error> + Sync + Send>;
lazy_static! {
    pub static ref RPC_ROUTER: HashMap<&'static str, RpcHandler> = {
        let mut map = HashMap::new();

        // Insert handlers with authorization middleware (assuming you have it implemented)
        map.insert("say_hello", Arc::new(authorization("super_secure_key".to_string(), echo_controller)));
        map.insert("create_auth_token", Arc::new(authorization("super_secure_key".to_string(), create_auth_token_controller)));

        map
    };
}
