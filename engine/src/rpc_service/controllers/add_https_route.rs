use crate::models::route::HttpsRoute;
use crate::models::ssl_context::SslContext;
use crate::utils::text_utility::normalize_string;
use crate::{core::shared_memory::ROUTER};

use crate::rpc_service::models::add_https_route_request::AddHttpsRouteRequest;
use crate::rpc_service::models::add_https_route_response::AddHttpsRouteResponse;
use crate::rpc_service::rpc_error::RPCError;
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

    let new_route = HttpsRoute {
        source,
        target,

        ssl_context: SslContext {
            certificate_file_path: ssl_cert_path,
            private_key_file_path: ssl_private_key_path,
        },
    };

    validate_ssl_context(
        &new_route.source,
        &new_route.ssl_context.certificate_file_path,
        &new_route.ssl_context.private_key_file_path,
    )?;

    check_route_already_used(&new_route)?;

    let mut router = ROUTER.write().unwrap();
    router.add_https_route(new_route);

    Ok(AddHttpsRouteResponse::build(None))
}

fn check_route_already_used(new_route: &HttpsRoute) -> Result<(), Error> {
    let router = ROUTER.read().unwrap();
    let all_route_list = router.list_routes();

    let normalized_new_source = normalize_string(&new_route.source);

    for each_route in all_route_list {
        let normalized_current_source = normalize_string(&each_route.source);

        if normalized_new_source == normalized_current_source {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Given source is already used by another route",
            ));
        }
    }

    Ok(())
}
