use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct IwsRoute {
    pub serving_path: String,
}