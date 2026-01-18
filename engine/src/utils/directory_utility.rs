use crate::constants::Constants;
use crate::log_error;
use std::fs;
use std::path::{Path, PathBuf};

/// Returns the path to the SSL upload directory.
///
/// Creates the directory if it does not exist.
///
/// # Returns
///
/// * `PathBuf` pointing to the SSL directory.
pub fn get_ssl_upload_path() -> PathBuf {
    let mut ssl_path = get_runtime_path();
    ssl_path.push("SSL");

    if !ssl_path.exists() {
        create_path(&ssl_path);
    }

    ssl_path
}

/// Returns the runtime path for the application based on the operating system.
///
/// Creates the directory if it does not exist.
///
/// # Returns
///
/// * `PathBuf` pointing to the runtime directory.
pub fn get_runtime_path() -> PathBuf {
    let path = if cfg!(target_os = "windows") {
        PathBuf::from(Constants::WIN_RUNTIME_PATH)
    } else if cfg!(target_os = "macos") {
        dirs::home_dir()
            .unwrap_or_default()
            .join(Constants::OSX_RUNTIME_PATH)
    } else {
        PathBuf::from(Constants::LINUX_RUNTIME_PATH)
    };

    if !path.exists() {
        create_path(&path);
    }

    path
}

/// Checks if a path exists and is a directory.
///
/// # Arguments
///
/// * `file_path` - The path to check.
///
/// # Returns
///
/// * `true` if it exists and is a directory, `false` otherwise.
pub fn is_directory_exist(file_path: &PathBuf) -> bool {
    file_path.is_dir()
}

/// Creates a directory and all of its parent components if they are missing.
///
/// # Arguments
///
/// * `path` - The directory path to create.
pub fn create_path(path: &Path) {
    if let Err(e) = fs::create_dir_all(path) {
        log_error!("Failed to create directory at '{}': {}", path.display(), e);
    }
}

/// Checks if a path is accessible (exists and has read permissions).
///
/// # Arguments
///
/// * `path` - The path to check.
///
/// # Returns
///
/// * `true` if accessible, `false` otherwise.
pub fn is_path_accessible(path: &PathBuf) -> bool {
    if !path.exists() {
        return false;
    }

    // Attempt to read metadata as a basic access check
    if fs::metadata(path).is_err() {
        return false;
    }

    if path.is_dir() {
        // For directories, try listing contents to verify read permission
        fs::read_dir(path).is_ok()
    } else {
        // For files, existence + metadata check is usually sufficient for "accessibility"
        // (strict read permission check would require opening the file)
        true
    }
}

/// Lists the contents of a directory, separating them into files and subdirectories.
///
/// # Arguments
///
/// * `parent_path` - The directory path to list.
///
/// # Returns
///
/// * `Some((Vec<String>, Vec<String>))` where the first vector contains file names
///   and the second contains directory names.
/// * `None` if the path is not a directory or cannot be read.
pub fn list_directory_content(parent_path: &PathBuf) -> Option<(Vec<String>, Vec<String>)> {
    let entries = fs::read_dir(parent_path).ok()?;

    let (files, directories): (Vec<String>, Vec<String>) = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_string();

            if path.is_dir() {
                Some((None, Some(name))) // (File, Dir)
            } else {
                Some((Some(name), None))
            }
        })
        .fold((Vec::new(), Vec::new()), |(mut files, mut dirs), (f, d)| {
            if let Some(name) = f {
                files.push(name);
            }
            if let Some(name) = d {
                dirs.push(name);
            }
            (files, dirs)
        });

    Some((files, directories))
}
