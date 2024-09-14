use crate::rpc_service::models::delete_https_route_model::{
    DeleteHttpsRouteRequest, DeleteHttpsRouteResponse,
};
use crate::runtime::Runtime;
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

pub fn delete_https_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match DeleteHttpsRouteRequest::new(params) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: delete_https_route".into(),
                data: None,
            });
        }
    };

    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let updated_runtime_snapshot = runtime_snapshot.delete_https_route(request.get_source());
    runtime.lock().unwrap().router = updated_runtime_snapshot;

    Ok(DeleteHttpsRouteResponse::build())
}
