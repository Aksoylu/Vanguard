extern crate prettytable;
use std::path::PathBuf;

use crate::core::http_server::HttpServer;
use crate::core::https_server::HttpsServer;
use crate::core::rpc_session::RpcSession;
use crate::core::shared_memory::{HTTP_SERVER, HTTPS_SERVER, LOGGER, ROUTER, RPC_SERVER, RUNTIME_BOOT_INFO};
use crate::models::boot_result::BootResult;
use crate::rpc_service::rpc_server::RPCServer;
use crate::utils::file_utility::save_json;
use crate::{
    constants::Constants,
    core::{log_service::LogService, router::Router},
    models::config::Config,
    utils::{directory_utility::get_runtime_path, file_utility::load_json},
};

use log::error;

pub struct Boot {}

impl Boot {
    pub fn init() -> BootResult {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Self::get_rpc_session_path(&runtime_path);

        let config_path = Self::get_config_path(&runtime_path);
        let (config, is_config_loaded_successfully) = Self::load_config(config_path.clone());
        if !is_config_loaded_successfully {
            error!("Failed to load configuration. Using default settings.");
        }

        let route_path = Self::get_route_path(&runtime_path);
        let (loaded_router, is_router_loaded_successfully) = Router::load(route_path.clone());
        if is_router_loaded_successfully {
            let mut router = ROUTER.write().unwrap();
            *router = loaded_router.clone();
        } else {
            error!("Failed to load router data. Initializing default empty router.");
        }

        let mut http_server = HTTP_SERVER.write().unwrap();
        *http_server = HttpServer::init(
            config.http_server.ip_address.clone(),
            config.http_server.port,
        );

        let mut https_server = HTTPS_SERVER.write().unwrap();
        *https_server = HttpsServer::init(
            config.https_server.ip_address.clone(),
            config.https_server.port,
        );

        let mut logger = LOGGER.write().unwrap();
        *logger = LogService::init(&runtime_path, config.logger.clone());

        let rpc_session = RpcSession::init(
            config.rpc_server.ip_address.clone(),
            config.rpc_server.port,
            config.rpc_server.private_secret_key.clone(),
        );
        let mut rpc_server = RPC_SERVER.write().unwrap();
        *rpc_server = RPCServer::init(rpc_session.clone());

        Self::save_rpc_session(rpc_session_path.clone(), &rpc_session);

        let boot_info = BootResult {
            config,
            router: loaded_router,
            runtime_path,
            config_path,
            rpc_session_path,
            route_path,
            is_config_loaded_successfully,
            is_router_loaded_successfully,
        };

        let mut runtime_boot_info = RUNTIME_BOOT_INFO.write().unwrap();
        *runtime_boot_info = boot_info.clone();

        boot_info
    }

    pub fn save_config(config_path: PathBuf, config: &Config) -> bool {
        let write_operation = save_json::<Config>(&config_path, config);

        write_operation.is_ok()
    }

    pub fn save_router(router_path: PathBuf, router: &Router) -> bool {
        let write_operation = save_json::<Router>(&router_path, router);

        write_operation.is_ok()
    }

    fn load_config(config_path: PathBuf) -> (Config, bool) {
        let read_config_operation = load_json::<Config>(&config_path);
        if read_config_operation.is_err() {
            return (Config::default(), false);
        }

        let config = read_config_operation.unwrap();
        let validation = config.validate();

        if validation.is_ok() {
            (config, true)
        } else {
            (Config::default(), false)
        }
    }

    fn save_rpc_session(rpc_session_path: PathBuf, rpc_session: &RpcSession) -> bool {
        let write_operation = save_json::<RpcSession>(&rpc_session_path, rpc_session);

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
