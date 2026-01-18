use serde::{Deserialize, Serialize};

/// Enum representing the type of an SSL file.
#[derive(PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum SSlFileType {
    Invalid,
    PemCertificate,
    PemPrivateKey,
}