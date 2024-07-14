use jsonrpc_core::Params;
use serde_json::Value;

pub struct RpcParameter {}

impl RpcParameter {
    pub fn extract_string(parameter_name: &str, params: Params) -> Option<String> {
        let value = params
            .clone()
            .parse::<Value>()
            .ok()
            .and_then(|v| v.get(parameter_name).map(|t| t.to_string()));

        value
    }
}
