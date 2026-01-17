use std::{fmt, str::FromStr};

use colored::Colorize;

pub enum LogLevel {
    Error,
    Info,
    Warning,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let label = match self {
            LogLevel::Error => "[ERROR]".red(),
            LogLevel::Info => "[INFO]".blue(),
            LogLevel::Warning => "[WARNING]".yellow(),
            LogLevel::Debug => "[DEBUG]".green(),
        };
        write!(f, "{}", label)
    }
}

impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ERROR" => Ok(LogLevel::Error),
            "INFO" => Ok(LogLevel::Info),
            "WARNING" => Ok(LogLevel::Warning),
            "DEBUG" => Ok(LogLevel::Debug),
            _ => Err(format!("'{}' is not a valid log level", s)),
        }
    }
}