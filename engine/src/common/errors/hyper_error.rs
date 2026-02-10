use std::fmt;

#[derive(Debug)]
pub struct HyperError(String);

impl fmt::Display for HyperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for HyperError {
    fn from(msg: String) -> Self {
        HyperError(msg)
    }
}

impl From<&str> for HyperError {
    fn from(msg: &str) -> Self {
        HyperError(msg.to_string())
    }
}

impl HyperError{
    pub fn get_message(&self) -> &str {
        self.0.as_str()
    }
}

impl std::error::Error for HyperError {}