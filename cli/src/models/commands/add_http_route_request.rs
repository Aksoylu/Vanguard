use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct AddHttpRouteRequest {
    pub source: String,
    pub target: String,
}
