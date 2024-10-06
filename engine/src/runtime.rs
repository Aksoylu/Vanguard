extern crate prettytable;
use colored::Colorize;
use prettytable::{format, row, Table};

use std::path::PathBuf;

use crate::{
    constants::Constants,
    core::router::Router,
    models::{config::Config, rpc_session::RpcSession},
    utils::{
        crypt_utility::generate_hash,
        directory_utility::get_runtime_path,
        file_utility::load_json,
        text_utility::{get_flag, pathbuf_to_string},
    },
};

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
    is_routes_loaded_successfully: bool,
}

impl Runtime {
    pub fn init() -> Self {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Runtime::get_rpc_session_path(&runtime_path);

        let config_path = Runtime::get_config_path(&runtime_path);

        let config = Runtime::load_config(config_path.clone());
        let is_config_loaded_successfully = config != Config::default();

        let rpc_session = Runtime::load_rpc_session(rpc_session_path.clone());
        let is_rpc_session_loaded_successfully = rpc_session != RpcSession::default();

        let route_path = Runtime::get_route_path(&runtime_path);
        let router = Router::load(route_path.clone());
        let is_routes_loaded_successfully = router != Router::default();

        if !is_config_loaded_successfully {
            // TODO: WRITE FILE BACK
        }

        if !is_rpc_session_loaded_successfully {
            // TODO: WRITE FILE BACK
        }

        if !is_routes_loaded_successfully {
            // TODO: WRITE FILE BACK
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
            is_routes_loaded_successfully,
        };

        instance.print();

        instance
    }

    pub fn update_config(&mut self, new_config: Config, runtime_sub: watch::Sender<()>) {
        self.config = new_config;
        let _ = runtime_sub.send(());
    }

    fn load_config(config_path: PathBuf) -> Config {
        let read_config_operation = load_json::<Config>(&config_path);
        if read_config_operation.is_err() {
            return Config::default();
        }

        let config = read_config_operation.unwrap();
        let validation = config.validate();

        if validation.is_ok() {
            return config;
        } else {
            let error_text = validation.err().unwrap_or_default();
            eprintln!("Invalid configuration: {}", error_text);
            return Config::default();
        }
    }

    fn load_rpc_session(rpc_session_path: PathBuf) -> RpcSession {
        let read_rpc_session_operation = load_json::<RpcSession>(&rpc_session_path);
        if read_rpc_session_operation.is_err() {
            return RpcSession::default();
        }

        let mut rpc_session = read_rpc_session_operation.unwrap();
        rpc_session.hash = generate_hash(rpc_session.private_key.clone());

        let validation = rpc_session.clone().validate();

        if validation.is_ok() {
            return rpc_session;
        } else {
            let error_text = validation.err().unwrap_or_default();
            eprintln!("Invalid Rpc Session: {}\nUsing default", error_text);
            return RpcSession::default();
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
            "Router File",
            format!(
                "{} {}",
                get_flag(self.is_routes_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.route_path).underline()
            ),
        ]);

        table.add_row(row![
            "Config File",
            format!(
                "{} {}",
                get_flag(self.is_config_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.config_path).underline()
            ),
        ]);

        table.add_row(row![
            "RPC Session File",
            format!(
                "{} {}",
                get_flag(self.is_rpc_session_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.rpc_session_path).underline()
            ),
        ]);

        table.add_row(row![
            "HTTP Routes",
            format!("Forwarding [{:?}]", &self.router.get_http_routes().len())
        ]);
        table.add_row(row![
            "Integrated Web Server Routes",
            format!("Serving [{:?}]", &self.router.get_iws_routes().len())
        ]);
        table.add_row(row![
            "HTTPS Routes",
            format!("Forwarding [{:?}]", &self.router.get_https_routes().len())
        ]);

        table.add_row(row![
            "Secure Integrated Web Server Routes",
            format!("Serving [{:?}]", &self.router.get_secure_iws_routes().len())
        ]);
        
        table.add_row(row![
            "JRPC Authentication Token",
            &self.rpc_session.hash.underline()
        ]);

        table.add_row(row![
            "JRPC Server",
            format!(
                "{} on {}",
                "[Active]".green(),
                &self.config.rpc_server.get_endpoint().underline()
            )
        ]);

        table.add_row(row![
            "HTTP Server",
            format!(
                "{} on {}",
                "[Active]".green(),
                &self.config.http_server.get_endpoint().underline()
            )
        ]);

        table.add_row(row![
            "HTTPS Server",
            format!(
                "{} on {}",
                "[Active]".green(),
                &self.config.https_server.get_endpoint().underline()
            )
        ]);

        table.printstd();
    }
}
