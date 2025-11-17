use jsonrpc_core::{Error, Value};

use crate::{
    core::shared_memory::ROUTER,
    models::route::HttpRoute,
    rpc_service::models::{
        add_http_route_request::AddHttpRouteRequest, add_http_route_response::AddHttpRouteResponse,
    },
};

pub fn add_http_route(payload: Value) -> Result<Value, Error> {
    let request = AddHttpRouteRequest::new(payload)?;

    let new_route = HttpRoute {
        source: request.get_source(),
        target: request.get_target(),
    };

    let mut router = ROUTER.write().unwrap();
    router.add_http_route(new_route);

    Ok(AddHttpRouteResponse::build(None))
}
