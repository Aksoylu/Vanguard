use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DeleteIwsRouteRequest {
    pub source: String,
}
