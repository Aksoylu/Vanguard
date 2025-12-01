use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}
