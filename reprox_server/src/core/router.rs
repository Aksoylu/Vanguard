use hyper::{Body, Request, Response};
use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::core::controllers::echo::echo_controller;

lazy_static! {
    pub static ref ROUTER: HashMap<&'static str, fn(Request<Body>) -> Result<Response<Body>, hyper::Error>> = {
        let mut map = HashMap::new();
        map.insert(
            "/echo",
            echo_controller as fn(Request<Body>) -> Result<Response<Body>, hyper::Error>,
        );
        map
    };
}
