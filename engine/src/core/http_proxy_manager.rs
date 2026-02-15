use crate::core::http_client::HttpClient;
use crate::models::traffic_policy::scope_traffic_policy::ScopeTrafficPolicy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct HttpProxyManager {
    http_clients: RwLock<HashMap<String, Arc<HttpClient>>>,
}

impl Default for HttpProxyManager {
    fn default() -> Self {
        Self {
            http_clients: RwLock::new(HashMap::new()),
        }
    }
}

impl HttpProxyManager {
    pub fn get(&self, traffic_policy: &ScopeTrafficPolicy) -> Arc<HttpClient> {
        // Build a unique key for the given traffic policy
        let key = format!(
            "{}_{}",
            traffic_policy.upstream_settings.get_pool_idle_timeout(),
            traffic_policy.upstream_settings.get_max_idle_conns_per_host()
        );

        // First try to read with a read lock by given key
        {
            let get_clients = self.http_clients.read().unwrap();
            let pooled_http_client = get_clients.get(&key);
            if pooled_http_client.is_some() {
                return Arc::clone(pooled_http_client.unwrap());
            }
        }

        // If not found, acquire write lock to insert
        let mut http_caller = self.http_clients.write().unwrap();

        // Double check because another thread might have inserted it
        if let Some(client) = http_caller.get(&key) {
            return Arc::clone(client);
        }

        let client = Arc::new(HttpClient::init(
            traffic_policy.upstream_settings.get_pool_idle_timeout(),
            traffic_policy.upstream_settings.get_max_idle_conns_per_host(),
        ));

        http_caller.insert(key, Arc::clone(&client));
        client
    }
}
