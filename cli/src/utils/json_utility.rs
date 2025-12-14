use serde_json::{Map, Value};

pub fn create_empty_json_object() -> Value {
    let empty_map = Map::new();
    Value::Object(empty_map)
}