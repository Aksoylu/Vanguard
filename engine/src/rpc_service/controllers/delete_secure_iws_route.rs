use jsonrpc_core::{Error, Value};

use crate::core::shared_memory::ROUTER;
use crate::rpc_service::models::delete_secure_iws_route_request::DeleteSecureIwsRouteRequest;
use crate::rpc_service::models::delete_secure_iws_route_response::DeleteSecureIwsRouteResponse;

pub fn delete_secure_iws_route(params: Value) -> Result<Value, Error> {
    let request = DeleteSecureIwsRouteRequest::new(params)?;
    let domain = request.get_source();

    let mut router = ROUTER.write().unwrap();
    router.delete_secure_iws_route(domain);

    Ok(DeleteSecureIwsRouteResponse::build())
}
