use serde::{Deserialize, Serialize};

use crate::models::entity::ssl_file::SSlFile;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUploadedSslFileListResponse {
    pub code: u16,
    pub message: String,
    pub ssl_file_list: Vec<SSlFile>,
}