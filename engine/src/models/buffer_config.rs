use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BufferConfig {
    Auto(String), // "auto"
    Fixed(usize),  // integer value
}

impl Default for BufferConfig {
    fn default() -> Self {
        BufferConfig::Auto("auto".to_string())
    }
}

impl BufferConfig {
    pub fn get_size(&self, default_fixed: usize) -> usize {
        match self {
            BufferConfig::Auto(_) => default_fixed,
            BufferConfig::Fixed(size) => *size,
        }
    }

    pub fn is_auto(&self) -> bool {
        matches!(self, BufferConfig::Auto(_))
    }
}
