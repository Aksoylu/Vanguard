use colored::{ColoredString, Colorize};
use std::path::PathBuf;

/// Formats a boolean input into a colored status string.
///
/// # Arguments
///
/// * `input` - The boolean status flag.
/// * `ok_flag` - The string to display if `input` is true (green).
/// * `error_flag` - The string to display if `input` is false (red).
///
/// # Returns
///
/// * A `ColoredString` formatted as brackets containing the flag.
pub fn status_flag(input: bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).green()
    } else {
        format!("[{}]", error_flag).red()
    }
}

/// Formats a boolean input into a colored warning/info status string.
///
/// # Arguments
///
/// * `input` - The boolean status flag.
/// * `ok_flag` - The string to display if `input` is true (blue).
/// * `error_flag` - The string to display if `input` is false (yellow).
///
/// # Returns
///
/// * A `ColoredString` formatted as brackets containing the flag.
pub fn warning_flag(input: bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).blue()
    } else {
        format!("[{}]", error_flag).yellow()
    }
}

/// Converts a `PathBuf` to a string, removing any double quotes.
///
/// # Arguments
///
/// * `input` - The path to convert.
///
/// # Returns
///
/// * A string representation of the path, defaulting to empty string if invalid UTF-8.
pub fn pathbuf_to_string(input: &PathBuf) -> String {
    input.to_string_lossy().replace('"', "")
}

/// Masks the middle part of a token string, leaving only the first 4 and last 4 characters visible.
///
/// If the token is shorter than or equal to 8 characters, it is completely masked.
///
/// # Arguments
///
/// * `input` - The token string to mask.
///
/// # Returns
///
/// * A masked string representation.
pub fn mask_token(input: &str) -> String {
    let char_count = input.chars().count();
    if char_count <= 8 {
        "****".to_string()
    } else {
        let start: String = input.chars().take(4).collect();
        let end: String = input.chars().skip(char_count - 4).collect();
        format!("{}************{}", start, end)
    }
}

/// Normalizes a string by removing whitespace and converting to lowercase.
///
/// # Arguments
///
/// * `input_string` - The string to normalize.
///
/// # Returns
///
/// * The normalized string.
pub fn normalize_string(input_string: &str) -> String {
    input_string
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
}

/// Parses a human-readable size string into bytes.
///
/// Supports units: B, KB, MB, GB (case-insensitive).
/// If no unit is provided, it defaults to KB.
///
/// # Arguments
///
/// * `input` - The size string to parse (e.g., "10mb", "1gb", "1024").
///
/// # Returns
///
/// * A `Result` containing the size in bytes or an error message.
pub fn parse_str_as_size(input: &str) -> Result<u64, String> {
    let normalized = normalize_string(input);
    if normalized.is_empty() {
        return Err("Input string is empty".to_string());
    }

    let split_index = normalized.find(|c: char| !c.is_numeric());

    let (num_part, unit_part) = match split_index {
        Some(index) => (&normalized[..index], &normalized[index..]),
        None => (&normalized[..], ""),
    };

    if num_part.is_empty() {
        return Err(format!("Invalid size format: '{}'", input));
    }

    let value: u64 = num_part
        .parse()
        .map_err(|_| format!("Invalid numeric value: '{}'", num_part))?;

    let multiplier: u64 = match unit_part {
        "kb" | "k" => 1024,
        "mb" | "m" => 1024 * 1024,
        "gb" | "g" => 1024 * 1024 * 1024,
        "" | "b" => 1,
        _ => return Err(format!("Invalid unit: '{}'", unit_part)),
    };

    Ok(value * multiplier)
}
