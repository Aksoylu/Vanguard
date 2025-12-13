use crate::models::route::{HttpRoute, HttpsRoute, IwsRoute, SecureIwsRoute};
use crate::rpc_service::rpc_status_message::RpcStatusMessage;
use hyper::StatusCode;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct GetRouteListResponse {
    pub code: u16,
    pub message: String,
    pub http_routes: Option<HashMap<String, HttpRoute>>,
    pub https_routes: Option<HashMap<String, HttpsRoute>>,
    pub iws_routes: Option<HashMap<String, IwsRoute>>,
    pub secure_iws_routes: Option<HashMap<String, SecureIwsRoute>>,
}

impl GetRouteListResponse {
    pub fn new() -> GetRouteListResponse {
        let instance = GetRouteListResponse {
            code: StatusCode::OK.as_u16(),
            message: RpcStatusMessage::OK.to_string(),
            http_routes: None,
            https_routes: None,
            iws_routes: None,
            secure_iws_routes: None,
        };

        return instance
    }

     pub fn build(&self) -> jsonrpc_core::Value {
        let serialized_json = match serde_json::to_string(&self) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
