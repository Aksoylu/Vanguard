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

        let result = convert_input_to_boolean(&console_input);
        if result.is_some() {
            return result.unwrap();
        }

        println!("Please select 'y' (yes) or 'n' (no)");
    }
}

pub fn convert_input_to_boolean(input: &str) -> Option<bool> {
    let normalized_input = input.trim().to_lowercase();

    if normalized_input == "y" || normalized_input == "yes" {
        return Some(true);
    }
    if normalized_input == "n" || normalized_input == "no" {
        return Some(false);
    }

    None
}
