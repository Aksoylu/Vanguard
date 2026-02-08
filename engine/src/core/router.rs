use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    constants::Constants,
    core::shared_memory::RUNTIME_BOOT_INFO,
    models::{
        http_route::HttpRoute,
        route::{HttpsRoute, IwsRoute, JsonRoute, SecureIwsRoute},
        ssl_context::SslContext,
        traffic_policy::scope_traffic_policy::ScopeTrafficPolicy,
    },
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

    #[serde(skip_serializing, skip_deserializing)]
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
        let read_route_operation = load_json::<Router>(&load_path);
        if read_route_operation.is_err() {
            return (Router::default(), false);
        }

        let mut router = read_route_operation.unwrap();

        let mut save_path = get_runtime_path().clone();
        save_path.push(Constants::ROUTER_FILENAME);

        router.save_path = save_path;

        (router, true)
    }

    pub fn save(&self) {
        let write_operation = save_json::<Router>(&self.save_path, &self.clone());
        if write_operation.is_err() {
            let fd = write_operation.err().unwrap();
            println!("{:?}", fd);
            println!("{:?}", &self.save_path);
            let save_path_as_string = &self.save_path.to_str().unwrap_or_default();
            eprintln!(
                "Could not write Router file on Path: {}.\nPlease check your runtime path is exist and Vanguard has correct permissions", 
                save_path_as_string
            );
        }
    }

    fn convert_to_json_route_vec(&self) -> Vec<JsonRoute> {
        let mut json_route_vec = <Vec<JsonRoute>>::new();

        for (source, http_route) in &self.http_route_table {
            json_route_vec.push(JsonRoute {
                protocol: "http".to_string(),
                source: source.clone(),
                target: Some(http_route.target.clone()),
                ssl: None,
                serving_path: None,
            });
        }

        for (source, https_route) in &self.https_route_table {
            json_route_vec.push(JsonRoute {
                protocol: "https".to_string(),
                source: source.clone(),
                target: Some(https_route.target.clone()),
                ssl: Some(https_route.ssl_context.clone()),
                serving_path: None,
            })
        }

        for (source, iws_route) in &self.iws_route_table {
            json_route_vec.push(JsonRoute {
                protocol: "iws".to_string(),
                source: source.clone(),
                target: None,
                ssl: None,
                serving_path: Some(iws_route.serving_path.clone()),
            })
        }

        for (source, secure_iws_route) in &self.secure_iws_route_table {
            json_route_vec.push(JsonRoute {
                protocol: "secureiws".to_string(),
                source: source.clone(),
                target: None,
                ssl: Some(secure_iws_route.ssl_context.clone()),
                serving_path: Some(secure_iws_route.serving_path.clone()),
            })
        }

        json_route_vec
    }

    /* Service Functions */
    pub fn list_routes(&self) -> Vec<JsonRoute> {
        let export_data = self.convert_to_json_route_vec();

        export_data
    }

    pub fn add_http_route(
        &mut self,
        source: &String,
        target: &String,
        traffic_policy: Option<ScopeTrafficPolicy>,
    ) {
        if self.http_route_table.contains_key(source) {
            self.http_route_table.remove(source);
        }

        let mut final_traffic_policy = ScopeTrafficPolicy::default();

        if traffic_policy.is_some() {
            final_traffic_policy = traffic_policy.unwrap();
        } else {
            let runtime_boot_info = RUNTIME_BOOT_INFO.read().unwrap();
            final_traffic_policy = runtime_boot_info.config.http_server.traffic_policy.clone();
        }

        let new_route = HttpRoute {
            target: target.to_owned(),
            traffic_policy: final_traffic_policy,
        };

        self.http_route_table.insert(source.to_owned(), new_route);
        self.save();
    }

    pub fn add_https_route(
        &mut self,
        source: &String,
        target: &String,
        ssl_cert_path: &String,
        ssl_private_key_path: &String,
    ) {
        if self.https_route_table.contains_key(source) {
            self.https_route_table.remove(source);
        }

        let new_route = HttpsRoute {
            target: target.to_owned(),
            ssl_context: SslContext {
                certificate_file_path: ssl_cert_path.to_owned(),
                private_key_file_path: ssl_private_key_path.to_owned(),
            },
        };

        self.https_route_table.insert(source.to_owned(), new_route);
        self.save();
    }

    pub fn add_iws_route(&mut self, source: &String, serving_path: &String) {
        if self.iws_route_table.contains_key(source) {
            self.iws_route_table.remove(source);
        }

        let new_route = IwsRoute {
            serving_path: serving_path.to_owned(),
        };

        self.iws_route_table.insert(source.to_owned(), new_route);
        self.save();
    }

    pub fn add_secure_iws_route(
        &mut self,
        source: &String,
        serving_path: &String,
        ssl_cert_path: &String,
        ssl_private_key_path: &String,
    ) {
        if self.secure_iws_route_table.contains_key(source) {
            self.secure_iws_route_table.remove(source);
        }

        let new_route: SecureIwsRoute = SecureIwsRoute {
            serving_path: serving_path.to_owned(),

            ssl_context: SslContext {
                certificate_file_path: ssl_cert_path.to_owned(),
                private_key_file_path: ssl_private_key_path.to_owned(),
            },
        };

        self.secure_iws_route_table
            .insert(source.to_owned(), new_route);
        self.save();
    }

    pub fn delete_http_route(&mut self, source: String) {
        if self.http_route_table.contains_key(&source) {
            self.http_route_table.remove(&source);
        }

        self.save();
    }

    pub fn delete_https_route(&mut self, source: String) {
        if self.https_route_table.contains_key(&source) {
            self.https_route_table.remove(&source);
        }

        self.save();
    }

    pub fn delete_iws_route(&mut self, source: String) {
        if self.iws_route_table.contains_key(&source) {
            self.iws_route_table.remove(&source);
        }

        self.save();
    }

    pub fn delete_secure_iws_route(&mut self, source: String) {
        if self.secure_iws_route_table.contains_key(&source) {
            self.secure_iws_route_table.remove(&source);
        }

        self.save();
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
