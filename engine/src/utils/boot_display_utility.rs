use crate::{
    constants::Constants,
    core::shared_memory::{ROUTER, RPC_SERVER},
    models::boot_result::BootResult,
    utils::text_utility::{mask_token, pathbuf_to_string, status_flag, warning_flag},
};
use colored::Colorize;
use prettytable::{Table, format::{self, TableFormat}, row};

pub struct BootDisplayUtility {
    boot_result: BootResult,
}

impl BootDisplayUtility {
    pub fn init(boot_result: BootResult) -> Self {
        BootDisplayUtility {
            boot_result: boot_result,
        }
    }

    fn table_format(&self) -> TableFormat {
        let format = format::FormatBuilder::new()
            .column_separator(' ')
            .left_border('\0')
            .right_border('\0')
            .padding(2, 1)
            .separators(
                &[format::LinePosition::Top],
                format::LineSeparator::new('─', '┬', '┌', '┐'),
            )
            .separators(
                &[format::LinePosition::Bottom],
                format::LineSeparator::new('─', '┴', '└', '┘'),
            )
            .separators(
                &[format::LinePosition::Intern],
                format::LineSeparator::new('─', '┼', '├', '┤'),
            )
            .build();

        format
    }

    fn add_build_info(&self, table: &mut Table) {
        table.add_row(row![
            "Vanguard Version",
            format!("Main Build {} ", Constants::VERSION_NAME)
        ]);
    }

    fn add_runtime_directory(&self, table: &mut Table) {
        table.add_row(row![
            "Runtime Directory",
            pathbuf_to_string(&self.boot_result.runtime_path)
        ]);
    }

    fn add_router_file(&self, table: &mut Table) {
        table.add_row(row![
            "Router File",
            format!(
                "{} {}",
                status_flag(
                    self.boot_result.is_router_loaded_successfully,
                    "OK",
                    "Not Loaded"
                ),
                pathbuf_to_string(&self.boot_result.route_path).underline(),
            ),
        ]);
    }

    fn add_config_file(&self, table: &mut Table) {
        table.add_row(row![
            "Config File",
            format!(
                "{} {}",
                status_flag(
                    self.boot_result.is_config_loaded_successfully,
                    "OK",
                    "Not Loaded"
                ),
                pathbuf_to_string(&self.boot_result.config_path).underline(),
            ),
        ]);
    }

    fn add_http_routes(&self, table: &mut Table) {
        let router = ROUTER.read().unwrap();
        let active_http_route_count = router.get_http_routes().len();
        let is_any_http_route_exist = active_http_route_count > 0;

        table.add_row(row![
            "HTTP Routes",
            format!(
                "{} {:?}",
                warning_flag(is_any_http_route_exist, "Forwarding", "Passive"),
                active_http_route_count
            )
        ]);
    }

    fn add_integrated_web_server_routes(&self, table: &mut Table) {
        let router = ROUTER.read().unwrap();
        let active_iws_route_count = router.get_iws_routes().len();
        let is_any_iws_route_exist = active_iws_route_count > 0;

        table.add_row(row![
            "Integrated Web Server Routes",
            format!(
                "{} {:?}",
                warning_flag(is_any_iws_route_exist, "Forwarding", "Idle"),
                active_iws_route_count
            )
        ]);
    }

    fn add_https_routes(&self, table: &mut Table) {
        let router = ROUTER.read().unwrap();
        let active_https_route_count = router.get_https_routes().len();
        let is_any_https_route_exist = active_https_route_count > 0;

        table.add_row(row![
            "HTTPS Routes",
            format!(
                "{} {:?}",
                warning_flag(is_any_https_route_exist, "Forwarding", "Idle"),
                active_https_route_count
            )
        ]);
    }

    fn add_secure_iws_routes(&self, table: &mut Table) {
        let router = ROUTER.read().unwrap();
        let active_secure_iws_route_count = router.get_secure_iws_routes().len();
        let is_any_iws_route_exist = active_secure_iws_route_count > 0;

        table.add_row(row![
            "Secure Integrated Web Server Routes",
            format!(
                "{} {:?}",
                warning_flag(is_any_iws_route_exist, "Forwarding", "Idle"),
                active_secure_iws_route_count
            )
        ]);
    }

    fn add_jrpc_authentication_token(&self, table: &mut Table) {
        let rpc_server = RPC_SERVER.read().unwrap();

        table.add_row(row![
            "JRPC Authentication Token",
            format!(
                "{}",
                mask_token(&rpc_server.rpc_session.authorization_token)
            )
        ]);
    }

    fn add_jrpc_encryption_key(&self, table: &mut Table) {
        let rpc_server = RPC_SERVER.read().unwrap();

        table.add_row(row![
            "JRPC Encryption Key",
            format!("{}", mask_token(&rpc_server.rpc_session.aes_encryption_key))
        ]);
    }

    fn add_jrpc_server(&self, table: &mut Table) {
        let is_active = self.boot_result.config.rpc_server.is_active;

        let formatted_endpoint = if is_active {
            self.boot_result
                .config
                .rpc_server
                .get_endpoint()
                .underline()
        } else {
            "".underline()
        };

        let flag = status_flag(is_active, "Active", "Passive");
        table.add_row(row![
            "JRPC Server",
            format!("{} on {}", flag, &formatted_endpoint)
        ]);
    }

    fn add_http_server(&self, table: &mut Table) {
        let is_active = self.boot_result.config.http_server.is_active;

        let formatted_endpoint = if is_active {
            self.boot_result
                .config
                .http_server
                .get_endpoint()
                .underline()
        } else {
            "".underline()
        };

        let flag = status_flag(is_active, "Active", "Passive");
        table.add_row(row![
            "HTTP Server",
            format!("{} on {}", flag, &formatted_endpoint)
        ]);
    }

    fn add_https_server(&self, table: &mut Table) {
        let is_active = self.boot_result.config.https_server.is_active;

        let formatted_endpoint = if is_active {
            self.boot_result
                .config
                .https_server
                .get_endpoint()
                .underline()
        } else {
            "".underline()
        };

        let flag = status_flag(is_active, "Active", "Passive");
        table.add_row(row![
            "HTTPS Server",
            format!("{} on {}", flag, &formatted_endpoint)
        ]);
    }

    fn add_log_output_path(&self, table: &mut Table) {
        let log_output_path = &self
            .boot_result
            .runtime_path
            .join(&self.boot_result.config.logger.log_dir_name)
            .to_string_lossy()
            .to_string();

        table.add_row(row![
            "Log Output Path",
            format!("{}", log_output_path.underline())
        ]);
    }

    fn add_max_log_file_size(&self, table: &mut Table) {
        let max_log_file_size = self.boot_result.config.logger.log_file_size;
        let display_size = (max_log_file_size as i32) / 1_000_000;

        table.add_row(row![
            "Maximum Log File Size",
            format!("{} ({} mb)", max_log_file_size, display_size)
        ]);
    }

    fn add_log_levels(&self, table: &mut Table) {
        let log_levels = &self
            .boot_result
            .config
            .logger
            .log_levels
            .iter()
            .map(|item| -> String { format!("[{}]", item) })
            .collect::<Vec<String>>()
            .join(", ");

        table.add_row(row!["Log Levels", log_levels]);
    }

    pub fn render(&self) {
        let mut table = Table::new();
        table.set_format(self.table_format());

        self.add_build_info(&mut table);
        self.add_runtime_directory(&mut table);
        self.add_config_file(&mut table);
        self.add_router_file(&mut table);
        self.add_http_routes(&mut table);
        self.add_integrated_web_server_routes(&mut table);
        self.add_https_routes(&mut table);
        self.add_secure_iws_routes(&mut table);
        self.add_jrpc_authentication_token(&mut table);
        self.add_jrpc_encryption_key(&mut table);
        self.add_jrpc_server(&mut table);
        self.add_http_server(&mut table);
        self.add_https_server(&mut table);
        self.add_log_output_path(&mut table);
        self.add_max_log_file_size(&mut table);
        self.add_log_levels(&mut table);

        table.printstd();
    }
}
