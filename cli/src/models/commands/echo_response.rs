use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct EchoResponse {
    code: u16,
    message: String,
}
