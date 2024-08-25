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

pub fn delete_ssl_cert(_runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
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
    let domain_related_files = get_domain_related_files(domain_name);

    let mut deleted_files: Vec<String> = vec![];

    for each_file in domain_related_files {
        if is_file_exist(each_file.clone()) {
            let is_deletion_success = delete_file(each_file.clone());
            if !is_deletion_success {
                continue;
            }

            let filename = get_pathbuf_filename(each_file);
            if filename.is_none() {
                continue;
            }

            deleted_files.push(filename.unwrap())
        }
    }

    Ok(DeleteSSlCertResponse::build(deleted_files))
}

/// @TODO: Its logic should be changed for detecting related files from runtime.
/// Else it may delete subdomain ssl files when a domain called. Because current logic
/// searches out all files with matching substring pattern.
/// In runtime, we've already keep 'accurate and absolute ssl file path' for each domain. 
/// This function should do it by using them 
fn get_domain_related_files(domain_name: String) -> Vec<PathBuf> {
    let ssl_path = get_ssl_path();
    let all_file_names: Vec<String> = list_all_files(ssl_path.clone());

    let related_file_paths: Vec<PathBuf> = vec![];

    for each_file_name in all_file_names {
        if each_file_name.contains(domain_name.as_str()) {
            let mut related_file_path = ssl_path.clone();
            related_file_path.push(each_file_name);
        }
    }

    related_file_paths
}
