use crate::rpc_service::models::delete_http_route_model::{DeleteHttpRouteRequest, DeleteHttpRouteResponse};
use crate::rpc_service::models::delete_iws_route_model::{DeleteIwsRouteRequest, DeleteIwsRouteResponse};
use crate::runtime::Runtime;
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;
use jsonrpc_core::ErrorCode;

pub fn delete_iws_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match DeleteIwsRouteRequest::new(params) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: delete_iws_route".into(),
                data: None,
            });
        }
    };

    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let updated_runtime_snapshot = runtime_snapshot.delete_iws_route( request.get_source());
    runtime.lock().unwrap().router = updated_runtime_snapshot;

    Ok(DeleteIwsRouteResponse::build())
}
