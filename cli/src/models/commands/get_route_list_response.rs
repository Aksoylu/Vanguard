use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::models::entity::http_route::HttpRoute;
use crate::models::entity::https_route::HttpsRoute;
use crate::models::entity::iws_route::IwsRoute;
use crate::models::entity::secure_iws_route::SecureIwsRoute;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRouteListResponse {
    pub code: u16,
    pub message: String,
    pub http_routes: Option<HashMap<String, HttpRoute>>,
    pub https_routes: Option<HashMap<String, HttpsRoute>>,
    pub iws_routes: Option<HashMap<String, IwsRoute>>,
    pub secure_iws_routes: Option<HashMap<String, SecureIwsRoute>>,
}
