use crate::rpc_service::models::delete_ssl_cert_model::{
    DeleteSSlCertRequest, DeleteSSlCertResponse,
};
use crate::runtime::Runtime;
use crate::utils::file_utility::{
    delete_file, get_pathbuf_filename, get_ssl_path, is_file_exist, list_all_files,
};
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Params, Value};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub fn delete_ssl_cert(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match DeleteSSlCertRequest::new(params) {
        Ok(req) => req,
        Err(error) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: delete_ssl_cert".into(),
                data: Some(Value::String(error.message.to_string())),
            });
        }
    };

    let domain_name = request.get_domain().to_lowercase();
    let cert_file_name = format!("{}@cert.pem", domain_name.clone());
    let private_key_file_name = format!("{}@privkey.pem", domain_name.clone());

    /* Do not allow to deletion if ssl cert using by any domain */
    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let all_routes = runtime_snapshot.list_routes();

    for each_route in all_routes {
        if each_route.protocol != "https".to_owned() {
            continue;
        }

        let route_ssl = each_route.ssl.clone();
        if route_ssl.is_none() {
            continue;
        }

        let ssl_context = route_ssl.unwrap();
        if ssl_context.cert == cert_file_name {
            let error_message = format!(
                "Ssl certificate file '{}' is currently used by domain: {}. Please delete related Https route before deleting ssl certificate",
                cert_file_name.clone(), domain_name.clone()
            );

            return Err(Error {
                code: ErrorCode::InternalError,
                message: error_message.into(),
                data: None,
            });
        }

        if ssl_context.private_key == private_key_file_name {
            let error_message = format!(
                "Ssl private key '{}' is currently used by domain: {}. Please delete related Https route before deleting ssl private key",
                private_key_file_name.clone(), domain_name.clone()
            );
        }
    }

    let ssl_path = get_ssl_path();
    let mut deleted_files: Vec<String> = vec![];

    /* Delete if ssl cert file exist */
    let mut cert_file_path = ssl_path.clone();
    cert_file_path.push(cert_file_name.clone());

    if is_file_exist(cert_file_path.clone()) {
        delete_file(cert_file_path);
        deleted_files.push(cert_file_name)
    }

    /* Delete if ssl private key file exist */
    let mut private_key_path = ssl_path.clone();
    private_key_path.push(private_key_file_name.clone());

    if is_file_exist(private_key_path.clone()) {
        delete_file(private_key_path);
        deleted_files.push(private_key_file_name)
    }

    Ok(DeleteSSlCertResponse::build(deleted_files))
}
