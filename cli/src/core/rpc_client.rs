use serde::de::value::Error;
use serde_json::{from_str, to_string};
use std::{collections::HashMap, fs::File, io::Read};

use crate::{core::rpc_session::RpcSession, settings::Settings};

#[derive()]
pub struct RpcClient {
    session: RpcSession,
    endpoint: String,
}

impl RpcClient {

    pub fn init_session() -> Result<Self, Error> {
        match RpcClient::read_rpc_session(Settings::SESSION_PATH.to_string()) {
            Ok(session) => {
                let endpoint: String = format!("{}:{}", session.ip_addr, session.port);

                return Ok(Self { session, endpoint });
            }
            Err(err) => return Err(err),
        }
    }

    pub fn init_manual() -> Result<Self, Error> {
        match RpcClient::read_rpc_session(Settings::SESSION_PATH.to_string()) {
            Ok(session) => {
                let endpoint: String = format!("{}:{}", session.ip_addr, session.port);

                return Ok(Self { session, endpoint });
            }
            Err(err) => return Err(err),
        }
    }

    pub async fn send_rpc(
        &self,
        method: String,
        parameter: Option<HashMap<String, String>>,
    ) -> Result<String, Error> {
        let mut route_table: HashMap<String, String> = HashMap::new();

        Ok("sa".to_string())
    }

    /// Public: This function is responsible of generating `RPC` session file for making possible to sending JRPC Request from CLI application
    ///
    /// # Arguments
    /// * `private_key` - Private key value that specified in `settings.json` file. (`&str`)
    ///
    fn read_rpc_session(session_file_path: String) -> Result<RpcSession, Error> {
        let mut file_buffer = String::new();

        let mut io = File::open(session_file_path).expect("Failed to read internal session data");
        io.read_to_string(&mut file_buffer)
            .expect("Failed to read file");

        let session: RpcSession =
            from_str(&file_buffer).expect("Failed to parse internal session data");

        Ok(session)
    }
}
