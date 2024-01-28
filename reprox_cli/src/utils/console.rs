use std::io;
use std::io::Write;

use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

pub fn clear_screen() {
    // Attempt to clear the screen
    if let Err(_e) = execute!(std::io::stdout(), Clear(ClearType::All)) {
        for _i in 1..=100 {
            println!("xxx\n");
        }
    }
}

pub fn console_read() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().to_string()
}


pub fn separator(count: usize) {
    println!("{}", "-".repeat(count));
}