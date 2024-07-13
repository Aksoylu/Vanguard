use jsonrpc_core::IoHandler;

use crate::utils::network_utility::parse_ip_address;

use jsonrpc_http_server::ServerBuilder;

use super::routes::RPCRouter;

pub struct RPCServer {
    ip_address: String,
    port: u16,
    auth_token: String,
    endpoint: String,
    rpc_registry: IoHandler,
}

impl RPCServer {
    pub fn singleton(ip_address: String, port: u16, auth_token: String) -> Self {
        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let endpoint = format!("{}:{}", parsed_ip_address, port);

        let router: RPCRouter = RPCRouter::build(auth_token.clone());

        let mut rpc_registry: IoHandler = IoHandler::default();
        rpc_registry = router.bind(rpc_registry.clone());

        Self {
            ip_address,
            port,
            auth_token,
            endpoint,
            rpc_registry,
        }
    }

    /// Public: This function is repsonsible of booting process of  JRPC Server
    pub async fn start(&self) {
        let server = ServerBuilder::new(self.rpc_registry.clone())
            .start_http(&self.endpoint.parse().unwrap())
            .expect("JRPC Server failed to start.");

        println!("JRPC Server running on {}", &self.endpoint);

        server.wait();
    }

}
