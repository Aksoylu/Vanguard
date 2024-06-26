use rustls::{Certificate, PrivateKey};
use tokio_rustls::rustls::{self, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio_rustls::TlsAcceptor; // Import for handling PEM files

use std::fs::File;
use std::io::{self, BufReader};
use std::sync::Arc;

use crate::core::models::SslPath;


pub fn create_ssl_context(optional_ssl_path:Option<SslPath>) -> TlsAcceptor{
    if optional_ssl_path.is_some() == false {
        eprintln!("Error: Https protocol needs SSL certificate and private key of it");
    }

    let ssl_path = optional_ssl_path.unwrap();


    match (load_certs("cert.pem"), load_private_key("key.pem")) {
        (Ok(certs), Ok(key)) => {
            let tls_config = configure_tls(certs, key).unwrap();
            let tls_context = TlsAcceptor::from(Arc::new(tls_config));
            
            tls_context
        }
        (Err(e), _) | (_, Err(e)) => {
            panic!("Failed to load certificates or private key on path:\n{}\n{}", ssl_path.cert, ssl_path.private_key);
        }
    }
} 


fn load_certs(filename: &str) -> io::Result<Vec<Certificate>> {
    let certfile = File::open(filename)?;
    let mut reader = BufReader::new(certfile);
    let certs = certs(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))?
        .into_iter()
        .map(Certificate)
        .collect();
    Ok(certs)
}

fn load_private_key(filename: &str) -> io::Result<PrivateKey> {
    let keyfile = File::open(filename)?;
    let mut reader = BufReader::new(keyfile);
    let keys = pkcs8_private_keys(&mut reader)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))?;
    Ok(PrivateKey(keys[0].clone()))
}

fn configure_tls(certs: Vec<Certificate>, key: PrivateKey) -> io::Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert/key"))?;
    Ok(config)
}