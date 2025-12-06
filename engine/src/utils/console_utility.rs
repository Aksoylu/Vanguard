use std::io::{self, Write};

pub fn approve_dialog(text: &str) -> bool {
    loop {
        print!("{}", text);
        io::stdout()
            .flush()
            .expect("Console std out can not flushed");

        let mut console_input = String::new();
        io::stdin()
            .read_line(&mut console_input)
            .expect("Can not read input from console");

        let temiz_girdi = console_input.trim().to_lowercase();

        match temiz_girdi.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Please select 'y' (yes) or 'n' (no)");
            }
        }
    }
}
