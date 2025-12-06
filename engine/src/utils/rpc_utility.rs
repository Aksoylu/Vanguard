use jsonrpc_core::Value;

pub struct RpcParameter {}

impl RpcParameter {
    pub fn extract_string(parameter_name: &str, params: &Value) -> Option<String> {
        let value = params
            .get(parameter_name)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        value
    }
}
