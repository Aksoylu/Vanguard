use std::net::Ipv4Addr;

use hyper::{header, Body, Request};

pub fn extract_host(req: &Request<Body>) -> String {
    req.headers()
        .get(header::HOST)
        .and_then(|host| host.to_str().ok())
        .map_or_else(|| "/".to_string(), |host_value| host_value.to_string())
}

pub fn parse_ip_address(value: String) -> Ipv4Addr {
    let parts: Vec<&str> = value.split('.').collect();

    Ipv4Addr::new(
        str_to_i8(parts[0]),
        str_to_i8(parts[1]),
        str_to_i8(parts[2]),
        str_to_i8(parts[3]),
    )
}

fn str_to_i8(value: &str) -> u8 {
    value.parse::<u8>().unwrap()
}
