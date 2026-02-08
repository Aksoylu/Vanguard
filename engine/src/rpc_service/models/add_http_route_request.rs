use crate::{models::traffic_policy::scope_traffic_policy::ScopeTrafficPolicy, rpc_service::rpc_error::RPCError, utils::rpc_utility::RpcParameter};
use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct AddHttpRouteRequest {
    source: String,
    target: String,
    traffic_policy: Option<ScopeTrafficPolicy>,
}

impl AddHttpRouteRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let source = RpcParameter::extract_string("source", &params);
        let target = RpcParameter::extract_string("target", &params);
        let traffic_policy = RpcParameter::extract_traffic_policy("traffic_policy", &params);

        if source.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'Source' parameter",
            ));
        }

        if target.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'Target' parameter",
            ));
        }

        Ok(Self {
            source: source.unwrap(),
            target: target.unwrap(),
            traffic_policy,
        })
    }

    // getters
    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }

    pub fn get_traffic_policy(&self) -> Option<ScopeTrafficPolicy> {
        self.traffic_policy.clone()
    }
}
