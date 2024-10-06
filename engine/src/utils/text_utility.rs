use std::path::PathBuf;

use colored::{ColoredString, Colorize};

pub fn clear_punctation(input: String) -> String {
    input
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect()
}

pub fn get_flag(input:bool, ok_flag: &str, error_flag: &str) -> ColoredString {
    if input {
        format!("[{}]", ok_flag).green()
    } else {
        format!("[{}]", error_flag).red()
    }
}

pub fn pathbuf_to_string(input: &PathBuf) -> String {
    String::from(input.to_str().unwrap_or_default().replace("\"", ""))
}

pub fn print_banner()  {
println!(r#"
##     ##    ###    ##    ##  ######   ##     ##    ###    ########  ######## 
##     ##   ## ##   ###   ## ##    ##  ##     ##   ## ##   ##     ## ##     ##
##     ##  ##   ##  ####  ## ##        ##     ##  ##   ##  ##     ## ##     ##
##     ## ##     ## ## ## ## ##   #### ##     ## ##     ## ########  ##     ##
 ##   ##  ######### ##  #### ##    ##  ##     ## ######### ##   ##   ##     ##
  ## ##   ##     ## ##   ### ##    ##  ##     ## ##     ## ##    ##  ##     ##
   ###    ##     ## ##    ##  ######    #######  ##     ## ##     ## ######## 
"#)
}