use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBuildVersionResponse {
    pub code: u16,
    pub build_version_number: f64,
    pub build_version_name: String
}