pub struct Settings;

impl Settings {
    pub const DEFAULT_OPERATION: &'static str = "help";
    pub const DEFAULT_CONNECTION_TYPE: &'static str = "automatic";
    pub const ROUTER_PATH: &'static str = "../runtime/routing.json";
    pub const SESSION_PATH: &'static str = "../runtime/.session.json";
}
