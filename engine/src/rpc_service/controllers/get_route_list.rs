use crate::common::enums::route_type::RouteType;
use crate::rpc_service::models::get_route_list_response::GetRouteListResponse;
use crate::{
    core::shared_memory::ROUTER, rpc_service::models::get_route_list_request::GetRouteListRequest,
};
use jsonrpc_core::{Error, Value};

pub fn get_route_list(params: Value) -> Result<Value, Error> {
    let request = GetRouteListRequest::new(params)?;
    let route_type = request.get_route_type();
    
    let router = ROUTER.read().unwrap();

    let mut response = GetRouteListResponse::new();

    if route_type == RouteType::All || route_type == RouteType::Http {
        response.http_routes = Some(router.get_http_routes());
    }

    if route_type == RouteType::All || route_type == RouteType::Https {
        response.https_routes = Some(router.get_https_routes());
    }

    if route_type == RouteType::All || route_type == RouteType::Iws {
        response.iws_routes = Some(router.get_iws_routes());
    }

    if route_type == RouteType::All || route_type == RouteType::SecureIws {
        response.secure_iws_routes = Some(router.get_secure_iws_routes());
    }

    Ok(response.build())
}
