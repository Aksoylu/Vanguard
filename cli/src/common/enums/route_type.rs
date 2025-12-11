use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RouteType {
    All,
    Http,
    Https,
    Iws,
    SecureIws,
}

impl FromStr for RouteType {
    type Err = &'static str; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.to_lowercase().replace("-", ""); 

        match normalized.as_str() {
            "all" => Ok(RouteType::All),
            "http" => Ok(RouteType::Http),
            "https" => Ok(RouteType::Https),
            "iws" => Ok(RouteType::Iws),
            "secureiws" => Ok(RouteType::SecureIws),
            _ => Err("Not accepted route type. Please type one of following: 'all', 'http', 'https', 'iws', 'secure-iws'"),
        }
    }
}