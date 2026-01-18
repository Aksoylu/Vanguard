use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddHttpsRouteResponse {
    pub code: u16,
    pub message: String,
    pub data: Option<Value>,
}
