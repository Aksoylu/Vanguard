use jsonrpc_core::IoHandler;
use std::sync::Arc;
use std::sync::Mutex;

use crate::core::rpc_session::RpcSession;
use crate::core::rpc_session::RPC_SESSION;
use crate::rpc_service::routes::ROUTES;
use crate::rpc_service::rpc_middleware::RpcMiddleware;
use crate::{log_info, runtime::Runtime, utils::network_utility::parse_ip_address};

use jsonrpc_http_server::ServerBuilder;

pub struct RPCServer {
    rpc_session: RpcSession,
    ip_address: String,
    port: u16,
    endpoint: String,
}

impl RPCServer {
    pub async fn new(rpc_session: RpcSession) -> Self {
        let ip_address = rpc_session.ip_addr.clone();
        let port = rpc_session.port.clone();

        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let endpoint = format!("{}:{}", parsed_ip_address, &port);

        Self {
            rpc_session,
            ip_address,
            port,
            endpoint
        }
    }

    pub async fn build_rpc_handler(&self) -> IoHandler {
        let mut controller_registry = IoHandler::default();

        let authorization_token = self.rpc_session.authorization_token.clone();
        let aes_encryption_key = self.rpc_session.aes_encryption_key.clone();

        for (service_name, controller_delegate) in ROUTES.iter() {
            controller_registry.add_method(
                service_name,
                RpcMiddleware::bind(
                    controller_delegate.clone(),
                    authorization_token.clone(),
                    aes_encryption_key.clone(),
                ),
            );
        }

        controller_registry
    }

    /// Public: This function is repsonsible of booting process of  JRPC Server
    pub async fn start(&self) {
        let rpc_handler =self.build_rpc_handler().await;

        let server = ServerBuilder::new(rpc_handler)
            .start_http(&self.endpoint.parse().unwrap())
            .expect("JRPC Server failed to start.");

        log_info!("Vanguard Engine JRPC server started on {}", &self.endpoint);

        server.wait();
    }
}
