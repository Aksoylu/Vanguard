use jsonrpc_core::{Error, Value};

use crate::core::shared_memory::ROUTER;
use crate::rpc_service::models::delete_http_route_request::DeleteHttpRouteRequest;
use crate::rpc_service::models::delete_https_route_response::DeleteHttpsRouteResponse;

pub fn delete_https_route(params: Value) -> Result<Value, Error> {
    let request = DeleteHttpRouteRequest::new(params)?;
    let domain = request.get_source();

    let mut router = ROUTER.write().unwrap();
    router.delete_https_route(domain);

    Ok(DeleteHttpsRouteResponse::build())
}
