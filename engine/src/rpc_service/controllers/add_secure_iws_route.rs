use crate::models::route::SecureIwsRoute;
use crate::models::ssl_context::SslContext;
use crate::rpc_service::models::add_secure_iws_request::{AddSecureIwsRequest, AddSecureIwsResponse};
use crate::utils::directory_utility::get_ssl_path;
use crate::utils::file_utility::is_file_exist;
use crate::utils::tls_utility::validate_ssl_context;
use crate::boot::Runtime;
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;
use jsonrpc_core::ErrorCode;

pub fn add_secure_iws_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match AddSecureIwsRequest::new(params) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: add_iws".into(),
                data: None,
            });
        }
    };

    let source = request.get_source();
    let serving_path = request.get_serving_path();
    let runtime_snapshot = runtime.lock().unwrap().router.clone();

    /*  If record with source already exist in route or serving path is already used by another IWS route, terminate flow */
    let route_list = runtime_snapshot.list_routes();
    for route in route_list {
        if route.source == source.clone(){
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Route source already registered".into(),
                data: None,
            });
        }

        if route.serving_path.is_some(){
            let each_serving_path = route.serving_path.unwrap_or_default().clone();

            if each_serving_path == serving_path.clone(){
                return Err(Error {
                    code: ErrorCode::InternalError,
                    message: "Route serving path already used by another IWS route".into(),
                    data: None,
                });
            }
        }
    }

    /* Look up ssl cert files for given domain and validate is able to creating SSL Context */
    let new_secure_iws_route = match create_secure_iws(source.clone(), serving_path.clone()) {
        Ok(new_secure_iws_route) => new_secure_iws_route,
        Err(error) => {
            return Err(Error {
                code: error.code,
                message: error.message.into(),
                data: error.data,
            });
        }
    };
    
    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let updated_runtime_snapshot = runtime_snapshot.add_secure_iws_route( new_secure_iws_route);
    runtime.lock().unwrap().router = updated_runtime_snapshot;
    Ok(AddSecureIwsResponse::build("ok".to_string()))
    
}

fn create_secure_iws(source: String, serving_path: String)  -> Result<SecureIwsRoute, Error> {
    let cert_file_name = format!("{}@cert.pem", source);
    let private_key_file_name = format!("{}@privkey.pem", source);

    let ssl_path = get_ssl_path();
    /* Check is related SSL certficate exist on file system */
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

    /* Check is related SSL private key exist on file system  */
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

    let new_secure_iws_record: SecureIwsRoute = SecureIwsRoute {
        source,
        serving_path,

        ssl_context: SslContext {
            cert: cert_file_name,
            private_key: private_key_file_name,
        },
    };

    return Ok(new_secure_iws_record);
}