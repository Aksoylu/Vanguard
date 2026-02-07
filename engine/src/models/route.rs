use serde::{Deserialize, Serialize};


use super::ssl_context::SslContext;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpsRoute {
    pub target: String,
    pub ssl_context: SslContext,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct IwsRoute {
    pub serving_path: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct SecureIwsRoute {
    pub serving_path: String,
    pub ssl_context: SslContext,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct JsonRoute {
    pub protocol: String,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SslContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_path: Option<String>,
}
