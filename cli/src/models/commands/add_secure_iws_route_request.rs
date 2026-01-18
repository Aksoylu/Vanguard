use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddSecureIwsRouteRequest {
    pub source: String,
    pub serving_path: String,
    pub ssl_cert_path: String,
    pub ssl_private_key_path: String,
}
