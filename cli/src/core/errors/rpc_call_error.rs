use std::{error::Error, fmt::{self, Display}};

use hyper::StatusCode;

#[derive(Debug)]
pub struct RPCError {
    pub code: StatusCode,
    pub message: String
}

impl RPCError {
  
    pub fn build(code: &StatusCode, message: &str) -> RPCError {
        RPCError {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl Display for RPCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[RPC Call Error] {} : {}", self.code, self.message)
    }
}

impl Error for RPCError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}