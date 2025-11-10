extern crate prettytable;
use std::path::PathBuf;

use crate::core::log_service::LOGGER;
use crate::core::rpc_session::{RPC_SESSION, RpcSession};
use crate::utils::display_utility::RuntimeDisplayUtility;
use crate::utils::file_utility::save_json;
use crate::{
    constants::Constants,
    core::{log_service::LogService, router::Router},
    models::config::Config,
    utils::{directory_utility::get_runtime_path, file_utility::load_json},
};

use tokio::sync::watch;

pub struct Runtime {
    pub config: Config,
    pub router: Router,

    pub runtime_path: PathBuf,
    pub config_path: PathBuf,
    pub rpc_session_path: PathBuf,
    pub route_path: PathBuf,

    pub is_jrpc_server_active: bool,
    pub is_http_server_active: bool,
    pub is_https_server_active: bool,
}

impl Runtime {
    pub fn init() -> Self {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Runtime::get_rpc_session_path(&runtime_path);

        let config_path = Runtime::get_config_path(&runtime_path);

        let (config, is_config_loaded_successfully) = Runtime::load_config(config_path.clone());

        let mut logger = LOGGER.write().unwrap();
        *logger = LogService::init(&runtime_path, config.logger.clone());

        let mut rpc_session = RPC_SESSION.write().unwrap();
        *rpc_session = RpcSession::init(
            config.rpc_server.ip_address.clone(),
            config.rpc_server.port.clone(),
            config.rpc_server.private_secret_key.clone(),
        );

        Runtime::save_rpc_session(rpc_session_path.clone(), &rpc_session);

        let route_path = Runtime::get_route_path(&runtime_path);
        let (router, is_router_loaded_successfully) = Router::load(route_path.clone());

        if !is_config_loaded_successfully {
            Runtime::save_config(config_path.clone(), &config);
        }

        if !is_router_loaded_successfully {
            Runtime::save_router(route_path.clone(), &router);
        }

        Runtime {
            config,
            router,

            runtime_path,
            config_path,
            rpc_session_path,
            route_path,

            is_jrpc_server_active: true,
            is_http_server_active: true,
            is_https_server_active: true,
        }
    }

    pub fn print(&self) {
        let runtime_display: RuntimeDisplayUtility =
            RuntimeDisplayUtility::new(self, true, true, true);

        runtime_display.print();
    }

    pub fn update_config(&mut self, new_config: Config, runtime_sub: watch::Sender<()>) {
        self.config = new_config;
        let _ = runtime_sub.send(());
    }

    fn load_config(config_path: PathBuf) -> (Config, bool) {
        let read_config_operation = load_json::<Config>(&config_path);
        if read_config_operation.is_err() {
            return (Config::default(), false);
        }

        let config = read_config_operation.unwrap();
        let validation = config.validate();

        if validation.is_ok() {
            return (config, true);
        } else {
            let error_text = validation.err().unwrap_or_default();
            return (Config::default(), false);
        }
    }

    fn save_config(config_path: PathBuf, config: &Config) -> bool {
        let write_operation = save_json::<Config>(&config_path, config);

        write_operation.is_ok()
    }

    fn save_rpc_session(rpc_session_path: PathBuf, rpc_session: &RpcSession) -> bool {
        let write_operation = save_json::<RpcSession>(&rpc_session_path, rpc_session);

        write_operation.is_ok()
    }

    fn save_router(router_path: PathBuf, router: &Router) -> bool {
        let write_operation = save_json::<Router>(&router_path, router);

        write_operation.is_ok()
    }

    fn get_config_path(runtime_path: &PathBuf) -> PathBuf {
        let mut config_path = runtime_path.clone();
        config_path.push(Constants::SETTINGS_FILENAME);
        config_path
    }

    fn get_route_path(runtime_path: &PathBuf) -> PathBuf {
        let mut runtime_path = runtime_path.clone();
        runtime_path.push(Constants::ROUTER_FILENAME);
        runtime_path
    }

    fn get_rpc_session_path(runtime_path: &PathBuf) -> PathBuf {
        let mut session_path = runtime_path.clone();
        session_path.push(Constants::SESSION_FILENAME);
        session_path
    }
}
