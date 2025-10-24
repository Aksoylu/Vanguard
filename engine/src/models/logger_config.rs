use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct LoggerConfig {
    #[serde(default = "default_log_dir_path")]
    pub log_dir_path: String,

    #[serde(default = "default_log_levels")]
    pub log_levels: Vec<String>,

    #[serde(default = "default_log_file_size")]
    pub log_file_size: u64,

    #[serde(default = "default_keep_last_logs")]
    pub keep_last_logs: usize,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_dir_path: default_log_dir_path(),
            log_levels: default_log_levels(),
            log_file_size: default_log_file_size(),
            keep_last_logs: default_keep_last_logs(),
        }
    }
}

fn default_log_dir_path() -> String {
    "logs".to_string()
}

fn default_log_levels() -> Vec<String> {
    Constants::DEFAULT_LOG_LEVELS
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