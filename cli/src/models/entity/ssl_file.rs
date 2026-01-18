use serde::Deserialize;
use serde::Serialize;

use crate::common::enums::ssl_file_type::SSlFileType;

/// Domain - SSL File Type pair for crud operations on @Vanguard path's ssl file item list
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SSlFile {
    pub domain: String,
    pub file_type: SSlFileType,
}
