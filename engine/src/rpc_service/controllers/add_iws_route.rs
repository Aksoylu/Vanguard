use crate::core::shared_memory::ROUTER;
use crate::models::route::IwsRoute;
use crate::rpc_service::models::add_iws_route_request::AddIwsRouteRequest;
use crate::rpc_service::models::add_iws_route_response::AddIwsRouteResponse;
use crate::rpc_service::rpc_error::RPCError;

use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub fn add_iws_route(params: Value) -> Result<Value, Error> {
    let request = AddIwsRouteRequest::new(params)?;

    let source = request.get_source();
    let serving_path = request.get_serving_path();

    //  If record with source already exist in route or serving path is already used by another IWS route, terminate flow
    let router_readonly = ROUTER.read().unwrap();
    let route_list = router_readonly.list_routes();

    for route in route_list {
        if route.source == source.clone() {
            return Err(RPCError::build(
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Route source already registered",
            ));
        }

        if route.serving_path.is_some() {
            let each_serving_path = route.serving_path.unwrap_or_default().clone();

            if each_serving_path == serving_path.clone() {
                return Err(RPCError::build(
                    &StatusCode::INTERNAL_SERVER_ERROR,
                    "Route serving path already used by another IWS route",
                ));
            }
        }
    }

    let new_iws_route = IwsRoute {
        source: source.clone(),
        serving_path: serving_path.clone(),
    };

    let mut router = ROUTER.write().unwrap();
    router.add_iws_route(new_iws_route);

    Ok(AddIwsRouteResponse::build())
}
