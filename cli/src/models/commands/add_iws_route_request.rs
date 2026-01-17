use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddIwsRouteRequest {
    pub source: String,
    pub serving_path: String,
}
