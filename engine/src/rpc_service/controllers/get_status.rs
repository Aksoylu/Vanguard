use jsonrpc_core::{Error, Value};

use crate::{core::shared_memory::RUNTIME_BOOT_INFO, rpc_service::models::get_status_response::GetStatusResponse};

/// This JRPC service is responsible of returning current boot info of engine
pub fn get_status(_payload: Value) -> Result<Value, Error> {
    let status_data = RUNTIME_BOOT_INFO.read().unwrap();
    let cop = status_data.clone();

    let response = GetStatusResponse::build(cop)?;
    Ok(response)
}
