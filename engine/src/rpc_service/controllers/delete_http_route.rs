use crate::{
    core::shared_memory::ROUTER,
    rpc_service::models::{
        delete_http_route_request::DeleteHttpRouteRequest,
        delete_http_route_response::DeleteHttpRouteResponse,
    },
};
use jsonrpc_core::{Error, Value};

pub fn delete_http_route(params: Value) -> Result<Value, Error> {
    let request = DeleteHttpRouteRequest::new(params)?;
    let domain = request.get_source();

    let mut router = ROUTER.write().unwrap();
    router.delete_http_route(domain);

    let response = DeleteHttpRouteResponse::build()?;
    Ok(response)
}
