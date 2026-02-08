use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

use crate::{
    core::{
        connection_manager::ConnectionManager, http_proxy_client::HttpProxyClient,
        http_server::HttpServer, https_server::HttpsServer, log_service::LogService,
        router::Router
    },
    models::{boot_result::BootResult, shutdown_signal::ShutdownSignal},
    rpc_service::rpc_server::RPCServer,
};

// Here we store static global instances for engine-wide and multithread read-write access
pub static HTTP_SERVER: Lazy<Arc<RwLock<HttpServer>>> =
    Lazy::new(|| Arc::new(RwLock::new(HttpServer::default())));

pub static LOGGER: Lazy<Arc<RwLock<LogService>>> =
    Lazy::new(|| Arc::new(RwLock::new(LogService::default())));

pub static HTTPS_SERVER: Lazy<Arc<RwLock<HttpsServer>>> =
    Lazy::new(|| Arc::new(RwLock::new(HttpsServer::default())));

pub static RPC_SERVER: Lazy<Arc<RwLock<RPCServer>>> =
    Lazy::new(|| Arc::new(RwLock::new(RPCServer::default())));

pub static RUNTIME_BOOT_INFO: Lazy<Arc<RwLock<BootResult>>> =
    Lazy::new(|| Arc::new(RwLock::new(BootResult::default())));

pub static ROUTER: Lazy<Arc<RwLock<Router>>> =
    Lazy::new(|| Arc::new(RwLock::new(Router::default())));

pub static CONNECTION_MANAGER: Lazy<ConnectionManager> = Lazy::new(|| ConnectionManager::default());

pub static HTTP_CLIENT: Lazy<HttpProxyClient> = Lazy::new(|| HttpProxyClient::default());

pub static SHUTDOWN_SIGNAL: Lazy<ShutdownSignal> = Lazy::new(|| ShutdownSignal::new());
