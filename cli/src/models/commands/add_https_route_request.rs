use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddHttpsRouteRequest {
    pub source: String,
    pub target: String,
    pub ssl_cert_path: String,
    pub ssl_private_key_path: String,
}
