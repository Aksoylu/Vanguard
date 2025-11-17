use jsonrpc_core::IoHandler;

use crate::core::rpc_session::RpcSession;
use crate::rpc_service::routes::ROUTES;
use crate::rpc_service::rpc_middleware::RpcMiddleware;
use crate::{log_info, utils::network_utility::parse_ip_address};

use jsonrpc_http_server::ServerBuilder;

#[derive(Clone)]
pub struct RPCServer {
    pub rpc_session: RpcSession,
}

impl Default for RPCServer {
    fn default() -> Self {
        let default_rpc_session = RpcSession::default();
        Self {
            rpc_session: default_rpc_session,
        }
    }
}

impl RPCServer {
    pub fn init(rpc_session: RpcSession) -> Self {
        Self { rpc_session }
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
                    aes_encryption_key.clone(),
                    authorization_token.clone(),
                ),
            );
        }

        controller_registry
    }

    /// Public: This function is repsonsible of booting process of  JRPC Server
    pub async fn start(&self) {
        let rpc_handler = self.build_rpc_handler().await;

        let parsed_ip_address = parse_ip_address(self.rpc_session.ip_addr.clone());
        let endpoint = format!("{}:{}", parsed_ip_address, self.rpc_session.port);

        let server = ServerBuilder::new(rpc_handler)
            .start_http(&endpoint.parse().unwrap())
            .expect("JRPC Server failed to start.");

        log_info!("Vanguard Engine JRPC server started on {}", &endpoint);

        server.wait();
    }
}
