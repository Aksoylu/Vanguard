use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger as FlexiLogger, Naming};
use log::{debug, error, info, warn};
use std::fmt::Debug;
use std::path::PathBuf;

use crate::constants::Constants;
use crate::models::logger_config::LoggerConfig;

// Global Logger Instance: Initially empty default config, updated in Runtime init

#[derive(Debug, Clone)]
pub struct LogService {
    pub config: LoggerConfig,
}

impl Default for LogService {
    fn default() -> Self {
        Self {
            config: LoggerConfig::default(),
        }
    }
}

impl LogService {
    pub fn init(runtime_path: &PathBuf, logger_config: LoggerConfig) -> Self {
        let log_dir_path = runtime_path.join(&logger_config.log_dir_name);

        FlexiLogger::try_with_str(Constants::LOG_LEVEL)
            .unwrap()
            .log_to_file(
                FileSpec::default()
                    .directory(log_dir_path)
                    .basename(Constants::LOG_FILE_BASE_NAME)
                    .suffix(Constants::LOG_SUFFIX),
            )
            .format(|write, now, record| {
                write!(
                    write,
                    "[{}] {:<5}: {}",
                    now.format(Constants::LOG_TIMESTAMP_FORMAT),
                    record.level(),
                    record.args()
                )
            })
            .rotate(
                Criterion::AgeOrSize(Age::Day, logger_config.log_file_size),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(logger_config.keep_last_logs),
            )
            .start()
            .unwrap();
        Self {
            config: logger_config,
        }
    }

    pub fn info<T: AsRef<str>>(&self, msg: T) {
        if self.config.log_levels.contains(&"INFO".to_string()) {
            info!("{}", msg.as_ref());
        }
    }
    pub fn warn<T: AsRef<str>>(&self, msg: T) {
        if self.config.log_levels.contains(&"WARNING".to_string()) {
            warn!("{}", msg.as_ref());
        }
    }
    pub fn error<T: AsRef<str>>(&self, msg: T) {
        if self.config.log_levels.contains(&"ERROR".to_string()) {
            error!("{}", msg.as_ref());
        }
    }
    pub fn debug<T: AsRef<str>>(&self, msg: T) {
        if self.config.log_levels.contains(&"DEBUG".to_string()) {
            debug!("{}", msg.as_ref());
        }
    }
}
