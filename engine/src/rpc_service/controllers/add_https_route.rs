use crate::models::route::HttpsRoute;
use crate::models::ssl_context::SslContext;
use crate::rpc_service::models::add_https_route_model::{
    AddHttpsRouteRequest, AddHttpsRouteResponse,
};
use crate::runtime::Runtime;
use crate::utils::file_utility::{get_ssl_path, is_file_exist};
use crate::utils::tls_utility::validate_ssl_context;
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

pub fn add_https_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match AddHttpsRouteRequest::new(params) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: add_http_route".into(),
                data: None,
            });
        }
    };

    let source = request.get_source();
    let target = request.get_target();

    let cert_file_name = format!("{}@cert.pem", source);
    let private_key_file_name = format!("{}@privkey.pem", source);

    let ssl_path = get_ssl_path();
    /* Check is SSL certficate exist on file system */
    let mut cert_path = ssl_path.clone();
    cert_path.push(cert_file_name.clone());
    if !is_file_exist(&cert_path) {
        return Err(Error {
            code: ErrorCode::InternalError,
            message: format!(
                "Ssl certificate not found on path: {}",
                cert_path.to_string_lossy()
            )
            .into(),
            data: None,
        });
    }

    /* Check is SSL private key exist on file system  */
    let mut private_key_path = ssl_path.clone();
    private_key_path.push(private_key_file_name.clone());
    if !is_file_exist(&private_key_path) {
        return Err(Error {
            code: ErrorCode::InternalError,
            message: format!(
                "Ssl certificate private key not found on path: {}",
                private_key_path.to_string_lossy()
            )
            .into(),
            data: None,
        });
    }

    /* Check  ssl ceretificate is valid and compatible with given domain & IP address */
    let validate_ssl_context_operation =
        validate_ssl_context(source.clone(), cert_path.clone(), private_key_path.clone());

    if !validate_ssl_context_operation {
        return Err(Error {
            code: ErrorCode::ParseError,
            message: "Ssl cert, private key and given source domain is not compatible ".into(),
            data: None,
        });
    }

    let new_route = HttpsRoute {
        source,
        target,

        ssl_context: SslContext {
            cert: cert_file_name,
            private_key: private_key_file_name,
        },
    };

    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let updated_runtime_snapshot = runtime_snapshot.add_https_route(new_route);

    runtime.lock().unwrap().router = updated_runtime_snapshot;

    Ok(AddHttpsRouteResponse::build("ok".to_string(), None))
}
