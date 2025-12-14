use crate::{
    constants::Constants,
    core::{errors::rpc_base_error::RPCBaseError, shared_memory::RPC_CLIENT},
    log_error, log_info,
    models::{
        base::remote_version_data::RemoteVersionData,
        commands::get_build_version_response::GetBuildVersionResponse,
    },
    utils::{console::print_colored, json_utility::create_empty_json_object},
};
use crossterm::style::Color;
use hyper::StatusCode;

pub async fn version() {
    println!("Vanguard CLI");
    println!("  Build: {}", Constants::VERSION_NUMBER);
    println!("  Version: {}", Constants::VERSION_NAME);

    let get_engine_version_info = get_version_info_from_engine().await;

    if get_engine_version_info.is_err() {
        print_colored("Can not get version information from Vanguard engine. Please check that Vanguard engine is running.", Color::Red);
        return;
    }

    let get_engine_version_response = get_engine_version_info.unwrap();
    if get_engine_version_response.code != StatusCode::OK.as_u16() {
        log_error!(
            "An error occured while getting version information from Vanguard engine: {}",
            get_engine_version_response.code
        );
        return;
    }

    let engine_build_number = get_engine_version_response.build_version_number;
    let engine_version_name = get_engine_version_response.build_version_name;

    println!("Vanguard Engine");
    println!(
        "  Build: {} ({})",
        &engine_build_number, &engine_version_name
    );

    println!("Checking for updates ...");
    let get_remote_version_data = get_latest_version_info_from_repository().await;
    if get_remote_version_data.is_err() {
        let error = get_remote_version_data.err().unwrap();
        log_error!("{}", error.reason);
        return;
    }

    let remote_version_data = get_remote_version_data.unwrap();

    println!("Latest Vanguard Version");
    println!(
        "  Build: {} ({})",
        &remote_version_data.version_number, &remote_version_data.version_name
    );

    if remote_version_data.version_number == Constants::VERSION_NUMBER {
        print_colored("Your Vanguard version is up to date", Color::Green);
    } else if engine_build_number >= Constants::VERSION_NUMBER {
        print_colored("Your Vanguard version is outdated. We strongly suggest you to keep your Vanguard version up to date", Color::Yellow);
        print!(
            "You can update your Vanguard version by following instructions at: {}",
            Constants::UPDATE_MANUAL_URL
        );
    }
}

async fn get_version_info_from_engine() -> Result<GetBuildVersionResponse, RPCBaseError> {
    let serialized_input = create_empty_json_object();

    let lock = {
        let rpc_client = RPC_CLIENT.read().await;
        let rpc_call_response = rpc_client
            .call("get_build_version", serialized_input)
            .await?;
        let result = rpc_call_response.result;

        let code = &result["code"].as_i64().unwrap_or_default();

        let version_number = &result["build_version_number"].as_f64().unwrap_or_default();
        let version_name = &result["build_version_name"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        Ok(GetBuildVersionResponse {
            code: code.to_owned() as u16,
            build_version_number: version_number.to_owned(),
            build_version_name: version_name.to_owned(),
        })
    }?;

    Ok(lock)
}

async fn get_latest_version_info_from_repository() -> Result<RemoteVersionData, RPCBaseError> {
    let response = reqwest::get(Constants::VERSION_CONTROL_URL)
        .await
        .map_err(|_| {
            RPCBaseError::build("Can not get remote version. Please check your WAN access")
        })?;

    let remote_version_data = response
        .json::<RemoteVersionData>()
        .await
        .map_err(|error| {
            let error_content = format!(
                "Can not parse remote version data or IO error occurred: {}",
                error.to_string()
            );
            RPCBaseError::build(error_content.as_str())
        })?;

    Ok(remote_version_data)
}
