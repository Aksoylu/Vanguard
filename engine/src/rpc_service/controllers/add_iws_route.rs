use std::{path::PathBuf, str::FromStr};

use crate::rpc_service::models::add_iws_route_request::AddIwsRouteRequest;
use crate::rpc_service::models::add_iws_route_response::AddIwsRouteResponse;
use crate::utils::directory_utility::is_path_accessible;
use crate::{core::shared_memory::ROUTER, rpc_service::rpc_error::RPCError};

use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub fn add_iws_route(params: Value) -> Result<Value, Error> {
    let request = AddIwsRouteRequest::new(params)?;

    let source = request.get_source();
    let serving_path = request.get_serving_path();

    let parsed_serving_path = PathBuf::from_str(serving_path.as_str()).unwrap_or_default();

    if !is_path_accessible(&parsed_serving_path) {
        return Err(RPCError::build(
            &StatusCode::BAD_REQUEST,
            "Given serving path does not exist or is not accessible",
        ));
    }

    let mut router = ROUTER.write().unwrap();
    router.add_iws_route(&source, &serving_path);

    let response = AddIwsRouteResponse::build()?;

    Ok(response)
}
