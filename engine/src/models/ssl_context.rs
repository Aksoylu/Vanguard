use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct SslContext {
    pub certificate_file_path: String,
    pub private_key_file_path: String,
}
