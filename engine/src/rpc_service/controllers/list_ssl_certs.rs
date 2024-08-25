use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;

use crate::rpc_service::models::list_ssl_cert_model::{ListSslCertResponse, SslCertEntity};
use crate::runtime::Runtime;
use crate::utils::file_utility::{get_ssl_path, list_all_files};
use crate::utils::tls_utility::detect_file_type;

pub fn list_ssl_certs(runtime: Arc<Mutex<Runtime>>, _params: Params) -> Result<Value, Error> {
    let ssl_file_vec = get_ssl_file_vec();

    Ok(ListSslCertResponse::build(ssl_file_vec))
}

fn get_ssl_file_vec() -> Vec<SslCertEntity> {
    let ssl_path = get_ssl_path();
    let all_files = list_all_files(ssl_path);

    let seperator = '@';

    let mut ssl_cert_entity_vec: Vec<SslCertEntity> = vec![];
    for each_file_name in all_files {
        let seperator_count = each_file_name.chars().filter(|&c| c == seperator).count();
        if seperator_count != 1 {
            continue;
        }

        let parts: Vec<&str> = each_file_name.split(seperator).collect();
        let domain_name = parts[0].to_string().to_lowercase();
        let file_name = parts[1].to_string().to_ascii_lowercase();

        if domain_name.is_empty() || file_name.is_empty() {
            continue;
        }

        append_entity(&mut ssl_cert_entity_vec, domain_name, file_name);
    }

    ssl_cert_entity_vec
}

fn append_entity(ssl_files: &mut Vec<SslCertEntity>, domain_name: String, file_name: String) {
    let file_type = detect_file_type(file_name);

    let mut is_entity_exist = false;

    for each_ssl_entity in ssl_files.into_iter() {
        if each_ssl_entity.domain == domain_name {
            each_ssl_entity.files.push(file_type.clone());
            is_entity_exist = true;
            break;
        }
    }

    if !is_entity_exist {
        let new_entity = SslCertEntity {
            domain: domain_name.clone(),
            files: vec![file_type],
        };

        ssl_files.push(new_entity);
    }
}
