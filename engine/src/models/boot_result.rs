extern crate prettytable;
use std::path::PathBuf;

use crate::models::config::Config;

pub struct BootResult {
    pub config: Config,

    pub runtime_path: PathBuf,
    pub config_path: PathBuf,
    pub rpc_session_path: PathBuf,
    pub route_path: PathBuf,

    pub is_config_loaded_successfully: bool,
    pub is_router_loaded_successfully: bool,
}
