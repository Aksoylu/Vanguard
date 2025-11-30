use std::{error::Error, fmt::{self, Display}};

use hyper::StatusCode;

#[derive(Debug)]
pub struct RPCCallError {
    pub code: StatusCode,
    pub message: String
}

impl RPCCallError {
  
    pub fn build(code: &StatusCode, message: &str) -> RPCCallError {
        RPCCallError {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl Display for RPCCallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[RPC Call Error] {} : {}", self.code, self.message)
    }
}

impl Error for RPCCallError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}