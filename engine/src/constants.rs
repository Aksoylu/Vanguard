pub struct Constants {}

impl Constants {
    pub const WIN_RUNTIME_PATH: &'static str = "C:\\ProgramData\\Vanguard";
    pub const LINUX_RUNTIME_PATH: &'static str = "/etc/Vanguard";
    pub const OSX_RUNTIME_PATH: &'static str = "Library/Application Support/Vanguard";

    pub const SETTINGS_FILENAME: &'static str = "settings.json";
    pub const SESSION_FILENAME: &'static str = ".session.json";
    pub const ROUTER_FILENAME: &'static str = "routing.json";

    pub const LOG_LEVEL: &'static str = "info";
    pub const LOG_FILE_BASE_NAME: &'static str = "vanguard";
    pub const LOG_TIMESTAMP_FORMAT: &'static str = "%Y-%m-%d_%H-%M-%S";
    pub const LOG_SUFFIX: &'static str = "log";
    pub const LOG_LEVELS: &'static [&'static str] = &["ERROR", "INFO", "WARNING", "DEBUG"];

    pub const DEFAULT_LOG_DIR_NAME: &'static str = "logs";
    pub const DEFAULT_LOG_LEVELS: &'static [&'static str] = &["ERROR", "INFO", "WARNING", "DEBUG"];
    pub const DEFAULT_LOG_FILE_SIZE: u64 = 10_000_000; // 10 MB
    pub const DEFAULT_KEEP_LAST_LOGS: usize = 7;

    pub const DEFAULT_HTTP_IP: &'static str = "0.0.0.0";
    pub const DEFAULT_HTTP_PORT: u16 = 80;

    pub const DEFAULT_HTTPS_IP: &'static str = "0.0.0.0";
    pub const DEFAULT_HTTPS_PORT: u16 = 443;

    pub const DEFAULT_RPC_IP: &'static str = "127.0.0.1";
    pub const DEFAULT_RPC_PORT: u16 = 4242;
    pub const DEFAULT_PRIVATE_KEY: &'static str = "0xVanguard";
}
