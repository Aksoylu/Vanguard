use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::rpc_service::{rpc_middleware::RpcHandler};

use super::controllers::{
    add_http_route::add_http_route, add_https_route::add_https_route, add_iws_route::add_iws_route,
    add_secure_iws_route::add_secure_iws_route, delete_http_route::delete_http_route,
    delete_https_route::delete_https_route, delete_iws_route::delete_iws_route,
    delete_secure_iws_route::delete_secure_iws_route, echo::echo, list_routes::list_routes,
    get_uploaded_ssl_entity_list::get_uploaded_ssl_entity_list
};

pub static ROUTES: Lazy<Vec<(&'static str, RpcHandler)>> = Lazy::new(|| {
    vec![
        ("echo", Arc::new(echo) as RpcHandler),
        ("add_http_route", Arc::new(add_http_route) as RpcHandler),
        ("add_https_route", Arc::new(add_https_route) as RpcHandler),
        ("add_iws_route", Arc::new(add_iws_route) as RpcHandler),
        ("add_secure_iws_route",Arc::new(add_secure_iws_route) as RpcHandler),
        ("delete_http_route",Arc::new(delete_http_route) as RpcHandler),
        ("delete_https_route",Arc::new(delete_https_route) as RpcHandler),
        ("delete_iws_route", Arc::new(delete_iws_route) as RpcHandler),
        ("delete_secure_iws_route",Arc::new(delete_secure_iws_route) as RpcHandler),
        ("get_uploaded_ssl_entity_list", Arc::new(get_uploaded_ssl_entity_list) as RpcHandler),
        //("upload_ssl_cert", Arc::new(upload_ssl_cert) as RpcHandler),
        //("delete_ssl_cert", Arc::new(delete_ssl_cert) as RpcHandler),
        ("list_routes", Arc::new(list_routes) as RpcHandler),
    ]
});
