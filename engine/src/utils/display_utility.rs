use crate::utils::text_utility::{mask_token, pathbuf_to_string, status_flag, warning_flag};
use colored::Colorize;
use prettytable::{format, row, Table};

use crate::runtime::Runtime;

pub struct RuntimeDisplayUtility<'a> {
    pub runtime_instance: &'a Runtime,
    pub is_config_loaded_successfully: bool,
    pub is_rpc_session_loaded_successfully: bool,
    pub is_router_loaded_successfully: bool,
}

impl<'a> RuntimeDisplayUtility<'a> {
    pub fn new(
        runtime: &'a Runtime,
        is_config_loaded_successfully: bool,
        is_rpc_session_loaded_successfully: bool,
        is_router_loaded_successfully: bool,
    ) -> Self {
        RuntimeDisplayUtility {
            runtime_instance: runtime,
            is_config_loaded_successfully,
            is_rpc_session_loaded_successfully,
            is_router_loaded_successfully,
        }
    }

    fn add_runtime_directory(&self, table: &mut Table) {
        table.add_row(row![
            "Runtime Directory",
            pathbuf_to_string(&self.runtime_instance.runtime_path)
        ]);
    }

    fn add_router_file(&self, table: &mut Table) {
        table.add_row(row![
            "Router File",
            format!(
                "{} {}",
                status_flag(self.is_router_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.runtime_instance.route_path).underline(),
            ),
        ]);
    }

    fn add_config_file(&self, table: &mut Table) {
        table.add_row(row![
            "Config File",
            format!(
                "{} {}",
                status_flag(self.is_config_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.runtime_instance.config_path).underline(),
            ),
        ]);
    }

    fn add_rpc_session_file(&self, table: &mut Table) {
        table.add_row(row![
            "RPC Session File",
            format!(
                "{} {}",
                status_flag(self.is_rpc_session_loaded_successfully, "OK", "Not Loaded"),
                pathbuf_to_string(&self.runtime_instance.rpc_session_path).underline()
            ),
        ]);
    }

    fn add_http_routes(&self, table: &mut Table) {
        let active_http_route_count = &self.runtime_instance.router.get_http_routes().len();
        let is_http_router_active = *active_http_route_count > 0;
        table.add_row(row![
            "HTTP Routes",
            format!(
                "{} Forwarding [{:?}]",
                warning_flag(is_http_router_active, "Forwarding", "Passive"),
                &self.runtime_instance.router.get_http_routes().len()
            )
        ]);
    }

    fn add_integrated_web_server_routes(&self, table: &mut Table) {
        let active_iws_route_count = &self.runtime_instance.router.get_iws_routes().len();
        let is_iws_router_active = *active_iws_route_count > 0;
        table.add_row(row![
            "Integrated Web Server Routes",
            format!(
                "{} Forwarding [{:?}]",
                warning_flag(is_iws_router_active, "Forwarding", "Idle"),
                &self.runtime_instance.router.get_iws_routes().len()
            )
        ]);
    }

    fn add_https_routes(&self, table: &mut Table) {
        let active_https_route_count = &self.runtime_instance.router.get_https_routes().len();
        let is_https_router_active = *active_https_route_count > 0;
        table.add_row(row![
            "HTTPS Routes",
            format!(
                "{} Forwarding [{:?}]",
                warning_flag(is_https_router_active, "Forwarding", "Idle"),
                &self.runtime_instance.router.get_https_routes().len()
            )
        ]);
    }

    fn add_secure_iws_routes(&self, table: &mut Table) {
        let active_secure_iws_route_count =
            &self.runtime_instance.router.get_secure_iws_routes().len();
        let is_secure_iws_router_active = *active_secure_iws_route_count > 0;
        table.add_row(row![
            "Secure Integrated Web Server Routes",
            format!(
                "{} Forwarding [{:?}]",
                warning_flag(is_secure_iws_router_active, "Forwarding", "Idle"),
                &self.runtime_instance.router.get_secure_iws_routes().len()
            )
        ]);
    }

    fn add_jrpc_authentication_token(&self, table: &mut Table) {
        table.add_row(row![
            "JRPC Authentication Token",
            format!("{}", mask_token(&self.runtime_instance.rpc_session.hash))
        ]);
    }

    fn add_jrpc_server(&self, table: &mut Table) {
        let flag = status_flag(
            self.runtime_instance.is_jrpc_server_active,
            "Active",
            "Passive",
        );

        let formatted_endpoint = if self.runtime_instance.is_http_server_active {
            self.runtime_instance
                .config
                .rpc_server
                .get_endpoint()
                .underline()
        } else {
            "".underline()
        };

        let endpoint = &formatted_endpoint;

        table.add_row(row!["JRPC Server", format!("{} on {}", flag, endpoint)]);
    }

    fn add_http_server(&self, table: &mut Table) {
        let flag = status_flag(
            self.runtime_instance.is_http_server_active,
            "Active",
            "Passive",
        );

        let formatted_endpoint = if self.runtime_instance.is_http_server_active {
            self.runtime_instance
                .config
                .http_server
                .get_endpoint()
                .underline()
        } else {
            "".underline()
        };

        let endpoint = &formatted_endpoint;

        table.add_row(row!["HTTP Server", format!("{} on {}", flag, endpoint)]);
    }

    fn add_https_server(&self, table: &mut Table) {
        let flag = status_flag(
            self.runtime_instance.is_https_server_active,
            "Active",
            "Passive",
        );

        let formatted_endpoint = if self.runtime_instance.is_https_server_active {
            self
                .runtime_instance
                .config
                .https_server
                .get_endpoint()
                .underline()
        } else {
            "".underline()
        };

        let endpoint = &formatted_endpoint;

        table.add_row(row!["HTTPS Server", format!("{} on {}", flag, endpoint)]);
    }

    fn add_logger_settings(&self, table: &mut Table) {
        let log_dir_path = self
            .runtime_instance
            .runtime_path
            .join(&self.runtime_instance.config.logger.log_dir_name);

        table.add_row(row![
            "Log Output Path",
            format!("{}", &log_dir_path.to_str().unwrap_or_default().underline(),)
        ]);

        // todo: enhance to show multiple log levels
        table.add_row(row![
            "Log Levels",
            format!(
                "{}",
                &self.runtime_instance.config.logger.log_levels.join(", ")
            )
        ]);

        table.add_row(row![
            "Logger Details",
            format!(
                "Maximum log file size :{}, keeping last {} logs.",
                &self.runtime_instance.config.logger.log_file_size,
                &self.runtime_instance.config.logger.keep_last_logs
            )
        ]);
    }

    pub fn print(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        self.add_runtime_directory(&mut table);
        self.add_config_file(&mut table);
        self.add_router_file(&mut table);
        self.add_rpc_session_file(&mut table);
        self.add_http_routes(&mut table);
        self.add_integrated_web_server_routes(&mut table);
        self.add_https_routes(&mut table);
        self.add_secure_iws_routes(&mut table);
        self.add_jrpc_authentication_token(&mut table);
        self.add_jrpc_server(&mut table);
        self.add_http_server(&mut table);
        self.add_https_server(&mut table);
        self.add_logger_settings(&mut table);

        table.printstd();
    }
}
