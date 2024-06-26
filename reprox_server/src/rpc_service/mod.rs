pub mod rpc_server;

mod controllers;
mod middleware;
mod routes;
mod models;

use jsonrpc_core::{Error, Params, Value};
use lazy_static::lazy_static;
use middleware::authorization::authorization;
use std::{collections::HashMap, sync::Arc};

use crate::rpc_service::controllers::{
    create_auth_token::create_auth_token_controller,
    echo::echo_controller,
};

// Define a static secure key
pub static SECURE_KEY: &str = "ua199806";

// Define the type alias for RpcHandler

pub type RpcHandler = Arc<dyn Fn(Params) -> Result<Value, Error> + Sync + Send>;

lazy_static! {
    /// Public Static : This is an Lazy-Static hashmap configures of endpoints for Vanguard JRPC server.
    ///  This JRPC requests may invoke by CLI, GUI or direct RPC 
    /// 
    /// # Arguments
    /// * `private_key` - Private key value that specified in `settings.json` file. (`&str`)
    ///
    /// 
    pub static ref RPC_ROUTER: HashMap<&'static str, RpcHandler> = {
        let mut map = HashMap::new();

        // Insert handlers with authorization middleware (assuming you have it implemented)
        map.insert("echo", Arc::new(echo_controller) as RpcHandler);
        map.insert("say_hello", Arc::new(authorization("ua199806".to_string(), echo_controller)) as RpcHandler);
        map.insert("create_auth_token", Arc::new(authorization("ua199806".to_string(), create_auth_token_controller)) as RpcHandler);

        map
    };

}