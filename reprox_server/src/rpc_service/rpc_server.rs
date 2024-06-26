use chrono::Utc;
use jsonrpc_core::IoHandler;
use serde_json::to_string;

use crate::{settings::Settings, utils::{
    crypt_utility::generate_hash,
    network_utility::parse_ip_address,
}};

use jsonrpc_http_server::ServerBuilder;

use super::{models::rpc_session::RpcSession, routes::RPCRouter};

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

        self.create_rpc_session();
        server.wait();
    }

    /// Public: This function is responsible of generating `RPC` session file for making possible to sending JRPC Request from CLI application
    ///
    /// # Arguments
    /// * `private_key` - Private key value that specified in `settings.json` file. (`&str`)
    ///
    pub fn create_rpc_session(&self) {
        let hash = generate_hash(self.auth_token.clone());
        let created_at = Utc::now().timestamp();

        let session = RpcSession {
            ip_addr: self.ip_address.clone(),
            port: self.port.to_string(),
            hash,
            created_at,
        };

        match to_string(&session) {
            Ok(json) => {
                if let Err(_) = std::fs::write(Settings::SESSION_PATH, json) {
                    eprintln!(
                        "Unable to write to session file on: {}",
                        Settings::SESSION_PATH
                    );
                }
            }
            Err(e) => eprintln!("Error on serializing Rpc Session"),
        }
    }
}
