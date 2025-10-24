use flexi_logger::{opt_format, Age, Cleanup, Criterion, FileSpec, Logger as FlexiLogger, Naming};
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

use crate::models::logger_config::LoggerConfig;

// Global Logger Instance: Initially empty default config, updated in Runtime init
pub static LOGGER: Lazy<Arc<RwLock<LogService>>> = Lazy::new(|| Arc::new(RwLock::new(LogService)));

#[derive(Debug, Clone)]
pub struct LogService;

impl LogService {
    pub fn init(logger_config: LoggerConfig) -> Self {
        FlexiLogger::try_with_str("info")
            .unwrap()
            .log_to_file(
                FileSpec::default()
                    .directory("logs")
                    .basename("vanguard")
                    .suffix("log"),
            )
            .format(|write, now, record| {
                write!(
                    write,
                    "[{}] {:<5}: {}",
                    now.format("%Y-%m-%d %H:%M:%S"),
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
        Self
    }

    pub fn info<T: AsRef<str>>(&self, msg: T) {
        info!("{}", msg.as_ref());
    }
    pub fn warn<T: AsRef<str>>(&self, msg: T) {
        warn!("{}", msg.as_ref());
    }
    pub fn error<T: AsRef<str>>(&self, msg: T) {
        error!("{}", msg.as_ref());
    }
    pub fn debug<T: AsRef<str>>(&self, msg: T) {
        debug!("{}", msg.as_ref());
    }
}
