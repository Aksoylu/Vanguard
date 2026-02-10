use serde::{Deserialize, Deserializer, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LoggerSettings {
    #[serde(default = "default_log_dir_name")]
    pub log_dir_name: String,

    #[serde(default = "default_log_levels")]
    #[serde(deserialize_with = "deserialize_log_levels")]
    pub log_levels: Vec<String>,

    #[serde(default = "default_log_file_size")]
    pub log_file_size: u64,

    #[serde(default = "default_keep_last_logs")]
    pub keep_last_logs: usize,
}

impl Default for LoggerSettings {
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
    Constants::DEFAULT_LOG_LEVELS
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn default_log_file_size() -> u64 {
    Constants::DEFAULT_LOG_FILE_SIZE
}

fn default_keep_last_logs() -> usize {
    Constants::DEFAULT_KEEP_LAST_LOGS
}

/// Custom deserializer for log levels to ensure only valid levels are accepted
fn deserialize_log_levels<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let readed_log_levels: Vec<String> = Deserialize::deserialize(deserializer)?;

    let mut log_levels = Vec::new();
    for each_readed_log_level in readed_log_levels {
        let is_valid_log_level = Constants::LOG_LEVELS
            .iter()
            .any(|&valid_level| valid_level.eq_ignore_ascii_case(&each_readed_log_level));

        if is_valid_log_level {
            log_levels.push(each_readed_log_level);
        }
    }

    Ok(log_levels.into_iter().map(|s| s.to_uppercase()).collect())
}
