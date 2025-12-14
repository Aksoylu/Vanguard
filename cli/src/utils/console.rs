use std::io::{self, stdout, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

/// Clears entire CLI buffer and window as multiplatform
pub fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(terminal::Clear(ClearType::Purge))?;
    stdout.flush()?;
    Ok(())
}

pub fn console_read(flag: &str) -> String {
    print!("{} ", flag);
    let mut input = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub fn separator(count: usize) {
    println!("{}", "-".repeat(count));
}

/// Prints the given text to the console with the specified foreground color.
///
/// Note: This function panics if it fails to write to stdout. In a production
/// application, you might want to return `io::Result<()>` instead of unwrapping.
pub fn print_colored(text: &str, color: Color) {
    let mut stdout = stdout();

    // The `execute` method is chained to ensure all commands are sent
    // sequentially and the color is reset afterward.
    if let Err(e) = stdout
        .execute(SetForegroundColor(color))
        .and_then(|s| s.execute(Print(text)))
        .and_then(|s| s.execute(ResetColor))
        .and_then(|s| s.execute(Print("\n")))
    // Add a newline at the end
    {
        // Handle the error if printing fails
        eprintln!("Error writing colored text: {}", e);
    }
}
