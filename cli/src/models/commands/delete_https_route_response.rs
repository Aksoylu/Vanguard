use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteHttpsRouteResponse {
    pub code: u16,
    pub message: String,
}
