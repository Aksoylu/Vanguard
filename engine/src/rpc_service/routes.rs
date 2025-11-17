use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::rpc_service::rpc_middleware::RpcHandler;

use super::controllers::add_https_route::add_https_route;
use super::controllers::delete_iws_route::delete_iws_route;
use super::controllers::delete_secure_iws_route::delete_secure_iws_route;
use super::controllers::list_routes::list_routes;
use super::controllers::{
    add_http_route::add_http_route, add_iws_route::add_iws_route,
    add_secure_iws_route::add_secure_iws_route, delete_http_route::delete_http_route,
    delete_https_route::delete_https_route, delete_ssl_cert::delete_ssl_cert, echo::echo,
    list_ssl_certs::list_ssl_certs, upload_ssl_cert::upload_ssl_cert,
};

pub static ROUTES: Lazy<Vec<(&'static str, RpcHandler)>> = Lazy::new(|| {
    vec![
        ("echo", Arc::new(echo) as RpcHandler),

        ("add_http_route", Arc::new(add_http_route) as RpcHandler),
        ("add_https_route", Arc::new(add_https_route) as RpcHandler),
        ("add_iws_route", Arc::new(add_iws_route) as RpcHandler),
        ("add_secure_iws_route",Arc::new(add_secure_iws_route) as RpcHandler),

        /*
        ("list_ssl_certs", Arc::new(list_ssl_certs) as RpcHandler),
        ("upload_ssl_cert", Arc::new(upload_ssl_cert) as RpcHandler),
        ("delete_ssl_cert", Arc::new(delete_ssl_cert) as RpcHandler),
        ("list_routes", Arc::new(list_routes) as RpcHandler),

        ("delete_http_route",Arc::new(delete_http_route) as RpcHandler),
        ("delete_https_route",Arc::new(delete_https_route) as RpcHandler),
        ("delete_iws_route", Arc::new(delete_iws_route) as RpcHandler),
        ("delete_secure_iws_route",Arc::new(delete_secure_iws_route) as RpcHandler),
        */
    ]
});
