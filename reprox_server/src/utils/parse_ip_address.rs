use std::net::Ipv4Addr;

use super::str_to_i8::str_to_i8;

pub(crate) fn parse_ip_address(value: String) -> Ipv4Addr {
    let parts: Vec<&str> = value.split('.').collect();

    Ipv4Addr::new(
        str_to_i8(parts[0]),
        str_to_i8(parts[1]),
        str_to_i8(parts[2]),
        str_to_i8(parts[3]),
    )
}
