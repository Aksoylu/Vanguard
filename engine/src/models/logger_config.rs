use serde::{Deserialize, Deserializer, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LoggerConfig {
    #[serde(default = "default_log_dir_name")]
    pub log_dir_name: String,

    #[serde(default = "default_log_levels", deserialize_with = "deserialize_log_levels")]
    pub log_levels: Vec<String>,

    #[serde(default = "default_log_file_size")]
    pub log_file_size: u64,

    #[serde(default = "default_keep_last_logs")]
    pub keep_last_logs: usize,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_dir_name: default_log_dir_name(),
            log_levels: default_log_levels(),
            log_file_size: default_log_file_size(),
            keep_last_logs: default_keep_last_logs(),
        }
    }
}

fn default_log_dir_name() -> String {
    Constants::DEFAULT_LOG_DIR_NAME.to_string()
}

fn default_log_levels() -> Vec<String> {
    Constants::DEFAULT_LOG_LEVELS_AS_STR
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn default_log_file_size() -> u64 {
    Constants::DEFAULT_LOG_FILE_SIZE
}

fn default_keep_last_logs() -> usize {
    Constants::DEFAULT_KEEP_LAST_LOGS
}

fn deserialize_log_levels<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let levels: Vec<String> = Deserialize::deserialize(deserializer)?;
    Ok(levels.into_iter().map(|s| s.to_uppercase()).collect())
}
