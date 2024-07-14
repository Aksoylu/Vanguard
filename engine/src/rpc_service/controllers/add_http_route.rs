use crate::{models::route::HttpRoute, runtime::Runtime, utils::rpc_utility::RpcParameter};
use jsonrpc_core::{Error, IoHandler, Params, Value};
use std::borrow::BorrowMut;
use std::sync::Arc;
use std::sync::Mutex;

pub fn add_http_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let http_route_name = RpcParameter::extract_string("route_name", params.clone());
    if http_route_name.is_none() {
        return Err(Error {
            code: jsonrpc_core::ErrorCode::ServerError(500),
            message: "Route name is not valid".into(),
            data: None,
        });
    }

    let new_route = HttpRoute {
        source: "xyz".to_owned(),
        target: "abc".to_owned(),
    };

    let rt = runtime.lock().unwrap().router.clone();

    let new_rt = rt.add_http_route(http_route_name.unwrap(), new_route);

    runtime.lock().unwrap().router = new_rt;

    Ok(Value::String(format!("Success")))
}
