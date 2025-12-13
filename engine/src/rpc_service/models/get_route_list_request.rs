use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

use crate::common::enums::route_type::RouteType;
use crate::rpc_service::rpc_error::RPCError;
use crate::utils::rpc_utility::RpcParameter;

pub struct GetRouteListRequest {
    pub route_type: RouteType,
}

impl GetRouteListRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let route_type = RpcParameter::extract_string_enum::<RouteType>("route_type", &params);

        if route_type.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'route_type' parameter",
            ));
        }

        Ok(Self {
            route_type: route_type.unwrap(),
        })
    }

    // getters
    pub fn get_route_type(&self) -> RouteType {
        self.route_type.clone()
    }
}
