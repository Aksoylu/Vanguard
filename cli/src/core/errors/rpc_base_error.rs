use std::{error::Error, fmt::{self, Display}};


#[derive(Debug)]
pub struct RPCBaseError {
    pub reason: String
}

impl RPCBaseError {
  
    pub fn build( message: &str) -> RPCBaseError {
        RPCBaseError {
            reason: message.into()
        }
    }
}

impl Display for RPCBaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[RPC Connection Error]: {}", self.reason)
    }
}

impl Error for RPCBaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}