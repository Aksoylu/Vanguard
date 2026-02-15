use serde::{Deserialize, Serialize};

use crate::models::settings::logger_settings::LoggerSettings;

use super::{
    http_server_config::HttpServerConfig, https_server_config::HttpsServerConfig,
    rpc_server_config::RpcServerConfig,
    traffic_policy::{
        global_traffic_policy::GlobalTrafficPolicy, scope_traffic_policy::ScopeTrafficPolicy,
    },
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub http_server: HttpServerConfig,
    pub https_server: HttpsServerConfig,
    pub rpc_server: RpcServerConfig,
    pub logger: LoggerSettings,

    #[serde(default = "default_global_traffic_policy")]
    pub global_traffic_policy: GlobalTrafficPolicy,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_server: Default::default(),
            https_server: Default::default(),
            rpc_server: Default::default(),
            logger: Default::default(),
            global_traffic_policy: GlobalTrafficPolicy::global(),
        }
    }
}

fn default_global_traffic_policy() -> GlobalTrafficPolicy {
    GlobalTrafficPolicy::global()
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if self.http_server.ip_address.is_empty() {
            return Err("HTTP server IP address is empty".into());
        }
        if self.http_server.port == 0 {
            return Err("HTTP server port is 0".into());
        }

        if self.https_server.ip_address.is_empty() {
            return Err("HTTPS server IP address is empty".into());
        }
        if self.https_server.port == 0 {
            return Err("HTTPS server port is 0".into());
        }

        if self.rpc_server.ip_address.is_empty() {
            return Err("RPC server IP address is empty".into());
        }

        if self.rpc_server.port == 0 {
            return Err("RPC server port is 0".into());
        }

        if self.rpc_server.private_secret_key.is_empty() {
            return Err("RPC server private key is empty".into());
        }

        Ok(())
    }

    /// Returns the effective traffic policy for the HTTP server.
    /// This merges the global traffic policy with any HTTP server overrides.
    pub fn get_http_effective_policy(&self) -> ScopeTrafficPolicy {
        let mut policy = ScopeTrafficPolicy {
            http1_protocol_settings: self.global_traffic_policy.http1_protocol_settings.clone(),
            http2_protocol_settings: self.global_traffic_policy.http2_protocol_settings.clone(),
            upstream_settings: self.global_traffic_policy.upstream_settings.clone(),
        };

        if let Some(ref overrides) = self.http_server.traffic_policy {
            policy.merge(overrides);
        }

        policy
    }

    /// Returns the effective traffic policy for the HTTPS server.
    /// This merges the global traffic policy with any HTTPS server overrides.
    pub fn get_https_effective_policy(&self) -> ScopeTrafficPolicy {
        let mut policy = ScopeTrafficPolicy {
            http1_protocol_settings: self.global_traffic_policy.http1_protocol_settings.clone(),
            http2_protocol_settings: self.global_traffic_policy.http2_protocol_settings.clone(),
            upstream_settings: self.global_traffic_policy.upstream_settings.clone(),
        };

        if let Some(ref overrides) = self.https_server.traffic_policy {
            policy.merge(overrides);
        }

        policy
    }
}
