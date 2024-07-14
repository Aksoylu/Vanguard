use jsonrpc_core::{Error, IoHandler, Params, Value};
use std::{collections::HashMap, sync::Arc};
use std::sync::Mutex;

use crate::{rpc_service::controllers::echo::echo_controller, runtime::Runtime};

use super::{controllers::{add_http_route::add_http_route, get_http_routes::get_http_routes}, middleware::authorization::authorization};

pub type RpcHandler =
    Arc<dyn Fn(Arc<Mutex<Runtime>>, Params) -> Result<Value, Error> + Send + Sync>;

pub struct RPCRouter {
    route_map: HashMap<&'static str, RpcHandler>,
    runtime: Arc<Mutex<Runtime>>
}

impl RPCRouter {
    pub fn build(runtime: Arc<Mutex<Runtime>>) -> Self {
        let mut route_map = HashMap::new();

        route_map.insert("echo", Arc::new(echo_controller) as RpcHandler);
        route_map.insert("get_http_routes", Arc::new(get_http_routes) as RpcHandler);
        route_map.insert("add_http_route", Arc::new(add_http_route) as RpcHandler);

        Self { route_map, runtime}
    }

    pub async fn bind(
        &self,
        mut function_register: IoHandler,
        runtime: Arc<Mutex<Runtime>>,
    ) -> IoHandler {
        let authorization_code = self.runtime.lock().unwrap().rpc_session.hash.clone();
        
        for (function_name, function_body) in self.route_map.iter() {
            let function_runtime = runtime.clone(); // Clone runtime for each iteration
            let function_body_clone = function_body.clone();
            let function_authorization_code = authorization_code.clone();
            function_register.add_method(function_name, move |params: Params| {
                
                let auth_result = authorization(function_authorization_code.clone(), params.clone());
                if auth_result.is_ok(){
                    let function_result = function_body_clone(function_runtime.clone(), params.clone());
                    return function_result;
                }
                
                return Err(Error {
                    code: jsonrpc_core::ErrorCode::ServerError(401),
                    message: "Unauthorized".into(),
                    data: None,
                });
            });
        }

        function_register
    }
}
