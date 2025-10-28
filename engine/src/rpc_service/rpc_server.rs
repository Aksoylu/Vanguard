use jsonrpc_core::IoHandler;
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::routes::ROUTES;
use crate::rpc_service::rpc_middleware::RpcMiddleware;
use crate::{log_info, runtime::Runtime, utils::network_utility::parse_ip_address};

use jsonrpc_http_server::ServerBuilder;

pub struct RPCServer {
    ip_address: String,
    port: u16,
    auth_token: String,
    endpoint: String,
    rpc_handler: IoHandler,
    runtime: Arc<Mutex<Runtime>>,
}

impl RPCServer {
    pub async fn singleton(
        ip_address: String,
        port: u16,
        auth_token: String,
        runtime: Arc<Mutex<Runtime>>,
    ) -> Self {
        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let endpoint = format!("{}:{}", parsed_ip_address, port);

        let rpc_handler = RPCServer::build_rpc_handler(runtime.clone()).await;

        Self {
            ip_address,
            port,
            auth_token,
            endpoint,
            rpc_handler,
            runtime,
        }
    }

    pub async fn build_rpc_handler(runtime: Arc<Mutex<Runtime>>) -> IoHandler {
        let mut controller_registry = IoHandler::default();

        let authorization_code = runtime.lock().unwrap().rpc_session.hash.clone();
        for (service_name, controller_delegate) in ROUTES.iter() {
            controller_registry.add_method(
                service_name,
                RpcMiddleware::bind(
                    controller_delegate.clone(),
                    runtime.clone(),
                    authorization_code.clone(),
                ),
            );
        }

        controller_registry
    }

    /// Public: This function is repsonsible of booting process of  JRPC Server
    pub async fn start(&self) {
        let server = ServerBuilder::new(self.rpc_handler.clone())
            .start_http(&self.endpoint.parse().unwrap())
            .expect("JRPC Server failed to start.");

        log_info!("Vanguard Engine JRPC server started on {}", &self.endpoint);

        server.wait();
    }
}
