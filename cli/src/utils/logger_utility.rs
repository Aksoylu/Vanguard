pub const ANSI_COLOR_RED: &str = "\x1b[31m";
pub const ANSI_COLOR_YELLOW: &str = "\x1b[33m";
pub const ANSI_COLOR_RESET: &str = "\x1b[0m";

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        eprintln!(
            "{}[ERROR] {}{}", 
            $crate::utils::logger_utility::ANSI_COLOR_RED,
            format!($($arg)*), 
            $crate::utils::logger_utility::ANSI_COLOR_RESET
        )
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        eprintln!(
            "{}[WARNING] {}{}", 
            $crate::utils::logger_utility::ANSI_COLOR_YELLOW, 
            format!($($arg)*), 
            $crate::utils::logger_utility::ANSI_COLOR_RESET
        )
    };
}
