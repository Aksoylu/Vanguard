use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone)]
pub struct Router {
    route_table: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Route {
    pub source: String,
    pub target: String,
}

impl Router {
    const ROUTER_PATH: &'static str = "../variables/routing.json";

    pub fn load() -> Self {
        let mut route_table: HashMap<String, String> = HashMap::new();

        let route_list = Self::read_file();
        for each_route in route_list {
            route_table.insert(each_route.source.clone(), each_route.target.clone());
        }

        Self { route_table }
    }

    fn read_file() -> Vec<Route> {
        let mut route_list: Vec<Route> = vec![];

        let mut file = match File::open(Router::ROUTER_PATH) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Failed to open routing.json file.");
                return route_list;
            }
        };

        let mut file_contents = String::new();
        if let Err(_) = file.read_to_string(&mut file_contents) {
            eprintln!("Failed to read the routing.json contents.");
            return route_list;
        }

        route_list = match serde_json::from_str(&file_contents) {
            Ok(person) => person,
            Err(_) => {
                eprintln!("Failed to deserializing routing.json.");
                return route_list;
            }
        };

        route_list
    }

    pub fn get(&self) -> HashMap<String, String> {
        return self.route_table.clone();
    }
}
