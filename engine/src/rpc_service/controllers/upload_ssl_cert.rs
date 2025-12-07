use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Value};

use crate::rpc_service::models::upload_ssl_cert_request::UploadSslCertRequest;
use crate::rpc_service::models::upload_ssl_cert_response::UploadSslCertResponse;
use crate::utils::file_utility::{get_absolute_ssl_file_path, write_file};
use crate::utils::tls_utility::{delete_ssl_file, validate_ssl_context};

pub fn upload_ssl_cert(params: Value) -> Result<Value, Error> {
    let request = UploadSslCertRequest::new(params)?;

    let domain_name = request.get_domain().to_lowercase();
    let raw_ssl_certificate = request.get_raw_certificate();
    let raw_ssl_private_key = request.get_raw_privatekey();

    let (uploaded_cert_path, uploaded_private_key_path) =
        upload_ssl_files(&domain_name, &raw_ssl_certificate, &raw_ssl_private_key)?;

    let validate_ssl_context_operation = validate_ssl_context(
        &domain_name,
        &uploaded_cert_path,
        &uploaded_private_key_path,
    );

    // Rollback strategy
    if validate_ssl_context_operation.is_err() {
        delete_ssl_file(&uploaded_cert_path)?;
        delete_ssl_file(&uploaded_private_key_path)?;

        return Err(Error {
            code: ErrorCode::ParseError,
            message: "Invalid ssl context on JRPC function: upload_ssl_cert".into(),
            data: None,
        });
    }

    Ok(UploadSslCertResponse::build("ok".to_string(), None))
}

fn upload_ssl_files(
    domain_name: &String,
    raw_ssl_certificate: &String,
    raw_ssl_private_key: &String,
) -> Result<(String, String), Error> {
    let upload_cert_path = format!("@vanguard/{}@cert.pem", domain_name);
    let upload_private_key_path = format!("@vanguard/{}@privkey.pem", domain_name);

    let cert_absolute_path = get_absolute_ssl_file_path(&upload_cert_path)?;
    let private_key_absolute_path = get_absolute_ssl_file_path(&upload_private_key_path)?;

    let write_certificate_operation =
        write_file(cert_absolute_path.clone(), raw_ssl_certificate.as_str());

    if write_certificate_operation.is_err() {
        return Err(Error {
            code: ErrorCode::InternalError,
            message: format!(
                "Failed to write ssl certificate on path: {}",
                &cert_absolute_path.to_string_lossy()
            ),
            data: None,
        });
    }

    let write_privatekey_operation = write_file(
        private_key_absolute_path.clone(),
        raw_ssl_private_key.as_str(),
    );

    if write_privatekey_operation.is_err() {
        return Err(Error {
            code: ErrorCode::InternalError,
            message: format!(
                "Failed to write ssl private key  on path: {}",
                &private_key_absolute_path.to_string_lossy()
            ),
            data: None,
        });
    }

    Ok((upload_cert_path, upload_private_key_path))
}
