use crate::utils::base64_utility;
use crate::utils::rpc_utility::RpcParameter;
use crate::utils::tls_utility::get_certificate_type;
use crate::utils::tls_utility::SSlFileType;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;

pub struct UploadSslCertRequest {
    domain: String,
    raw_certificate: String,
    raw_privatekey: String,
}

impl UploadSslCertRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let _domain: Option<String> = RpcParameter::extract_string("domain", params.clone());
        let _raw_certificate = RpcParameter::extract_string("raw_certificate", params.clone());
        let _raw_privatekey = RpcParameter::extract_string("raw_privatekey", params.clone());

        if _domain.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Given domain is not valid".into(),
                data: None,
            });
        }

        if _raw_certificate.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Given file doesn't represent a valid cert or private key".into(),
                data: None,
            });
        }

        if _raw_privatekey.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Given file doesn't represent a valid cert or private key".into(),
                data: None,
            });
        }

        let domain = _domain.unwrap();
        let base64_encrypted_certificate = _raw_certificate.unwrap();
        let base64_encrypted_privatekey = _raw_privatekey.unwrap();

        let certificate = base64_utility::decode_b64(base64_encrypted_certificate);
        let privatekey = base64_utility::decode_b64(base64_encrypted_privatekey);

        if certificate.is_none() || privatekey.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Please encode your certificate and private key with base64".into(),
                data: None,
            });
        }

        let raw_certificate = certificate.unwrap();
        let raw_privatekey = privatekey.unwrap();

        if get_certificate_type(raw_certificate.clone()) != SSlFileType::PemCertificate {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Given file doesn't represent a valid SSL Certificate".into(),
                data: None,
            });
        }

        if get_certificate_type(raw_privatekey.clone()) != SSlFileType::PemPrivateKey {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Given file doesn't represent a valid SSL Certificate Private Key".into(),
                data: None,
            });
        }

        Ok(Self {
            domain,
            raw_certificate,
            raw_privatekey,
        })
    }

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

#[derive(Serialize, Deserialize)]
pub struct UploadSslCertResponse {
    code: i32,
    message: String,
    data: Option<Value>,
}

impl UploadSslCertResponse {
    pub fn build(message: String, data: Option<Value>) -> jsonrpc_core::Value {
        let response = UploadSslCertResponse {
            code: 200,
            message,
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
