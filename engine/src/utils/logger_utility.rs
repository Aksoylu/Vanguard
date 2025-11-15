#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::core::shared_memory::LOGGER.read().unwrap().info(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::core::shared_memory::LOGGER.read().unwrap().error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::core::shared_memory::LOGGER.read().unwrap().warn(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
       $crate::core::shared_memory::LOGGER.read().unwrap().debug(format!($($arg)*))
    };
}
