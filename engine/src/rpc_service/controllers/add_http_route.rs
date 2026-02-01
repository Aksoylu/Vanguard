use jsonrpc_core::{Error, Value};

use crate::{
    core::shared_memory::ROUTER,
    rpc_service::models::{
        add_http_route_request::AddHttpRouteRequest, add_http_route_response::AddHttpRouteResponse,
    },
};

/// @todo: implement global upstream settings logic here
pub fn add_http_route(payload: Value) -> Result<Value, Error> {
    let request = AddHttpRouteRequest::new(payload)?;

    let source = request.get_source();
    let target = request.get_target();


    let mut router = ROUTER.write().unwrap();
    router.add_http_route(&source, &target);

    let response = AddHttpRouteResponse::build()?;
    Ok(response)
}
