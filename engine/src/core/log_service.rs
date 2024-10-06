use colored::*; // Bring all colored features into scope

pub struct LogService {}

impl LogService {
    /// Gets both of &str and String
    pub fn error<T: AsRef<str>>(text: T) {
        println!("\n{} >> {}", "ERROR".red(), text.as_ref());
    }

    /// Gets both of &str and String
    pub fn success<T: AsRef<str>>(text: T) {
        println!("\n{} >> {}", "SUCCESS".green(), text.as_ref());
    }

    /// Gets both of &str and String
    pub fn warning<T: AsRef<str>>(text: T) {
        println!("\n{} >> {}", "WARNING".yellow(), text.as_ref());
    }

    /// Gets both of &str and String
    pub fn info<T: AsRef<str>>(text: T) {
        println!("\n{} >> {}\n", "INFO".blue(), text.as_ref());
    }

    /// Gets both of &str and String
    pub fn output<T: AsRef<str>>(text: T) {
        println!("\nOUTPUT >> {}\n", text.as_ref());
    }
}
