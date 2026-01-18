use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSecureIwsRouteResponse {
    pub code: u16,
    pub message: String,
}
