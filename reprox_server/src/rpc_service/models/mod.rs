use std::sync::Arc;
use jsonrpc_core::{Error, Params, Value};

pub mod rpc_session;

pub type RpcHandler = Arc<dyn Fn(Params) -> Result<Value, Error> + Sync + Send>;
