extern crate prettytable;
use colored::Colorize;
use prettytable::{format, row, Table};
use std::path::PathBuf;

use crate::core::log_service::LOGGER;
use crate::utils::file_utility::save_json;
use crate::utils::text_utility::mask_token;
use crate::{
    constants::Constants,
    core::{log_service::LogService, router::Router},
    models::{config::Config, rpc_session::RpcSession},
    utils::{
        crypt_utility::generate_hash,
        directory_utility::get_runtime_path,
        file_utility::load_json,
        text_utility::{get_flag, pathbuf_to_string},
    },
};
use crate::{fixed_row, log_error, log_info};

use tokio::sync::watch;

pub struct Runtime {
    pub config: Config,
    pub rpc_session: RpcSession,
    pub router: Router,

    config_path: PathBuf,
    is_config_loaded_successfully: bool,

    rpc_session_path: PathBuf,
    is_rpc_session_loaded_successfully: bool,

    route_path: PathBuf,
    is_router_loaded_successfully: bool,
}

impl Runtime {
    pub fn init() -> Self {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Runtime::get_rpc_session_path(&runtime_path);

        let config_path = Runtime::get_config_path(&runtime_path);

        let (config, is_config_loaded_successfully) = Runtime::load_config(config_path.clone());

        let mut logger = LOGGER.write().unwrap();
        *logger = LogService::init(config.logger.clone());

        let (rpc_session, is_rpc_session_loaded_successfully) =
            Runtime::load_rpc_session(rpc_session_path.clone());

        let route_path = Runtime::get_route_path(&runtime_path);
        let (router, is_router_loaded_successfully) = Router::load(route_path.clone());

        if !is_config_loaded_successfully {
            Runtime::save_config(config_path.clone(), &config);
        }

        if !is_rpc_session_loaded_successfully {
            Runtime::save_rpc_session(rpc_session_path.clone(), &rpc_session);
        }

        if !is_router_loaded_successfully {
            Runtime::save_router(route_path.clone(), &router);
        }

        let instance = Runtime {
            config,
            rpc_session,
            router,

            config_path,
            is_config_loaded_successfully,

            rpc_session_path,
            is_rpc_session_loaded_successfully,

            route_path,
            is_router_loaded_successfully,
        };

        instance.print();

        instance
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

    fn load_rpc_session(rpc_session_path: PathBuf) -> (RpcSession, bool) {
        let read_rpc_session_operation = load_json::<RpcSession>(&rpc_session_path);
        if read_rpc_session_operation.is_err() {
            return (RpcSession::default(), false);
        }

        let mut rpc_session = read_rpc_session_operation.unwrap();
        rpc_session.hash = generate_hash(rpc_session.private_key.clone());

        let validation = rpc_session.clone().validate();

        if validation.is_ok() {
            return (rpc_session, true);
        } else {
            return (RpcSession::default(), false);
        }
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

    fn print(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table.add_row(row![
            "Runtime Directory",
            pathbuf_to_string(&get_runtime_path())
        ]);

        table.add_row(row![
            "Logger Settings",
            fixed_row!(
                "80",
                "Logs path : '{}' with maximum file size :{}. Keeping last {} logs.",
                &self.config.logger.log_dir_path,
                &self.config.logger.log_file_size,
                &self.config.logger.keep_last_logs
            )
        ]);

        table.add_row(row![
            "Router File",
            fixed_row!(
                "80",
                "{} {}",
                get_flag(self.is_router_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.route_path).underline(),
            ),
        ]);

        table.add_row(row![
            "Config File",
            fixed_row!(
                "80",
                "{} {}",
                get_flag(self.is_config_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.config_path).underline()
            ),
        ]);

        table.add_row(row![
            "RPC Session File",
            fixed_row!(
                "80",
                "{} {}",
                get_flag(self.is_rpc_session_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.rpc_session_path).underline()
            ),
        ]);

        table.add_row(row![
            "HTTP Routes",
            fixed_row!(
                "80",
                "Forwarding [{:?}]",
                &self.router.get_http_routes().len()
            )
        ]);

        table.add_row(row![
            "Integrated Web Server Routes",
            fixed_row!("80", "Serving [{:?}]", &self.router.get_iws_routes().len())
        ]);

        let http_route_len= &self.router.get_https_routes().len();
        let http_router_active = *http_route_len > 0;

        table.add_row(row![
            "HTTPS Routes",
            fixed_row!(
                "80",
                "{} [{:?}]",
                get_flag(http_router_active, "Forwarding", "Passive"),
                &http_route_len
            )
        ]);

        table.add_row(row![
            "Secure Integrated Web Server Routes",
            fixed_row!(
                "80",
                "Serving [{:?}]",
                &self.router.get_secure_iws_routes().len()
            )
        ]);

        table.add_row(row![
            "JRPC Authentication Token",
            fixed_row!("80", "{}", mask_token(&self.rpc_session.hash))
        ]);

        table.add_row(row![
            "JRPC Server",
            fixed_row!(
                "80",
                "{} on {}",
                "[Active]".green(),
                &self.config.rpc_server.get_endpoint().underline()
            )
        ]);

        table.add_row(row![
            "HTTP Server",
            fixed_row!(
                "80",
                "{} on {}",
                "[Active]".green(),
                &self.config.http_server.get_endpoint().underline()
            )
        ]);

        table.add_row(row![
            "HTTPS Server",
            fixed_row!(
                "80",
                "{} on {}",
                "[Active]".green(),
                &self.config.https_server.get_endpoint().underline()
            )
        ]);

        table.printstd();
    }
}
