use crate::{ models::base::boot_data::{BootData}};

// todo: development required here
pub struct RPCClient{
    pub boot_data: Option<BootData>
}

impl RPCClient{
    pub fn init(boot_data: BootData) -> RPCClient{

        RPCClient{boot_data: Some(boot_data)}
    }
}

impl Default for RPCClient{
    fn default() -> Self {
        Self{boot_data: None}
    }
}