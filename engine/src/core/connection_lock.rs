use crate::core::shared_memory::CONNECTION_MANAGER;

pub struct ConnectionLock;

impl Drop for ConnectionLock {
    fn drop(&mut self) {
        CONNECTION_MANAGER.release_connection();
    }
}