use colored::{ColoredString, Colorize};
use std::path::PathBuf;

pub fn clear_punctation(input: String) -> String {
    input
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect()
}

pub fn status_flag(input: bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).green()
    } else {
        format!("[{}]", error_flag).red()
    }
}

pub fn warning_flag(input: bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).blue()
    } else {
        format!("[{}]", error_flag).yellow()
    }
}

pub fn pathbuf_to_string(input: &PathBuf) -> String {
    String::from(input.to_str().unwrap_or_default().replace("\"", ""))
}

pub fn mask_token(input: &str) -> String {
    if input.len() <= 8 {
        "****".to_string()
    } else {
        let start = &input[..4];
        let end = &input[input.len() - 4..];
        format!("{}************{}", start, end)
    }
}

pub fn normalize_string(input_string: &String) -> String{
    let normalized = input_string.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .to_lowercase();

    normalized
}
