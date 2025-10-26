use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    constants::Constants,
    models::route::{HttpRoute, HttpsRoute, IwsRoute, JsonRoute, SecureIwsRoute},
    utils::{
        directory_utility::get_runtime_path,
        file_utility::{load_json, save_json},
    },
};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Router {
    http_route_table: HashMap<String, HttpRoute>,
    https_route_table: HashMap<String, HttpsRoute>,
    iws_route_table: HashMap<String, IwsRoute>,
    secure_iws_route_table: HashMap<String, SecureIwsRoute>,
    save_path: PathBuf,
}

impl Default for Router {
    fn default() -> Self {
        let http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let https_route_table: HashMap<String, HttpsRoute> = HashMap::new();
        let iws_route_table: HashMap<String, IwsRoute> = HashMap::new();
        let secure_iws_route_table: HashMap<String, SecureIwsRoute> = HashMap::new();

        let mut save_path = get_runtime_path().clone();
        save_path.push(Constants::ROUTER_FILENAME);

        Self {
            http_route_table,
            https_route_table,
            iws_route_table,
            secure_iws_route_table,
            save_path,
        }
    }
}

impl Router {
    pub fn load(load_path: PathBuf) -> (Router, bool) {
        let read_route_operation = load_json::<Vec<JsonRoute>>(&load_path);
        if read_route_operation.is_err() {
            return (Router::default(), false);
        }

        let route_list = read_route_operation.unwrap();
        let router = Router::build(load_path, route_list);
        if router.is_ok() {
            return (router.unwrap(), true);
        } else {
            return (Router::default(), false);
        }
    }

    pub fn save(&self) {
        let export_data = self.convert_to_json_route_vec();
        let write_operation = save_json(&self.save_path, &export_data);

        if write_operation.is_err() {
            let save_path_as_string = &self.save_path.to_str().unwrap_or_default();
            eprintln!(
                "Could not write Router file on Path: {}.\nPlease check your runtime path is exist and Vanguard has correct permissions", 
                save_path_as_string
            );
        }
    }

    fn build(save_path: PathBuf, route_list: Vec<JsonRoute>) -> Result<Self, String> {
        let mut http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let mut https_route_table: HashMap<String, HttpsRoute> = HashMap::new();
        let mut iws_route_table: HashMap<String, IwsRoute> = HashMap::new();
        let mut secure_iws_route_table: HashMap<String, SecureIwsRoute> = HashMap::new();

        for each_route in route_list {
            let protocol_name = each_route.protocol.clone().to_lowercase();

            if protocol_name == "http" {
                let new_http_route = HttpRoute {
                    source: each_route.source.clone(),
                    target: each_route.target.clone().unwrap_or_default(),
                };
                http_route_table.insert(each_route.source.clone(), new_http_route.clone());
            } else if protocol_name == "https" {
                let new_https_route = HttpsRoute {
                    source: each_route.source.clone(),
                    target: each_route.target.clone().unwrap_or_default(),
                    ssl_context: each_route.ssl.clone().unwrap_or_default(),
                };

                https_route_table.insert(each_route.source.clone(), new_https_route.clone());
            } else if protocol_name == "iws" {
                let new_iws_route: IwsRoute = IwsRoute {
                    source: each_route.source.clone(),
                    serving_path: each_route.serving_path.clone().unwrap_or_default(),
                };

                iws_route_table.insert(each_route.source.clone(), new_iws_route.clone());
            } else if protocol_name == "secureiws" {
                let new_secure_iws_route = SecureIwsRoute {
                    source: each_route.source.clone(),
                    serving_path: each_route.serving_path.clone().unwrap_or_default(),
                    ssl_context: each_route.ssl.clone().unwrap_or_default(),
                };

                secure_iws_route_table.insert(
                    new_secure_iws_route.source.clone(),
                    new_secure_iws_route.clone(),
                );
            } else {
                return Err(format!("Error: Unsupported protocol: {}", protocol_name).into());
            }
        }

        Ok(Router {
            http_route_table,
            https_route_table,
            iws_route_table,
            secure_iws_route_table,
            save_path,
        })
    }

    fn convert_to_json_route_vec(&self) -> Vec<JsonRoute> {
        let mut json_route_vec = <Vec<JsonRoute>>::new();

        for each_http_entity in &self.http_route_table {
            let each_http_route = each_http_entity.1;

            json_route_vec.push(JsonRoute {
                protocol: "http".to_string(),
                source: each_http_route.source.clone(),
                target: Some(each_http_route.target.clone()),
                ssl: None,
                serving_path: None,
            });
        }

        for each_https_entity in &self.https_route_table {
            let each_https_route = each_https_entity.1;

            json_route_vec.push(JsonRoute {
                protocol: "https".to_string(),
                source: each_https_route.source.clone(),
                target: Some(each_https_route.target.clone()),
                ssl: Some(each_https_route.ssl_context.clone()),
                serving_path: None,
            })
        }

        for each_iws_entity in &self.iws_route_table {
            let each_iws_route = each_iws_entity.1;

            json_route_vec.push(JsonRoute {
                protocol: "iws".to_string(),
                source: each_iws_route.source.clone(),
                target: None,
                ssl: None,
                serving_path: Some(each_iws_route.serving_path.clone()),
            })
        }

        for each_secure_iws_entity in &self.secure_iws_route_table {
            let each_secure_iws_route = each_secure_iws_entity.1;

            json_route_vec.push(JsonRoute {
                protocol: "secureiws".to_string(),
                source: each_secure_iws_route.source.clone(),
                target: None,
                ssl: Some(each_secure_iws_route.ssl_context.clone()),
                serving_path: Some(each_secure_iws_route.serving_path.clone()),
            })
        }

        json_route_vec
    }

    /* Service Functions */
    pub fn list_routes(&self) -> Vec<JsonRoute> {
        let export_data = self.convert_to_json_route_vec();

        export_data
    }

    pub fn add_http_route(mut self, route_body: HttpRoute) -> Self {
        let route_name = route_body.source.clone();

        if self.http_route_table.contains_key(&route_name) {
            self.http_route_table.remove(&route_name);
        }

        self.http_route_table.insert(route_name, route_body);

        self.save();
        self
    }

    pub fn add_https_route(mut self, route_body: HttpsRoute) -> Self {
        let route_name = route_body.source.clone();

        if self.https_route_table.contains_key(&route_name) {
            self.https_route_table.remove(&route_name);
        }

        self.https_route_table.insert(route_name, route_body);

        self.save();
        self
    }

    pub fn add_iws_route(mut self, route_body: IwsRoute) -> Self {
        let route_name = route_body.source.clone();

        if self.iws_route_table.contains_key(&route_name) {
            self.iws_route_table.remove(&route_name);
        }

        self.iws_route_table.insert(route_name, route_body);

        self.save();
        self
    }

    pub fn add_secure_iws_route(mut self, route_body: SecureIwsRoute) -> Self {
        let route_name = route_body.source.clone();

        if self.secure_iws_route_table.contains_key(&route_name) {
            self.secure_iws_route_table.remove(&route_name);
        }

        self.secure_iws_route_table.insert(route_name, route_body);

        self.save();
        self
    }

    pub fn delete_http_route(mut self, source: String) -> Self {
        if self.http_route_table.contains_key(&source) {
            self.http_route_table.remove(&source);
        }

        self.save();
        self
    }

    pub fn delete_https_route(mut self, source: String) -> Self {
        if self.https_route_table.contains_key(&source) {
            self.https_route_table.remove(&source);
        }

        self.save();
        self
    }

    pub fn delete_iws_route(mut self, source: String) -> Self {
        if self.iws_route_table.contains_key(&source) {
            self.iws_route_table.remove(&source);
        }

        self.save();
        self
    }

    pub fn delete_secure_iws_route(mut self, source: String) -> Self {
        if self.secure_iws_route_table.contains_key(&source) {
            self.secure_iws_route_table.remove(&source);
        }

        self.save();
        self
    }

    pub fn get_http_routes(&self) -> HashMap<String, HttpRoute> {
        self.http_route_table.clone()
    }

    pub fn get_https_routes(&self) -> HashMap<String, HttpsRoute> {
        self.https_route_table.clone()
    }

    pub fn get_iws_routes(&self) -> HashMap<String, IwsRoute> {
        self.iws_route_table.clone()
    }

    pub fn get_secure_iws_routes(&self) -> HashMap<String, SecureIwsRoute> {
        self.secure_iws_route_table.clone()
    }
}
