use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::rpc_service::{controllers::{get_build_version::get_build_version,}, rpc_middleware::RpcHandler};

use super::controllers::{
    add_http_route::add_http_route, add_https_route::add_https_route, add_iws_route::add_iws_route,
    add_secure_iws_route::add_secure_iws_route, delete_http_route::delete_http_route,
    delete_https_route::delete_https_route, delete_iws_route::delete_iws_route,
    delete_secure_iws_route::delete_secure_iws_route, echo::echo, 
    get_route_list::get_route_list,
    get_status::get_status
};

pub static ROUTES: Lazy<Vec<(&'static str, RpcHandler)>> = Lazy::new(|| {
    vec![
        ("echo", Arc::new(echo) as RpcHandler),
        ("get_build_version", Arc::new(get_build_version) as RpcHandler),
        ("add_http_route", Arc::new(add_http_route) as RpcHandler),
        ("add_https_route", Arc::new(add_https_route) as RpcHandler),
        ("add_iws_route", Arc::new(add_iws_route) as RpcHandler),
        ("add_secure_iws_route",Arc::new(add_secure_iws_route) as RpcHandler),
        ("delete_http_route",Arc::new(delete_http_route) as RpcHandler),
        ("delete_https_route",Arc::new(delete_https_route) as RpcHandler),
        ("delete_iws_route", Arc::new(delete_iws_route) as RpcHandler),
        ("delete_secure_iws_route",Arc::new(delete_secure_iws_route) as RpcHandler),
        ("get_route_list", Arc::new(get_route_list) as RpcHandler),
        ("get_status", Arc::new(get_status) as RpcHandler)
    ]
});
