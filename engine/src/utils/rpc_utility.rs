use core::fmt;
use std::str::FromStr;

use jsonrpc_core::Value;

use crate::models::traffic_policy::TrafficPolicy;

pub struct RpcParameter {}

impl RpcParameter {
    pub fn extract_string(parameter_name: &str, params: &Value) -> Option<String> {
        let value = params
            .get(parameter_name)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        value
    }

    /// Extracts a string parameter and tries to parse it into a type T using FromStr.
    /// The T::Err type from FromStr is propagated as a String for simplicity
    pub fn extract_string_enum<T>(parameter_name: &str, params: &Value) -> Option<T>
    where
        T: FromStr,           // The generic type T must implement FromStr
        T::Err: fmt::Display, // Its error type must implement Display for the map_err
    {
        params
            .get(parameter_name)
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<T>().ok())
    }

    /// Extracts a traffic_policy parameter from JSON-RPC params and deserializes it.
    /// Returns a default TrafficPolicy if the parameter is missing or invalid.
    /// This allows clients to optionally provide custom traffic policies per route.
    pub fn extract_traffic_policy(parameter_name: &str, params: &Value) -> Option<TrafficPolicy> {
        params
            .get(parameter_name)
            .and_then(|v| serde_json::from_value::<TrafficPolicy>(v.clone()).ok())
    }
}
