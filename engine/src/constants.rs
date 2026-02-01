pub struct Constants {}

impl Constants {
    pub const VERSION_NUMBER: f64 = 1.0;
    pub const VERSION_NAME: &'static str = "v1.0 Beta";
    pub const VERSION_CONTROL_URL: &'static str =
        "https://raw.githubusercontent.com/Aksoylu/Vanguard/refs/heads/main/Version";

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

    pub const DEFUALT_HTTPS_IS_ACTIVE: bool = true;
    pub const DEFAULT_HTTP_IP: &'static str = "0.0.0.0";
    pub const DEFAULT_HTTP_PORT: u16 = 80;

    pub const DEFUALT_HTTP_IS_ACTIVE: bool = true;
    pub const DEFAULT_HTTPS_IP: &'static str = "0.0.0.0";
    pub const DEFAULT_HTTPS_PORT: u16 = 443;

    pub const DEFUALT_RPC_IS_ACTIVE: bool = true;
    pub const DEFAULT_RPC_IP: &'static str = "127.0.0.1";
    pub const DEFAULT_RPC_PORT: u16 = 4242;
    pub const DEFAULT_PRIVATE_SECRET_KEY: &'static str = "0xVanguard";

    // Only Global Scalability & Performance Settings
    pub const DEFAULT_HTTP1_HEADER_READ_TIMEOUT: u64 = 10; // 10 seconds
    pub const DEFAULT_MAXIMUM_TOTAL_CONNECTIONS: u64 = 10000; // 10k connections in same time
    pub const DEFAULT_SERVER_READ_TIMEOUT: u64 = 30; // 30 seconds

    // All Scalability & Performance Settings
    pub const DEFAULT_HTTP_CLIENT_TIMEOUT: u64 = 30; // 30 seconds
    pub const DEFAULT_POOL_IDLE_TIMEOUT: u64 = 60; // 60 seconds
    pub const DEFAULT_MAX_IDLE_CONNS_PER_HOST: usize = 100;
    pub const DEFAULT_SERVER_WRITE_TIMEOUT: u64 = 30; // 30 seconds
    pub const DEFAULT_MAX_REQUEST_BODY_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
    pub const DEFAULT_MAX_REQUESTS_PER_MINUTE: u32 = 120; // 2 RPS on average
}
