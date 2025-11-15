use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::models::get_http_route_list_response::GetHttpRouteListResponse;
use crate::boot::Runtime;

pub fn list_routes(runtime: Arc<Mutex<Runtime>>, _params: Params) -> Result<Value, Error> {
    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let all_routes = runtime_snapshot.list_routes();

    Ok(GetHttpRouteListResponse::build(all_routes))
}
