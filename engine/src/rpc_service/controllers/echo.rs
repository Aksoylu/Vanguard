use jsonrpc_core::{Error, IoHandler, Params, Value};
use std::sync::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::runtime::Runtime;

pub fn echo_controller(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    Ok(Value::String(format!("Hello, {:?}", params)))
}
