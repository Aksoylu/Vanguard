use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::constants::Constants;

pub fn get_ssl_path() -> PathBuf {
    let mut ssl_path = get_runtime_path();
    ssl_path.push("SSL");

    if !ssl_path.exists() {
        create_path(&ssl_path);
    }

    ssl_path
}

pub fn get_runtime_path() -> PathBuf {
    let path = if cfg!(target_os = "windows") {
        PathBuf::from(Constants::WIN_RUNTIME_PATH)
    } else if cfg!(target_os = "macos") {
        let mut path = dirs::home_dir().unwrap();
        path.push(Constants::OSX_RUNTIME_PATH);
        path
    } else {
        PathBuf::from(Constants::LINUX_RUNTIME_PATH)
    };

    if !path.exists() {
        create_path(&path);
    }

    path
}

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

pub fn update_runtime_file() {}

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

pub fn is_file_exist(file_path: PathBuf) -> bool{
    let path = PathBuf::from(file_path);
    path.exists()
}

pub fn get_pathbuf_filename(file_path: PathBuf) -> Option<String>{
    let get_filename_operation = file_path.file_name();

    if get_filename_operation.is_none(){
        return None
    }

    let filename = get_filename_operation.unwrap().to_string_lossy();
    
    Some(filename.to_lowercase())
}

fn create_path(path: &PathBuf) {
    let create_operation = fs::create_dir_all(path);
    if create_operation.is_err() {
        panic!(
            "Failed to create  directory on: {}",
            path.to_str().unwrap_or_default()
        );
    }
}
