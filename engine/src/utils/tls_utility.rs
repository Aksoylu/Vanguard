use rustls::server::ResolvesServerCertUsingSni;
use rustls::sign::{CertifiedKey, RsaSigningKey};
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde::{Deserialize, Serialize};
use tokio_rustls::rustls::{self, ServerConfig};
use tokio_rustls::TlsAcceptor;

use crate::models::route::HttpsRoute;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;
use std::sync::Arc;

use super::directory_utility::get_ssl_path;

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
pub fn create_ssl_context(routes: HashMap<std::string::String, HttpsRoute>) -> TlsAcceptor {
    let mut sni_resolver = ResolvesServerCertUsingSni::new();

    let ssl_path = get_ssl_path();

    for (source, https_route) in routes {
        let mut ssl_certificate_path = ssl_path.clone();
        ssl_certificate_path.push(https_route.ssl_context.cert.clone());

        let mut ssl_private_key_path = ssl_path.clone();
        ssl_private_key_path.push(https_route.ssl_context.private_key.clone());

        let load_cert_operation = load_certs(ssl_certificate_path.clone());
        if load_cert_operation.is_err() {
            panic!(
                "Could not found SSL Certificate on path: {} ",
                ssl_certificate_path.to_string_lossy()
            );
        }

        let load_private_key_operation = load_private_key(ssl_private_key_path.clone());
        if load_private_key_operation.is_err() {
            panic!(
                "Could not found SSL Private Key on path: {} ",
                ssl_private_key_path.to_string_lossy()
            );
        }

        let certs: Vec<Certificate> = load_cert_operation.unwrap();
        let key: PrivateKey = load_private_key_operation.unwrap();
        let certified_key = create_certified_key(certs, key);
        sni_resolver.add(source.as_str(), certified_key).unwrap();
    }

    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(sni_resolver));

    TlsAcceptor::from(Arc::new(tls_config))
}

pub fn load_certs(path: PathBuf) -> io::Result<Vec<Certificate>> {
    let certfile = File::open(path)?;
    let mut reader = BufReader::new(certfile);
    let certs = certs(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to load certificate"))?;
    Ok(certs.into_iter().map(Certificate).collect())
}

pub fn load_private_key(path: PathBuf) -> io::Result<PrivateKey> {
    let keyfile = File::open(path)?;
    let mut reader = BufReader::new(keyfile);
    let keys = pkcs8_private_keys(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to load private key"))?;
    Ok(PrivateKey(keys[0].clone()))
}

pub fn validate_ssl_context(
    domain: String,
    certificate_upload_path: PathBuf,
    privatekey_upload_path: PathBuf,
) -> bool {
    let load_cert_operation = load_certs(certificate_upload_path);
    if load_cert_operation.is_err() {
        return false;
    }

    let load_privatekey_operation = load_private_key(privatekey_upload_path);
    if load_privatekey_operation.is_err() {
        return false;
    }

    validate_certificate(
        domain,
        load_cert_operation.unwrap(),
        load_privatekey_operation.unwrap(),
    )
}

pub fn validate_certificate(domain: String, certs: Vec<Certificate>, key: PrivateKey) -> bool {
    let mut sni_resolver = ResolvesServerCertUsingSni::new();
    let certified_key = create_certified_key(certs, key);
    let resolve_ssl_context_operation = sni_resolver.add(domain.as_str(), certified_key);

    resolve_ssl_context_operation.is_ok()
}

pub fn detect_file_type(file_name: String) -> SSlFileType {
    if file_name == "cert.pem" {
        return SSlFileType::PemCertificate;
    } else if file_name == "privkey.pem" {
        return SSlFileType::PemPrivateKey;
    }

    SSlFileType::Invalid
}

fn create_certified_key(certs: Vec<Certificate>, key: PrivateKey) -> CertifiedKey {
    let signing_key = RsaSigningKey::new(&key).unwrap();
    CertifiedKey::new(certs, Arc::new(signing_key))
}
