use mime_guess::{from_path, Mime};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, File, Metadata};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Loads a JSON file and deserializes it into the specified type `T`.
///
/// # Arguments
///
/// * `file_path` - The path to the JSON file.
///
/// # Returns
///
/// * `Ok(T)` if the file is successfully read and deserialized.
/// * `Err` if the file cannot be opened or the content cannot be deserialized.
pub fn load_json<T>(file_path: &Path) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let data: T = serde_json::from_reader(reader)?;
    Ok(data)
}

/// Serializes the given data into a JSON file at the specified path.
///
/// # Arguments
///
/// * `file_path` - The path where the JSON file will be saved.
/// * `data` - The data to serialize and save.
///
/// # Returns
///
/// * `Ok(())` on success.
/// * `Err` if the file cannot be created or the data cannot be serialized.
pub fn save_json<T>(file_path: &PathBuf, data: &T) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

/// Writes the given string content to a file.
///
/// # Arguments
///
/// * `file_path` - The destination file path.
/// * `content` - The string content to write.
///
/// # Returns
///
/// * `Ok(())` on success.
/// * `Err` if an error occurs during the write operation.
pub fn write_file(file_path: PathBuf, content: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Deletes a file at the specified path.
///
/// # Arguments
///
/// * `file_path` - The path of the file to delete.
///
/// # Returns
///
/// * `true` if the file was successfully deleted.
/// * `false` if the file did not exist or could not be deleted.
pub fn delete_file(file_path: PathBuf) -> bool {
    fs::remove_file(file_path).is_ok()
}

/// Lists all files in the given directory.
///
/// # Arguments
///
/// * `parent_path` - The directory path to search.
///
/// # Returns
///
/// * A vector of file names (strings) found in the directory.
/// * Returns an empty vector if the directory cannot be read.
pub fn list_all_files(parent_path: PathBuf) -> Vec<String> {
    fs::read_dir(parent_path)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_file())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default()
}

/// Checks if a file exists and is indeed a file.
///
/// # Arguments
///
/// * `file_path` - The path to check.
///
/// # Returns
///
/// * `true` if the path exists and is a file.
/// * `false` otherwise.
pub fn is_file_exist(file_path: &PathBuf) -> bool {
    file_path.is_file()
}

/// Detects a MIME type by file extension.
/// MIME type means http response type that sent to server. If could not detect, returns "application/octet-stream" as default
/// # Returns
///
/// * The detected `Mime` type.
pub fn get_content_type(file_path: &PathBuf) -> Mime {
    from_path(file_path).first_or_octet_stream()
}

pub async fn open_file(file_path: &PathBuf) -> Option<tokio::fs::File> {
    let file_stream = tokio::fs::File::open(file_path).await.ok();
    file_stream
}

pub fn generate_file_tag(content_length: u64, last_modified: u64) -> String {
    let file_tag = format!("W/\"{:x}-{:x}\"", content_length, last_modified);

    file_tag
}

pub fn get_last_modified(metadata: &Metadata) -> u64 {
    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
