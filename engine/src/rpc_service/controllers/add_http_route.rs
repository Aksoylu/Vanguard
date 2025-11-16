use crate::core::shared_memory::ROUTER;
use crate::models::route::HttpRoute;
use crate::rpc_service::models::add_http_route_request::AddHttpRouteRequest;
use crate::rpc_service::models::add_http_route_response::AddHttpRouteResponse;
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Value};


pub fn add_http_route(payload: Value) -> Result<Value, Error> {
    let request = match AddHttpRouteRequest::new(payload) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: add_http_route".into(),
                data: None,
            });
        }
    };

    let new_route = HttpRoute {
        source: request.get_source(),
        target: request.get_target(),
    };

    let mut router = ROUTER.write().unwrap();
    router.add_http_route(new_route);


    Ok(AddHttpRouteResponse::build("Success".to_string(), None))
}
