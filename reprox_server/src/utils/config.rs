use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Environments {
    pub http_server_ip_address: String,
    pub http_server_port: i32,
}

impl Environments {
    pub fn load() -> Self {
        dotenv().ok();

        let http_server_ip_address = env::var("HTTP_SERVER_IP_ADDRESS").expect("Http Server IP Address must be set");
        let http_server_port = env::var("HTTP_SERVER_PORT")
            .expect("Http Server PORT must be set")
            .parse()
            .expect("Failed to parse HTTP_SERVER_PORT");

        Self {
            http_server_ip_address,
            http_server_port,
        }
    }
}
