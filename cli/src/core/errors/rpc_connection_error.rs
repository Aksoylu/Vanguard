use std::{error::Error, fmt::{self, Display}};


#[derive(Debug)]
pub struct RPCConnectionError {
    pub reason: String
}

impl RPCConnectionError {
  
    pub fn build( message: &str) -> RPCConnectionError {
        RPCConnectionError {
            reason: message.into()
        }
    }
}

impl Display for RPCConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[RPC Connection Error]: {}", self.reason)
    }
}

impl Error for RPCConnectionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}