use colored::{ColoredString, Colorize};
use std::path::PathBuf;

pub fn status_flag(input: bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).green()
    } else {
        format!("[{}]", error_flag).red()
    }
}
