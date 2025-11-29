use crate::models::rpc::rpc_session::RpcSession;

pub struct BootData {
    pub rpc_session: Option<RpcSession>,
    pub is_rpc_session_loaded_successfully: bool
}
