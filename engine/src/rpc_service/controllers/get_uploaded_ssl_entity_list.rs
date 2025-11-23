use jsonrpc_core::{Error, Value};

use crate::models::ssl_entity::SslEntity;
use crate::rpc_service::models::get_uploaded_ssl_entity_list_response::GetUplodadedSslEntityListResponse;
use crate::utils::directory_utility::get_ssl_upload_path;
use crate::utils::file_utility::list_all_files;
use crate::utils::tls_utility::detect_file_type;

pub fn get_uploaded_ssl_entity_list(_params: Value) -> Result<Value, Error> {
    let uploaded_ssl_file_path_list: Vec<String> = get_uploaded_ssl_file_list();

    let mut ssl_entity_vec: Vec<SslEntity> = vec![];
    for each_file_name in uploaded_ssl_file_path_list {
        let certificate_info = read_certificate_info(&each_file_name);
        if certificate_info.is_some() {
            ssl_entity_vec.push(certificate_info.unwrap());
        }
    }

    Ok(GetUplodadedSslEntityListResponse::build(ssl_entity_vec))
}

fn get_uploaded_ssl_file_list() -> Vec<String> {
    let ssl_file_upload_path = get_ssl_upload_path();
    let uploaded_ssl_files: Vec<String> = list_all_files(ssl_file_upload_path);

    uploaded_ssl_files
}

fn read_certificate_info(ssl_cert_file_name: &String) -> Option<SslEntity> {
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

    Some(SslEntity {
        domain,
        file_type,
    })
}
