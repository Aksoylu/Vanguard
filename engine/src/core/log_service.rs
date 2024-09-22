use colored::*; // Bring all colored features into scope

pub struct LogService {}

impl LogService {
    /// Gets both of &str and String
    pub fn error<T: AsRef<str>>(text: T) {
        println!("\nERROR >> {}\n", text.as_ref().red());
    }

    /// Gets both of &str and String
    pub fn success<T: AsRef<str>>(text: T) {
        println!("\nSUCCESS >> {}\n", text.as_ref().green());
    }

    /// Gets both of &str and String
    pub fn warning<T: AsRef<str>>(text: T) {
        println!("\nWARNING >> {}\n", text.as_ref());
    }

    /// Gets both of &str and String
    pub fn info<T: AsRef<str>>(text: T) {
        println!("\nINFO >> {}\n", text.as_ref().yellow());
    }

    /// Gets both of &str and String
    pub fn output<T: AsRef<str>>(text: T) {
        println!("\nOUTPUT >> {}\n", text.as_ref());
    }
}
