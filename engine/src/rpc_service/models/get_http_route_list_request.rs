use crate::models::route::JsonRoute;
use jsonrpc_core::Value;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct GetHttpRouteListRequest {
    code: i32,
    message: String,
    data: Vec<JsonRoute>,
}
