use std::process;

/// @todo implement graceful shutdown here
pub async fn exit() {
    process::exit(0);
}
