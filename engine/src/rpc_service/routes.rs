use jsonrpc_core::{Error, IoHandler, Params, Value};
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::runtime::Runtime;

use super::controllers::add_https_route::add_https_route;
use super::controllers::delete_iws_route::delete_iws_route;
use super::controllers::delete_secure_iws_route::delete_secure_iws_route;
use super::controllers::list_routes::list_routes;
use super::{
    controllers::{
        add_http_route::add_http_route, add_iws_route::add_iws_route,
        add_secure_iws_route::add_secure_iws_route, delete_http_route::delete_http_route,
        delete_https_route::delete_https_route, delete_ssl_cert::delete_ssl_cert, echo::echo,
        list_ssl_certs::list_ssl_certs, upload_ssl_cert::upload_ssl_cert,
    },
    middleware::authorization::authorization,
};

pub type RpcHandler =
    Arc<dyn Fn(Arc<Mutex<Runtime>>, Params) -> Result<Value, Error> + Send + Sync>;

pub struct RPCRouter {
    route_map: HashMap<&'static str, RpcHandler>,
    runtime: Arc<Mutex<Runtime>>,
}

impl RPCRouter {
    pub fn build(runtime: Arc<Mutex<Runtime>>) -> Self {
        let mut route_map = HashMap::new();

        route_map.insert("echo", Arc::new(echo) as RpcHandler);

        route_map.insert("list_ssl_certs", Arc::new(list_ssl_certs) as RpcHandler);
        route_map.insert("upload_ssl_cert", Arc::new(upload_ssl_cert) as RpcHandler);
        route_map.insert("delete_ssl_cert", Arc::new(delete_ssl_cert) as RpcHandler);

        route_map.insert("list_routes", Arc::new(list_routes) as RpcHandler);

        route_map.insert("add_http_route", Arc::new(add_http_route) as RpcHandler);
        route_map.insert("delete_http_route", Arc::new(delete_http_route) as RpcHandler);

        route_map.insert("add_https_route", Arc::new(add_https_route) as RpcHandler);
        route_map.insert("delete_https_route",Arc::new(delete_https_route) as RpcHandler);

        route_map.insert("add_iws_route", Arc::new(add_iws_route) as RpcHandler);
        route_map.insert("delete_iws_route", Arc::new(delete_iws_route) as RpcHandler);

        route_map.insert("add_secure_iws_route", Arc::new(add_secure_iws_route) as RpcHandler);
        route_map.insert("delete_secure_iws_route", Arc::new(delete_secure_iws_route) as RpcHandler,);

        // add http content provider (web server)
        // delete http content provider
        // list http content provider

        // Task: Dynamic Life Cycle Implementation
        // update_settings
        // get_settings
        // shutdown_http_server
        // shutdown_https_server
        // shutdown_vanguard

        // boot_http_server
        // boot_https_server

        Self { route_map, runtime }
    }

    pub async fn bind(
        &self,
        mut function_register: IoHandler,
        runtime: Arc<Mutex<Runtime>>,
    ) -> IoHandler {
        let authorization_code = self.runtime.lock().unwrap().rpc_session.hash.clone();

        for (function_name, function_body) in self.route_map.iter() {
            let function_runtime = runtime.clone();
            let function_body_clone = function_body.clone();
            let function_authorization_code = authorization_code.clone();
            function_register.add_method(function_name, move |params: Params| {
                let auth_result =
                    authorization(function_authorization_code.clone(), params.clone());
                if auth_result.is_ok() {
                    let function_result =
                        function_body_clone(function_runtime.clone(), params.clone());
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
