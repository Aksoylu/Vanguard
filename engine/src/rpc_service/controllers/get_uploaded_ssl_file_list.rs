use jsonrpc_core::{Error, Value};

use crate::models::ssl_file::SSlFile;
use crate::rpc_service::models::get_uploaded_ssl_file_list_response::GetUploadedSslFileListResponse;
use crate::utils::directory_utility::get_ssl_upload_path;
use crate::utils::file_utility::list_all_files;
use crate::utils::tls_utility::detect_file_type;

pub fn get_uploaded_ssl_file_list(_payload: Value) -> Result<Value, Error> {
    let uploaded_ssl_file_path_list: Vec<String> = get_ssl_file_list();

    let mut ssl_entity_vec: Vec<SSlFile> = vec![];
    for each_file_name in uploaded_ssl_file_path_list {
        let certificate_info = read_certificate_info(&each_file_name);
        if certificate_info.is_some() {
            ssl_entity_vec.push(certificate_info.unwrap());
        }
    }

    Ok(GetUploadedSslFileListResponse::build(ssl_entity_vec))
}

fn get_ssl_file_list() -> Vec<String> {
    let ssl_file_upload_path = get_ssl_upload_path();
    let uploaded_ssl_files: Vec<String> = list_all_files(ssl_file_upload_path);

    uploaded_ssl_files
}

fn read_certificate_info(ssl_cert_file_name: &String) -> Option<SSlFile> {
    let seperator = '@';

    let seperator_count = ssl_cert_file_name
        .chars()
        .filter(|&c| c == seperator)
        .count();

    if seperator_count != 1 {
        return None;
    }

    let parts: Vec<&str> = ssl_cert_file_name.split(seperator).collect();

    let domain = parts[0].to_string().to_lowercase();
    let file_name = parts[1].to_string().to_ascii_lowercase();

    let file_type = detect_file_type(file_name);

    Some(SSlFile { domain, file_type })
}
