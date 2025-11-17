use hyper::StatusCode;
use jsonrpc_core::Error;
use mime_guess::{from_path, Mime};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::utils::directory_utility::get_ssl_upload_path;

pub fn load_json<T>(file_path: &Path) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: T = serde_json::from_str(&contents)?;
    Ok(data)
}

pub fn save_json<T>(file_path: &PathBuf, data: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

/// Writes the given content to a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file where the content will be written.
/// * `content` - The content to write to the file.
///
/// # Returns
///
/// * `Ok(())` on success.
/// * `Err(io::Error)` if an error occurs during the write process.
pub fn write_file(file_path: PathBuf, content: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn delete_file(file_path: PathBuf) -> bool {
    if !file_path.exists() {
        return false;
    }

    let delete_operation = fs::remove_file(&file_path);
    delete_operation.is_ok()
}

pub fn list_all_files(parent_path: PathBuf) -> Vec<String> {
    let mut file_names = Vec::new();

    if let Ok(entries) = fs::read_dir(parent_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            file_names.push(file_name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    file_names
}

pub fn is_file_exist(file_path: &PathBuf) -> bool {
    let path = PathBuf::from(file_path);

    if !path.exists() {
        return false;
    }

    let read_metadata_operation = fs::metadata(path);
    if read_metadata_operation.is_err() {
        return false;
    }

    read_metadata_operation.unwrap().is_file()
}

pub async fn read_file_as_binary(file_path: &PathBuf) -> Option<Vec<u8>> {
    let file = File::open(file_path);
    if file.is_err() {
        return None;
    }

    let mut hex_content: Vec<u8> = vec![];

    let read_operation = file.unwrap().read_to_end(&mut hex_content);
    if read_operation.is_err() {
        return None;
    }

    return Some(hex_content);
}

/// Detects a MIME type by file extension.
/// MIME type means http response type that sent to server. If could not detect, returns "application/octet-stream" as default
/// # Returns
///
/// * `Mime` on success.
pub fn get_content_type(file_path: &PathBuf) -> Mime {
    from_path(file_path).first_or_octet_stream()
}

/// Checks given string and condiers it as path.
/// If starts with @vanguard, replaces it with Vanguard's current upload path so it allows user to specify
/// relative path for extending GUI & CLI experience.
/// Else directly uses it as absolute path
/// At the end, checks that is file name exist, else throws RPCError
pub fn get_absolute_ssl_file_path(file_path_as_string: &String) -> Result<PathBuf, Error> {
    let mut absolute_path = PathBuf::new();

    if file_path_as_string.starts_with("@vanguard") {
        let vanguard_relative_path = file_path_as_string.replace("@vanguard", "");

        absolute_path = get_ssl_upload_path();
        absolute_path.push(vanguard_relative_path);
    } else {
        absolute_path = PathBuf::from(file_path_as_string)
    }

    if is_file_exist(&absolute_path) {
        Ok(absolute_path.clone())
    } else {
        return Err(Error {
            code: jsonrpc_core::ErrorCode::InternalError,
            message: format!(
                "File not found at path '{}'",
                absolute_path.to_string_lossy()
            ),
            data: None,
        });
    }
}
