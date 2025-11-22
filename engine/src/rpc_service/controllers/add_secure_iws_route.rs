use crate::core::shared_memory::ROUTER;
use crate::models::route::SecureIwsRoute;
use crate::models::ssl_context::SslContext;
use crate::rpc_service::models::{
    add_secure_iws_request::AddSecureIwsRequest, add_secure_iws_response::AddSecureIwsResponse,
};

use crate::rpc_service::rpc_error::RPCError;
use crate::utils::text_utility::normalize_string;
use crate::utils::tls_utility::validate_ssl_context;
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub fn add_secure_iws_route(params: Value) -> Result<Value, Error> {
    let request = AddSecureIwsRequest::new(params)?;

    let source = request.get_source();
    let serving_path = request.get_serving_path();
    let ssl_cert_path: String = request.get_ssl_cert_path();
    let ssl_private_key_path: String = request.get_ssl_private_key_path();

    let new_route: SecureIwsRoute = SecureIwsRoute {
        source,
        serving_path,

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
    router.add_secure_iws_route(new_route);

    Ok(AddSecureIwsResponse::build())
}

fn check_route_already_used(new_route: &SecureIwsRoute) -> Result<(), Error> {
    let router = ROUTER.read().unwrap();
    let all_route_list = router.list_routes();

    let normalized_new_source = normalize_string(&new_route.source);

    for each_route in all_route_list {
        let normalized_current_source = normalize_string(&each_route.source);

        if normalized_new_source == normalized_current_source {
            return Err(RPCError::build(
                &StatusCode::NOT_ACCEPTABLE,
                "Given source is already used by another route. Please remove it first",
            ));
        }

        if each_route.serving_path.is_some() {
            let each_serving_path = each_route.serving_path.as_ref().unwrap();
            if new_route.serving_path == *each_serving_path {
                return Err(RPCError::build(
                    &StatusCode::NOT_ACCEPTABLE,
                    "Given serving path is already used by another IWS route. Please remove it first",
                ));
            }
        }
    }

    Ok(())
}
