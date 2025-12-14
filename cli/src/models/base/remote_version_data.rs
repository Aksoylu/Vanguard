use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct RemoteVersionData{
    #[serde(rename = "VERSION_NUMBER")]
    pub version_number: f64,
    #[serde(rename = "VERSION_NAME")]
    pub version_name: String
}