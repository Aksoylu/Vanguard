use std::fmt::format;

use crate::{
    common::enums::log_level::LogLevel,
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error,
    models::{
        commands::get_status_response::GetStatusResponse,
        entity::{
            engine_http_server_config::EngineHttpServerConfig,
            engine_https_server_config::EngineHttpsServerConfig,
            engine_logger_config::EngineLoggerConfig,
        },
    },
    utils::{console::separator, json_utility::create_empty_json_object, text_utility::status_flag},
};
use colored::Colorize;
use hyper::StatusCode;
use prettytable::{
    color,
    format::{self, TableFormat},
    Attr, Cell, Row, Table,
};

pub async fn get_status() {
    let result = execute().await;

    if result.is_err() {
        let error_message = result.unwrap_err();
        log_error!("{}", error_message.reason);
        return;
    }

    let get_status_response = result.unwrap();

    if get_status_response.code != StatusCode::OK.as_u16() {
        log_error!("An error occured while getting status of Vanguard Engine.");
        return;
    }

    separator(36);
    print_table_header();
    print_status_table(&get_status_response);
    separator(36);
}

async fn execute() -> Result<GetStatusResponse, RPCBaseError> {
    let request = create_empty_json_object();

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client.call("get_status", request).await?;
        let result = rpc_call_response.result;

        let response: GetStatusResponse = serde_json::from_value(result)
            .map_err(|e| RPCBaseError::build(&format!("Yanıt ayrıştırma hatası: {}", e)))?;

        Ok(response)
    }?;

    Ok(lock)
}

fn print_table_header() {
    let styled_header = format!("[{}]", "Vanguard Engine Status").cyan().bold();
    println!("{}", styled_header);
}

fn table_format() -> TableFormat {
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

fn print_status_table(data: &GetStatusResponse) {
    let mut table = Table::new();
    table.set_format(table_format());
    println!("{:?}", data);

    /*
    print_engine_info(
        data.runtime_path,
        &data.rpc_session_path,
    );
    print_engine_startup_config(
        &mut table,
        &data.is_config_loaded_successfully,
        &data.config,

        &data.is_router_loaded_successfully,
        &data.route_path,
    );
     */

    print_http_service_status(
        &mut table,
        &data.config.http_server,
        &data.http_route_count,
        &data.iws_route_count,
    );

    print_https_service_status(
        &mut table,
        &data.config.https_server,
        &data.https_route_count,
        &data.secure_iws_route_count,
    );

    print_engine_logger_config(&mut table, &data.runtime_path, &data.config.logger);

    print_config_file(
        &mut table,
        data.is_config_loaded_successfully,
        &data.config_path,
    );

    table.printstd();
}

fn render_server_status(is_active: bool, endpoint: String) -> String {
    let status_text = if is_active {
        format!(
            "{} {} (Socket: {})",
            "●".green(),
            "Running".white().bold(),
            endpoint.dimmed()
        )
    } else {
        format!("{} {}", "●".red(), "Disabled".white().bold(),)
    };

    status_text
}

fn render_forwarding_status(is_active: bool, route_count: &usize) -> String {
    let forwarding_status = if is_active {
        if *route_count > 0 {
            format!(
                "{} {} ({})",
                "►".green(),
                "Forwarding".white().bold(),
                route_count.clone().to_string().dimmed()
            )
        } else {
            format!(
                "{} {} ({})",
                "⏸︎".yellow(),
                "Waiting".white().bold(),
                "Idle".dimmed()
            )
        }
    } else {
        format!(
            "{} {} ({})",
            "⏹︎".red(),
            "Passive".white().bold(),
            "-".dimmed()
        )
    };

    forwarding_status
}

/// @todo
fn print_engine_info() {}

fn print_http_service_status(
    table: &mut Table,
    http_config: &EngineHttpServerConfig,
    http_route_count: &usize,
    iws_route_count: &usize,
) {
    let formatted_endpoint = format!("{}:{}", http_config.ip_address, http_config.port);
    let server_status = render_server_status(http_config.is_active, formatted_endpoint);

    let http_forwarding_status = render_forwarding_status(http_config.is_active, http_route_count);
    let iws_forwarding_status = render_forwarding_status(http_config.is_active, iws_route_count);

    table.add_row(Row::new(vec![
        Cell::new("Http & IWS Server"),
        Cell::new(server_status.as_str()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Http Forwarding"),
        Cell::new(http_forwarding_status.as_str()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("IWS Forwarding"),
        Cell::new(iws_forwarding_status.as_str()),
    ]));
}

fn print_https_service_status(
    table: &mut Table,
    https_config: &EngineHttpsServerConfig,
    https_route_count: &usize,
    secure_iws_route_count: &usize,
) {
    let formatted_endpoint = format!("{}:{}", https_config.ip_address, https_config.port);
    let server_status = render_server_status(https_config.is_active, formatted_endpoint);

    let forwarding_status = render_forwarding_status(https_config.is_active, https_route_count);
    let secure_iws_forwarding_status =
        render_forwarding_status(https_config.is_active, secure_iws_route_count);

    table.add_row(Row::new(vec![
        Cell::new("Https Server"),
        Cell::new(server_status.as_str()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Https Forwarding"),
        Cell::new(forwarding_status.as_str()),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Secure IWS Forwarding"),
        Cell::new(secure_iws_forwarding_status.as_str()),
    ]));
}

fn print_engine_logger_config(
    table: &mut Table,
    runtime_path: &String,
    logger_config: &EngineLoggerConfig,
) {
    // printing log file details
    let file_size_as_mb = logger_config.log_file_size / 1000000;
    let file_size_as_str = format!("{} ({} mb)", logger_config.log_file_size, file_size_as_mb);
    let log_file_details = format!(
        "Maximum log file size: {} | Keeping last {} days",
        file_size_as_str, logger_config.keep_last_logs
    );

    table.add_row(Row::new(vec![
        Cell::new("Log File"),
        Cell::new(log_file_details.as_str()),
    ]));

    // printing log levels
    let mut log_levels_as_string = String::new();
    for log_level_as_string in &logger_config.log_levels {
        let logic_log_level: LogLevel = log_level_as_string.parse().unwrap();
        log_levels_as_string = format!("{} {}", log_levels_as_string, logic_log_level)
    }

    table.add_row(Row::new(vec![
        Cell::new("Log Levels"),
        Cell::new(log_levels_as_string.as_str()),
    ]));

    // printing log file path
    let log_file_path = format!("{}/{}", &runtime_path, logger_config.log_dir_name).underline();

    table.add_row(Row::new(vec![
        Cell::new("Log File Path"),
        Cell::new(log_file_path.to_string().as_str()),
    ]));
}

fn print_config_file(
    table: &mut Table,
    is_config_loaded_successfully: bool,
    config_path: &String,
) {
    let formatted_config_file_path = format!(
        "{} {}",
        status_flag(is_config_loaded_successfully, "Loaded", "Not Loaded"),
        config_path.underline()
    );

    table.add_row(Row::new(vec![
        Cell::new("Config File"),
        Cell::new(formatted_config_file_path.as_str()),
    ]));
}
