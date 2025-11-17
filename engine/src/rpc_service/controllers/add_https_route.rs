use crate::core::shared_memory::ROUTER;
use crate::models::route::HttpsRoute;
use crate::models::ssl_context::SslContext;

use crate::rpc_service::models::add_https_route_request::AddHttpsRouteRequest;
use crate::rpc_service::models::add_https_route_response::AddHttpsRouteResponse;
use crate::rpc_service::rpc_error::RPCError;
use crate::utils::file_utility::get_absolute_ssl_file_path;
use crate::utils::tls_utility::validate_ssl_context;
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

/// Checks that given ssl cert file and private key file path.
/// Remember that this function also considers relative (Upload path included, as @vanguard) check
/// At the last step, this function also   ssl ceretificate is valid and compatible with given domain & IP address
/// Is ssl context is valid, creates HTTPS Route
pub fn add_https_route(params: Value) -> Result<Value, Error> {
    let request = AddHttpsRouteRequest::new(params)?;

    let source: String = request.get_source();
    let target: String = request.get_target();
    let ssl_cert_path: String = request.get_ssl_cert_path();
    let ssl_private_key_path: String = request.get_ssl_private_key_path();

    let is_ssl_cert_valid = validate_ssl_context(&source, &ssl_cert_path, &ssl_private_key_path);

    if !is_ssl_cert_valid {
        return Err(RPCError::build(
            &StatusCode::NOT_ACCEPTABLE,
            "Ssl cert, private key and given source domain is not compatible ",
        ));
    }

    let new_route = HttpsRoute {
        source,
        target,

        ssl_context: SslContext {
            certificate_file_path: ssl_cert_path,
            private_key_file_path: ssl_private_key_path,
        },
    };

    let mut router = ROUTER.write().unwrap();
    router.add_https_route(new_route);

    Ok(AddHttpsRouteResponse::build(None))
}
