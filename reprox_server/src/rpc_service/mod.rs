pub mod rpc_server;

mod controllers;

use hyper::{Body, Request, Response};
use jsonrpc_core::{Error, Params};
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::HashMap;

use crate::rpc_service::controllers::{create_auth_token::create_auth_token_controller, echo::echo_controller};


lazy_static! {
    pub static ref RPC_ROUTER: HashMap<&'static str, fn(Params) -> Result<Value, Error>> = {
        let mut map = HashMap::new();
        map.insert(
            "say_hello",
            echo_controller as fn(Params) -> Result<Value, Error>,
        );

        map.insert(
            "create_auth_token",
            create_auth_token_controller as fn(Params) -> Result<Value, Error>,
        );
        map
    };
}
