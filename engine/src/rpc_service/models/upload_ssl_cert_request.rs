use crate::rpc_service::rpc_error::RPCError;
use crate::utils::base64_utility;
use crate::utils::rpc_utility::RpcParameter;
use crate::utils::tls_utility::get_certificate_type;
use crate::utils::tls_utility::SSlFileType;

use hyper::StatusCode;
use jsonrpc_core::{Error, Value};

pub struct UploadSslCertRequest {
    domain: String,
    raw_certificate: String,
    raw_privatekey: String,
}

impl UploadSslCertRequest {
    pub fn new(params: Value) -> Result<Self, Error> {
        let _domain: Option<String> = RpcParameter::extract_string("domain", &params);
        let _certificate = RpcParameter::extract_string("certificate", &params);
        let _privatekey = RpcParameter::extract_string("privatekey", &params);

        if _domain.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'domain' parameter",
            ));
        }

        if _certificate.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'certificate' (Valid SSL certificate) as parameter",
            ));
        }

        if _privatekey.is_none() {
            return Err(RPCError::build(
                &StatusCode::BAD_REQUEST,
                "Please provide 'privatekey' (Valid SSL private key) as parameter",
            ));
        }

        let domain = _domain.unwrap();
        // Note that given certificate and private key given as Base64 encoded
        // because JRPC doesnt support file upload
        let decoded_certificate = base64_utility::decode_b64(_certificate.unwrap());
        let decoded_privatekey = base64_utility::decode_b64(_privatekey.unwrap());

        if decoded_certificate.is_none() || decoded_privatekey.is_none() {
            return Err(RPCError::build(
                &StatusCode::INTERNAL_SERVER_ERROR,
                "Please provide Base64 encoded SSL certificate and private key",
            ));
        }

        let certificate = decoded_certificate.unwrap();
        let privatekey = decoded_privatekey.unwrap();

        if get_certificate_type(certificate.clone()) != SSlFileType::PemCertificate {
            return Err(RPCError::build(
                &StatusCode::METHOD_NOT_ALLOWED,
                "Given 'certificate' parameter content doesn't represent a valid SSL certificate",
            ));
        }

        if get_certificate_type(privatekey.clone()) != SSlFileType::PemPrivateKey {
            return Err(RPCError::build(
                &StatusCode::METHOD_NOT_ALLOWED,
                "Given  doesn't represent a valid SSL certificate private Key",
            ));
        }

        Ok(Self {
            domain,
            raw_certificate: certificate,
            raw_privatekey: privatekey,
        })
    }

    // getters
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    pub fn get_raw_certificate(&self) -> String {
        self.raw_certificate.clone()
    }

    pub fn get_raw_privatekey(&self) -> String {
        self.raw_privatekey.clone()
    }
}
