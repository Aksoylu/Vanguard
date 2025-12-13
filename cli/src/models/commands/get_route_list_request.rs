use serde::Deserialize;
use serde::Serialize;

use crate::common::enums::route_type::RouteType;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRouteListRequest {
    pub route_type: RouteType
}
