use jsonrpc_core::{Error, Value};

use crate::{
    constants::Constants, rpc_service::models::get_build_version_response::GetBuildVersionResponse,
};

pub fn get_build_version(_payload: Value) -> Result<Value, Error> {
    let build_version_number = Constants::VERSION_NUMBER;
    let build_version_name = Constants::VERSION_NAME.to_string();

    let response = GetBuildVersionResponse::build(build_version_number, build_version_name)?;

    Ok(response)
}
