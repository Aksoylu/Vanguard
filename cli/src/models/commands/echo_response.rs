use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    pub code: u16,
    pub message: String,
}
