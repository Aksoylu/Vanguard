use jsonrpc_core::{Error, Value};

use crate::{
    core::shared_memory::ROUTER,
    rpc_service::models::{
        add_http_route_request::AddHttpRouteRequest, add_http_route_response::AddHttpRouteResponse,
    },
};

/// @todo: implement traffic policy in CLI side too !
pub fn add_http_route(payload: Value) -> Result<Value, Error> {
    let request = AddHttpRouteRequest::new(payload)?;

    let source = request.get_source();
    let target = request.get_target();
    let traffic_policy = request.get_traffic_policy();

    let mut router = ROUTER.write().unwrap();
    router.add_http_route(&source, &target, traffic_policy);

    let response = AddHttpRouteResponse::build()?;
    Ok(response)
}
