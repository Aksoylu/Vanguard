use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::PathBuf;

static ANSI_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").unwrap());

pub fn strip_ansi_codes(s: &str) -> String {
    ANSI_RE.replace_all(s, "").to_string()
}

pub fn clear_punctation(input: String) -> String {
    input
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect()
}

pub fn get_flag(input: bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).green()
    } else {
        format!("[{}]", error_flag).red()
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

#[macro_export]
macro_rules! fixed_row {
    ($width:expr, $($arg:tt)*) => {{
        let width: usize = $width.parse::<usize>().unwrap_or(80);
        let formatted = format!($($arg)*);

        let display_width = console::measure_text_width(&formatted);
        let padding = width - display_width;

        format!("{}{}", formatted, " ".repeat(padding))
    }};
}
