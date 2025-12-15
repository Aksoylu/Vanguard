extern crate prettytable;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{core::router::Router, models::config::Config};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootResult {
    pub config: Config,
    pub router: Router,

    pub runtime_path: PathBuf,
    pub config_path: PathBuf,
    pub rpc_session_path: PathBuf,
    pub route_path: PathBuf,

    pub is_config_loaded_successfully: bool,
    pub is_router_loaded_successfully: bool,
}

impl Default for BootResult {
    fn default() -> Self {
        Self {
            config: Config::default(),
            router: Router::default(),
            runtime_path: PathBuf::default(),
            config_path: PathBuf::default(),
            rpc_session_path: PathBuf::default(),
            route_path: PathBuf::default(),
            is_config_loaded_successfully: false,
            is_router_loaded_successfully: false,
        }
    }
}
