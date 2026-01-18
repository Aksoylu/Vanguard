use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct DeleteSecureIwsRouteResponse {
    code: u16,
    message: String,
}
