use jsonrpc_core::{Error, IoHandler, Params, Value};
use std::sync::Mutex;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use crate::runtime::Runtime;

pub fn get_http_routes(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let runtime = runtime.clone();
    let http_routes = runtime.lock().unwrap().router.get_http_routes();
    Ok(Value::String(format!("Http Routes: {:?}", http_routes)))
}
