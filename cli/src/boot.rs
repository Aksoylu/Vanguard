use crate::utils::file_utility::load_json;
use crate::core::rpc_session::RpcSession;
use crate::constants::Constants;
use std::path::PathBuf;

pub struct Boot {}

impl Boot {
    pub fn init() -> BootResult {
        let runtime_path = get_runtime_path();
        let rpc_session_path = Self::get_rpc_session_path(&runtime_path);

        let (rpc_session, is_rpc_session_loaded_successfully) = Self::load_rpc_session(rpc_session_path.clone());
        if !is_config_loaded_successfully {
            error!("Failed to load RPC Session. Using default settings.");
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
