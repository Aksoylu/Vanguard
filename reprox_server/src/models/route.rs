use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Route
{
    pub source: String,
    pub target: String,
}