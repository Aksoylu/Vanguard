use crate::rpc_service::models::upload_ssl_cert_model::{
    UploadSslCertRequest, UploadSslCertResponse,
};

use crate::utils::file_utility::{delete_file, write_file};
use crate::utils::tls_utility::validate_ssl_context;

use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error, Params, Value};
use std::sync::{Arc, Mutex};

pub fn upload_ssl_cert(_runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match UploadSslCertRequest::new(params) {
        Ok(req) => req,
        Err(error) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: upload_ssl_cert".into(),
                data: Some(Value::String(error.message.to_string())),
            });
        }
    };

    let domain_name = request.get_domain().to_lowercase();
    let certificate_file_name = format!("{}@cert.pem", &domain_name);
    let privatekey_file_name = format!("{}@privkey.pem", &domain_name);

    let ssl_upload_path = get_ssl_path();

    let mut certificate_upload_path = ssl_upload_path.clone();
    certificate_upload_path.push(certificate_file_name);

    let mut privatekey_upload_path = ssl_upload_path.clone();
    privatekey_upload_path.push(privatekey_file_name);

    let write_certificate_operation = write_file(
        certificate_upload_path.clone(),
        request.get_raw_certificate().as_str(),
    );
    let write_privatekey_operation = write_file(
        privatekey_upload_path.clone(),
        request.get_raw_privatekey().as_str(),
    );
    if write_certificate_operation.is_err() || write_privatekey_operation.is_err() {
        return Err(Error {
            code: ErrorCode::InternalError,
            message: "Failed write operation on JRPC function: upload_ssl_cert".into(),
            data: None,
        });
    }

    let validate_ssl_context_operation = validate_ssl_context(
        request.get_domain(),
        certificate_upload_path.clone(),
        privatekey_upload_path.clone(),
    );

    if !validate_ssl_context_operation {
        /* Rollback strategy */
        delete_file(certificate_upload_path);
        delete_file(privatekey_upload_path);

        return Err(Error {
            code: ErrorCode::ParseError,
            message: "Invalid ssl context on JRPC function: upload_ssl_cert".into(),
            data: None,
        });
    }

    Ok(UploadSslCertResponse::build("ok".to_string(), None))
}
