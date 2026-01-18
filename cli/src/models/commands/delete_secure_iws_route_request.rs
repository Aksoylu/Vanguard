use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DeleteSecureIwsRouteRequest {
    pub source: String,
}
