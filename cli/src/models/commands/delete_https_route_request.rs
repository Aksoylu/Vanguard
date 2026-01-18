use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DeleteHttpsRouteRequest {
    pub source: String,
}
