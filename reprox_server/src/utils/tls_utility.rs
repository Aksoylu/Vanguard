use rustls::{Certificate, PrivateKey};
use tokio_rustls::rustls::{self, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys}; // Import for handling PEM files

use std::fs::File;
use std::io::{self, BufReader};

pub fn load_certs(filename: &str) -> io::Result<Vec<Certificate>> {
    let certfile = File::open(filename)?;
    let mut reader = BufReader::new(certfile);
    let certs = certs(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))?
        .into_iter()
        .map(Certificate)
        .collect();
    Ok(certs)
}

pub fn load_private_key(filename: &str) -> io::Result<PrivateKey> {
    let keyfile = File::open(filename)?;
    let mut reader = BufReader::new(keyfile);
    let keys = pkcs8_private_keys(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))?;
    Ok(PrivateKey(keys[0].clone()))
}

pub fn configure_tls(certs: Vec<Certificate>, key: PrivateKey) -> io::Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert/key"))?;
    Ok(config)
}