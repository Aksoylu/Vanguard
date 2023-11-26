use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Environments {
    pub http_server_ip_address: String,
    pub http_server_port: u16,
    pub rpc_enabled: bool,
    pub rpc_server_port: u16,
    pub rpc_key: String
}

impl Environments {
    pub fn load() -> Self {
        dotenv().ok();

        let http_server_ip_address = env::var("HTTP_SERVER_IP_ADDRESS").expect("HTTP_SERVER_IP_ADDRESS must be set");
        
        let http_server_port = env::var("HTTP_SERVER_PORT")
            .expect("HTTP_SERVER_PORT must be set")
            .parse()
            .expect("Failed to parse HTTP_SERVER_PORT");
        
        let rpc_enabled: bool = env::var("START_RPC_SERVER")
            .expect("START_RPC_SERVER must be set")
            .parse()
            .expect("Failed to parse START_RPC_SERVER");
        
        let rpc_server_port: u16 = env::var("RPC_SERVER_PORT")
            .expect("RPC_SERVER_PORT must be set")
            .parse()
            .expect("Failed to parse HTTP_SERVER_PORT");

        let rpc_key = env::var("RPC_KEY").expect("RPC_KEY must be set");

        Self {
            http_server_ip_address,
            http_server_port,
            rpc_enabled,
            rpc_server_port,
            rpc_key
        }
    }
}
