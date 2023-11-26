use jsonrpc_core::IoHandler;

use crate::utils::parse_ip_address::parse_ip_address;

use jsonrpc_http_server::ServerBuilder;

use super::RPC_ROUTER;

pub struct RPCServer{
    ip_address: String,
    port: u16,
    key: String,
    endpoint: String,
    function_register: IoHandler,
}

impl RPCServer{
    pub fn singleton(ip_address: &String, port: &u16, key: &String) -> Self {
        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let parsed_port = port.clone();
        let endpoint = format!("{}:{}", parsed_ip_address, parsed_port);

        let mut function_register: IoHandler = IoHandler::default();

        for(function_name, function_body) in RPC_ROUTER.iter() {
            function_register.add_method(function_name, function_body);

        }

        Self {
            ip_address: ip_address.clone(),
            port: port.clone(),
            key: key.clone(),
            endpoint,
            function_register
        }
    }

    pub async fn start(&self) {
        let server = ServerBuilder::new(self.function_register.clone())
            .start_http(&self.endpoint.parse().unwrap())
            .expect("JRPC Server failed to start.");

        println!("JRPC Server running on {}", &self.endpoint);

        server.wait();
    }
}
