use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;

pub mod rpc_session;

pub type RpcHandler = Arc<dyn Fn(Params) -> Result<Value, Error> + Sync + Send>;
