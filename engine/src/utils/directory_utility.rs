use crate::constants::Constants;
use crate::log_error;
use std::fs::{self};
use std::path::PathBuf;

pub fn get_ssl_upload_path() -> PathBuf {
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

pub fn is_directory_exist(file_path: &PathBuf) -> bool {
    let path = PathBuf::from(file_path);

    if !path.exists() {
        return false;
    }

    let read_metadata_operation = fs::metadata(path);
    if read_metadata_operation.is_err() {
        return false;
    }

    read_metadata_operation.unwrap().is_dir()
}

pub fn create_path(path: &PathBuf) {
    let create_operation = fs::create_dir_all(path);
    if create_operation.is_err() {
        log_error!(
            "Failed to create  directory on: {}",
            path.to_str().unwrap_or_default()
        );
    }
}

pub fn is_path_accessible(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }

    // Try to read metadata to check if the path is accessible
    let metadata_result = fs::metadata(path);
    if metadata_result.is_err() {
        return false;
    }

    // Check if we have read permissions by trying to access the path
    if path.is_dir() {
        // For directories, try to read the directory
        fs::read_dir(path).is_ok()
    } else if path.is_file() {
        // For files, metadata access is sufficient
        true
    } else {
        // Unknown path type
        false
    }
}

pub fn list_directory_content(parent_path: &PathBuf) -> Option<(Vec<String>, Vec<String>)> {
    let mut files = Vec::new();
    let mut directories = Vec::new();

    if !parent_path.is_dir() {
        return None;
    }

    let read_directory_operation = fs::read_dir(parent_path);
    if read_directory_operation.is_err() {
        return None;
    }

    let dir_data = read_directory_operation.ok();
    if dir_data.is_none() {
        return None;
    }

    let dir_entry = dir_data.unwrap();
    for entity in dir_entry {
        if entity.is_ok() {
            let entity_path = entity.unwrap().path();
            let entity_name = entity_path
                .file_name()
                .and_then(|name| name.to_str()) // Convert `OsStr` to `&str`
                .map(|s| s.to_string());

            if entity_name.is_some() {
                if entity_path.is_dir() {
                    directories.push(entity_name.unwrap());
                } else {
                    files.push(entity_name.unwrap());
                }
            }
        }
    }

    Some((files, directories))
}
