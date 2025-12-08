use serde::Deserialize;
use serde::Serialize;

use crate::models::entity::json_route::JsonRoute;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetHttpRouteListResponse {
    pub code: u16,
    pub message: String,
    pub data: Vec<JsonRoute>,
}
