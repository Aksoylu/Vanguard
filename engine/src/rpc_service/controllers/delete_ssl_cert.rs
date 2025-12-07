use jsonrpc_core::{Error, Value};

use crate::core::shared_memory::ROUTER;
use crate::rpc_service::models::delete_ssl_cert_request::DeleteSSlCertRequest;
use crate::rpc_service::models::delete_ssl_cert_response::DeleteSSlCertResponse;

use crate::utils::file_utility::{delete_file, get_absolute_ssl_file_path, is_file_exist};
use jsonrpc_core::ErrorCode;

pub fn delete_ssl_cert(params: Value) -> Result<Value, Error> {
    let request = DeleteSSlCertRequest::new(params)?;
    let domain = request.get_domain();

    let domain_name = request.get_domain().to_lowercase();

    check_domain_usage(&domain_name)?;

    let cert_relative_path = format!("@vanguard/{}@cert.pem", &domain_name);
    let private_key_relative_path = format!("{}@privkey.pem", &domain_name);

    let cert_absolute_path = get_absolute_ssl_file_path(&cert_relative_path)?;
    let private_key_absolute_path = get_absolute_ssl_file_path(&private_key_relative_path)?;

    let mut deleted_file_path_vec: Vec<String> = vec![];

    if is_file_exist(&cert_absolute_path) {
        delete_file(cert_absolute_path.clone());
        deleted_file_path_vec.push(cert_absolute_path.to_string_lossy().to_string())
    }

    if is_file_exist(&private_key_absolute_path) {
        delete_file(private_key_absolute_path.clone());
        deleted_file_path_vec.push(private_key_absolute_path.to_string_lossy().to_string())
    }

    Ok(DeleteSSlCertResponse::build(deleted_file_path_vec))
}

fn check_domain_usage(domain_name: &String) -> Result<(), Error> {
    let router = ROUTER.read().unwrap();
    let all_routes = router.list_routes();

    /* Do not allow to deletion if ssl cert using by any domain */
    for each_route in all_routes {
        if each_route.protocol != "https".to_owned() {
            continue;
        }

        if each_route.source != domain_name.to_owned() {
            continue;
        }

        let route_ssl = each_route.ssl.clone();
        if route_ssl.is_none() {
            continue;
        }

        let cert_absolute_path = get_absolute_ssl_file_path(&format!("@vanguard/{}@cert.pem", &domain_name))?;
        let private_key_absolute_path = get_absolute_ssl_file_path(&format!("@vanguard/{}@privkey.pem", &domain_name))?;

        let mut error_message = format!("Given domain currently used by route with same name. Please ensure that you removed all routes before deleting SSL asset");

        if is_file_exist(&cert_absolute_path) {
            error_message = format!(
                "Ssl certificate file '{}' is currently used by domain: {}. Please delete related {} route before deleting ssl certificate",
                cert_absolute_path.to_string_lossy(),
                &domain_name,
                &each_route.protocol
            );
        }

        if is_file_exist(&private_key_absolute_path) {
            error_message = format!(
                "Ssl private key file '{}' is currently used by domain: {}. Please remove related {} route before deleting ssl certificate",
                private_key_absolute_path.to_string_lossy(),
                &domain_name,
                &each_route.protocol
            );
        }

        return Err(Error {
            code: ErrorCode::InternalError,
            message: error_message.into(),
            data: None,
        });
    }

    Ok(())
}
