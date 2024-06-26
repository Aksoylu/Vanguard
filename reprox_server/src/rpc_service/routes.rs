use std::{collections::HashMap, sync::Arc};
use super::{middleware::authorization::authorization, RpcHandler};

use crate::rpc_service::controllers::{
    create_auth_token::create_auth_token_controller,
    echo::echo_controller,
};

pub struct RPCRoutes{
    route_map:HashMap<&'static str, RpcHandler>
}

impl  RPCRoutes {
    pub fn build(auth_token: String) -> Self{
        let mut map = HashMap::new();

        map.insert("echo", Arc::new(echo_controller) as RpcHandler);
        map.insert("say_hello", Arc::new(authorization(auth_token.clone(), echo_controller)) as RpcHandler);
        map.insert("create_auth_token", Arc::new(authorization(auth_token.clone(), create_auth_token_controller)) as RpcHandler);

        Self{
            route_map: map
        }
    }
}