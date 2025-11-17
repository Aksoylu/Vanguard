use jsonrpc_core::{Error, Value};

use crate::core::shared_memory::ROUTER;
use crate::rpc_service::models::delete_iws_route_request::DeleteIwsRouteRequest;
use crate::rpc_service::models::delete_iws_route_response::DeleteIwsRouteResponse;

pub fn delete_iws_route(params: Value) -> Result<Value, Error> {
    let request = DeleteIwsRouteRequest::new(params)?;
    let domain = request.get_source();

    let mut router = ROUTER.write().unwrap();
    router.delete_iws_route(domain);

    Ok(DeleteIwsRouteResponse::build())
}
