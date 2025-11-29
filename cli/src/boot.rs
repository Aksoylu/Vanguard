use std::path::PathBuf;

use crate::constants::Constants;
use crate::log_error;

use crate::utils::{
    directory_utility::get_runtime_path,
    file_utility::load_json
};

use crate::models::{
    base::boot_data::BootData,
    rpc::rpc_session::RpcSession
};


pub struct Boot {}

impl Boot {
    pub fn init() -> BootData {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Self::get_rpc_session_path(&runtime_path);

        let (rpc_session, is_rpc_session_loaded_successfully) = Self::load_rpc_session(rpc_session_path.clone());
        if is_rpc_session_loaded_successfully 
        {
            return BootData { rpc_session: Some(rpc_session), is_rpc_session_loaded_successfully: true }
        }
        else {
            log_error!("Failed to load RPC Session. Vanguard Engine connection is not established. Please try again");
            BootData { rpc_session: None, is_rpc_session_loaded_successfully: false }
        }
    }

    fn get_rpc_session_path(runtime_path: &PathBuf) -> PathBuf {
        let mut session_path = runtime_path.clone();
        session_path.push(Constants::SESSION_FILENAME);
        session_path
    }

    fn load_rpc_session(rpc_session_path: PathBuf) -> (RpcSession, bool) {
        let read_operation = load_json::<RpcSession>(&rpc_session_path);
        if read_operation.is_err() {
            return (RpcSession::default(), false);
        }

        let rpc_session = read_operation.unwrap();
        (rpc_session, true)
    }
}
