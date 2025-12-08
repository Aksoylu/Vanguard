use jsonrpc_core::Error;
use rustls::server::ResolvesServerCertUsingSni;
use rustls::sign::{CertifiedKey, RsaSigningKey};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::{Deserialize, Serialize};
use tokio_rustls::rustls::{self, ServerConfig};
use tokio_rustls::TlsAcceptor;

use crate::models::route::{HttpsRoute, SecureIwsRoute};
use crate::utils::file_utility::{delete_file, get_absolute_ssl_file_path};
use jsonrpc_core::ErrorCode;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum SSlFileType {
    Invalid,
    PemCertificate,
    PemPrivateKey,
}

pub fn get_certificate_type(content: String) -> SSlFileType {
    let lines: Vec<&str> = content.lines().collect();

    if lines.len() < 5 {
        return SSlFileType::Invalid;
    }

    let first_line = lines.first().unwrap().trim().to_string();
    let last_line = lines.last().unwrap().trim().to_string();

    if first_line.trim() == "-----BEGIN CERTIFICATE-----"
        && last_line == "-----END CERTIFICATE-----"
    {
        return SSlFileType::PemCertificate;
    }

    if first_line == "-----BEGIN PRIVATE KEY-----" && last_line == "-----END PRIVATE KEY-----" {
        return SSlFileType::PemPrivateKey;
    }

    SSlFileType::Invalid
}

pub fn create_ssl_context(
    https_routes: HashMap<String, HttpsRoute>,
    secure_iws_routes: HashMap<String, SecureIwsRoute>,
) -> TlsAcceptor {
    let mut sni_resolver = ResolvesServerCertUsingSni::new();

    /* Loop for creating sni resolving for all https routes */
    for (source, https_route) in https_routes {
        // Load certificate
        let cert_file_path = &https_route.ssl_context.certificate_file_path;
        let ssl_cert_list = match load_ssl_certs(cert_file_path) {
            Ok(c) => c,
            Err(err) => {
                panic!(
                    "An error occurred while loading SSL certificate for '{}': {}",
                    source, err.message
                );
            }
        };

        // Load private key
        let private_key_file_path = &https_route.ssl_context.private_key_file_path;
        let private_key = match load_ssl_private_key(private_key_file_path) {
            Ok(k) => k,
            Err(err) => {
                panic!(
                    "An error occurred while loading SSL private key for '{}' Https Route: {}",
                    source, err.message
                );
            }
        };

        let certified_key = create_certified_key(ssl_cert_list, private_key);
        sni_resolver.add(source.as_str(), certified_key).unwrap();
    }

    /* Loop for creating sni resolving for all secure IWS routes */
    for (source, secure_iws_route) in secure_iws_routes {
        // Load certificate
        let cert_file_path = &secure_iws_route.ssl_context.certificate_file_path;
        let ssl_cert_list = match load_ssl_certs(cert_file_path) {
            Ok(c) => c,
            Err(err) => {
                panic!(
                    "An error occurred while loading SSL certificate for '{}' Secure IWS Route: {}",
                    source, err.message
                );
            }
        };

        // Load private key
        let private_key_file_path = &secure_iws_route.ssl_context.private_key_file_path;
        let private_key = match load_ssl_private_key(private_key_file_path) {
            Ok(k) => k,
            Err(err) => {
                panic!(
                    "An error occurred while loading SSL private key for '{}': {}",
                    source, err.message
                );
            }
        };

        let certified_key = create_certified_key(ssl_cert_list, private_key);
        sni_resolver.add(source.as_str(), certified_key).unwrap();
    }

    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(sni_resolver));

    TlsAcceptor::from(Arc::new(tls_config))
}

pub fn validate_ssl_context(
    domain: &String,
    ssl_cert_path: &String,
    ssl_private_key_path: &String,
) -> Result<(), Error> {
    let ssl_cert_list = load_ssl_certs(ssl_cert_path)?;
    let private_key = load_ssl_private_key(ssl_private_key_path)?;

    let mut sni_resolver = ResolvesServerCertUsingSni::new();
    let certified_key = create_certified_key(ssl_cert_list, private_key);

    sni_resolver
        .add(domain.as_str(), certified_key)
        .map_err(|error_body| Error {
            code: jsonrpc_core::ErrorCode::InternalError,
            message: error_body.to_string(),
            data: None,
        })?;

    Ok(())
}

pub fn detect_file_type(file_name: String) -> SSlFileType {
    if file_name == "cert.pem" {
        return SSlFileType::PemCertificate;
    } else if file_name == "privkey.pem" {
        return SSlFileType::PemPrivateKey;
    }

    SSlFileType::Invalid
}

pub fn delete_ssl_file(file_path: &String) -> Result<bool, Error>{
    let absolute_file_path = get_absolute_ssl_file_path(file_path)?;
    let is_success = delete_file(absolute_file_path);
    Ok(is_success)
}

fn create_certified_key(certs: Vec<Certificate>, key: PrivateKey) -> CertifiedKey {
    let signing_key = RsaSigningKey::new(&key).unwrap();
    CertifiedKey::new(certs, Arc::new(signing_key))
}

fn load_ssl_certs(certificate_file_path: &String) -> Result<Vec<Certificate>, Error> {
    let absolute_cert_file_path = get_absolute_ssl_file_path(certificate_file_path)?;
    let readed_file = File::open(&absolute_cert_file_path).map_err(|_| Error {
        code: jsonrpc_core::ErrorCode::InternalError,
        message: format!(
            "File not found at path '{}'",
            &absolute_cert_file_path.to_string_lossy()
        ),
        data: None,
    })?;

    let mut reader = BufReader::new(readed_file);

    let certs = certs(&mut reader).map_err(|_| Error {
        code: ErrorCode::InternalError,
        message: format!(
            "Failed to load certificate from path '{}'",
            absolute_cert_file_path.to_string_lossy()
        ),
        data: None,
    })?;

    Ok(certs.into_iter().map(Certificate).collect())
}

fn load_ssl_private_key(private_key_file_path: &String) -> Result<PrivateKey, Error> {
    let absolute_private_key_file_path = get_absolute_ssl_file_path(private_key_file_path)?;

    let readed_file = File::open(&absolute_private_key_file_path).map_err(|_| Error {
        code: jsonrpc_core::ErrorCode::InternalError,
        message: format!(
            "File not found at path '{}'",
            &absolute_private_key_file_path.to_string_lossy()
        ),
        data: None,
    })?;

    let mut reader = BufReader::new(readed_file);

    let private_keys = pkcs8_private_keys(&mut reader).map_err(|_| Error {
        code: jsonrpc_core::ErrorCode::InternalError,
        message: format!(
            "Failed to load private key from path '{}'",
            &absolute_private_key_file_path.to_string_lossy()
        ),
        data: None,
    })?;

    let primary_private_key_as_binary = private_keys[0].clone();

    Ok(PrivateKey(primary_private_key_as_binary))
}
