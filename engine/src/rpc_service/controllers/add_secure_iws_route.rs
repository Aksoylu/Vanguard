use crate::core::shared_memory::ROUTER;
use crate::rpc_service::models::{
    add_secure_iws_route_request::AddSecureIwsRouteRequest,
    add_secure_iws_route_response::AddSecureIwsRouteResponse,
};

use crate::rpc_service::rpc_error::RPCError;
use crate::utils::text_utility::normalize_string;
use crate::utils::tls_utility::validate_ssl_context;
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub fn add_secure_iws_route(params: Value) -> Result<Value, Error> {
    let request = AddSecureIwsRouteRequest::new(params)?;

    let source = request.get_source();
    let serving_path = request.get_serving_path();
    let ssl_cert_path: String = request.get_ssl_cert_path();
    let ssl_private_key_path: String = request.get_ssl_private_key_path();
    let traffic_policy = request.get_traffic_policy();

    validate_ssl_context(&source, &ssl_cert_path, &ssl_private_key_path)?;

    check_route_already_used(&source, &serving_path)?;

    let mut router = ROUTER.write().unwrap();
    router.add_secure_iws_route(
        &source,
        &serving_path,
        &ssl_cert_path,
        &ssl_private_key_path,
        traffic_policy,
    );

    Ok(AddSecureIwsRouteResponse::build())
}

fn check_route_already_used(source: &String, serving_path: &String) -> Result<(), Error> {
    let router = ROUTER.read().unwrap();
    let all_route_list = router.list_routes();

    let normalized_new_source = normalize_string(&source);

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
            if serving_path.clone() == *each_serving_path {
                return Err(RPCError::build(
                    &StatusCode::NOT_ACCEPTABLE,
                    "Given serving path is already used by another IWS route. Please remove it first",
                ));
            }
        }
    }

    Ok(())
}
