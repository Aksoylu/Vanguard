use crate::core::shared_memory::ROUTER;
use crate::rpc_service::models::get_http_route_list_response::GetHttpRouteListResponse;
use jsonrpc_core::{Error, Value};

pub fn get_http_route_list(_params: Value) -> Result<Value, Error> {
    let router = ROUTER.read().unwrap();
    let all_routes = router.list_routes();

    Ok(GetHttpRouteListResponse::build(all_routes))
}
