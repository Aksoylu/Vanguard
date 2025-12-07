use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddHttpRouteResponse {
    pub code: i64,
    pub message: String,
}
