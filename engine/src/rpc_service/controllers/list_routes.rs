use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::models::get_http_routes_model::GetHttpRouteResponse;
use crate::runtime::Runtime;

pub fn list_routes(runtime: Arc<Mutex<Runtime>>, _params: Params) -> Result<Value, Error> {
    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let all_routes = runtime_snapshot.list_routes();

    Ok(GetHttpRouteResponse::build(all_routes))
}
