use serde::Deserialize;
use serde::Serialize;

use crate::utils::tls_utility::SSlFileType;

/// Domain - SSL File Type pair for crud operations on @Vanguard path's ssl entity item list
#[derive(Serialize, Deserialize, Clone)]
pub struct SslEntity {
    pub domain: String,
    pub file_type: SSlFileType,
}
