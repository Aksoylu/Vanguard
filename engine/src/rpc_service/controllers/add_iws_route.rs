use crate::models::route::IwsRoute;
use crate::rpc_service::models::add_iws_route_model::{AddIwsRouteRequest, AddIwsRouteResponse};
use crate::runtime::Runtime;
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

pub fn add_iws_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match AddIwsRouteRequest::new(params) {
        Ok(req) => req,
        Err(err) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: err.message,
                data: None,
            });
        }
    };

    let source = request.get_source();
    let serving_path = request.get_serving_path();

    let runtime_snapshot = runtime.lock().unwrap().router.clone();

    /*  If record with source already exist in route or serving path is already used by another IWS route, terminate flow */
    let route_list = runtime_snapshot.list_routes();
    for route in route_list {
        if route.source == source.clone() {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Route source already registered".into(),
                data: None,
            });
        }

        if route.serving_path.is_some() {
            let each_serving_path = route.serving_path.unwrap_or_default().clone();

            if each_serving_path == serving_path.clone() {
                return Err(Error {
                    code: ErrorCode::InternalError,
                    message: "Route serving path already used by another IWS route".into(),
                    data: None,
                });
            }
        }
    }

    /* Seperate business flow into two branches by [use_ssl] property */
    let new_iws_route = IwsRoute {
        source: source.clone(),
        serving_path: serving_path.clone(),
    };

    let updated_runtime_snapshot = runtime_snapshot.add_iws_route(new_iws_route);

    runtime.lock().unwrap().router = updated_runtime_snapshot;
    Ok(AddIwsRouteResponse::build("ok".to_string()))
}
