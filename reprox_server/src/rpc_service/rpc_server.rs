use chrono::{DateTime, Utc};
use jsonrpc_core::{IoHandler, Params};
use serde_json::to_string;

use crate::{models::rpc_session::RpcSession, utils::{generate_hash::generate_hash, generate_salt::generate_salt, parse_ip_address::parse_ip_address}};

use jsonrpc_http_server::ServerBuilder;

use super::RPC_ROUTER;

pub struct RPCServer {
    ip_address: String,
    port: u16,
    secure_key: String,
    endpoint: String,
    function_register: IoHandler,
}

impl RPCServer {
    const SESSION_PATH: &'static str = "../variables/.session.json";

    pub fn singleton(ip_address: String, port: u16, secure_key: String) -> Self {
        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let endpoint = format!("{}:{}", parsed_ip_address, port);
        let mut function_register: IoHandler = IoHandler::default();

        // Register methods from RPC_ROUTER into function_register
        for (function_name, function_body) in RPC_ROUTER.iter() {
            function_register.add_method(function_name, move |params:Params| {
                function_body(params.clone())
            });
        }

        Self {
            ip_address,
            port,
            secure_key,
            endpoint,
            function_register,
        }
    }

    pub async fn start(&self) {
        let server = ServerBuilder::new(self.function_register.clone())
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
    pub fn create_rpc_session(&self){
        let salt = generate_salt();
        let hash = generate_hash(self.secure_key.clone(), salt);
        let created_at = Utc::now().timestamp();

        let session = RpcSession{
            ip_addr: self.ip_address.clone(),
            port: self.port.to_string(),
            hash,
            created_at
        };   

        match to_string(&session) {
            Ok(json) => {
                if let Err(_) = std::fs::write(RPCServer::SESSION_PATH, json) {
                    eprintln!("Unable to write to session file on: {}", RPCServer::SESSION_PATH);
                }
            },
            Err(e) => eprintln!("Error on serializing Rpc Session"),
        }
    }
}
