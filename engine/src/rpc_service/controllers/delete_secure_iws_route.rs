use crate::rpc_service::models::delete_http_route_model::{
    DeleteHttpRouteRequest, DeleteHttpRouteResponse,
};
use crate::runtime::Runtime;
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

pub fn delete_secure_iws_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match DeleteHttpRouteRequest::new(params) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: delete_secure_iws_route".into(),
                data: None,
            });
        }
    };

    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let updated_runtime_snapshot = runtime_snapshot.delete_secure_iws_route(request.get_source());
    runtime.lock().unwrap().router = updated_runtime_snapshot;

    Ok(DeleteHttpRouteResponse::build())
}
