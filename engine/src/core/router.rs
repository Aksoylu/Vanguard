use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::models::route::{HttpRoute, HttpsRoute, JsonRoute};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Router {
    http_route_table: HashMap<String, HttpRoute>,
    https_route_table: HashMap<String, HttpsRoute>,
}

impl Default for Router {
    fn default() -> Self {
        let http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let https_route_table: HashMap<String, HttpsRoute> = HashMap::new();

        Self {
            http_route_table,
            https_route_table,
        }
    }
}

impl Router {
    pub fn create(route_list: Vec<JsonRoute>) -> Result<Self, String> {
        let mut http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let mut https_route_table: HashMap<String, HttpsRoute> = HashMap::new();

        for each_route in route_list {
            let protocol_name = each_route.protocol.clone().to_lowercase();

            if protocol_name == "http" {
                let new_http_route = HttpRoute {
                    source: each_route.source.clone(),
                    target: each_route.target.clone(),
                };
                http_route_table.insert(each_route.source.clone(), new_http_route.clone());
            } else if protocol_name == "https" {
                let new_https_route = HttpsRoute {
                    source: each_route.source.clone(),
                    target: each_route.target.clone(),
                    ssl_path: each_route.ssl.clone().unwrap_or_default(),
                };

                https_route_table.insert(each_route.source.clone(), new_https_route.clone());
            } else {
                return Err(format!("Error: Unsupported protocol: {}", protocol_name).into());
            }
        }

        Ok(Self {
            http_route_table,
            https_route_table,
        })
    }

    pub fn get_http_routes(&self) -> HashMap<String, HttpRoute> {
        self.http_route_table.clone()
    }

    pub fn add_http_route(mut self, route_name: String, route_body: HttpRoute) -> Self {
        self.http_route_table.insert(route_name, route_body);

        self
    }

    pub fn get_https_routes(&self) -> HashMap<String, HttpsRoute> {
        self.https_route_table.clone()
    }
}
