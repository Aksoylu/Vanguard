use std::future::Future;
use chrono::Utc;
use tokio::time::timeout;
use tokio::time::error::Elapsed;
use std::time::Instant as Clock;

/// Returns the current timestamp in seconds.
pub fn get_current_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn start_clock() -> Clock {
    let start_time: Clock = Clock::now();
    start_time
}

/// Takes Clock as parameter, returns elapsed time as ms n U128 type
pub fn stop_clock(start_time: Clock) -> u128 {
    
    start_time.elapsed().as_millis()
}

/// Converts a u64 value representing milliseconds to a Duration.
pub fn u64_to_duration(value: u64) -> std::time::Duration {
    std::time::Duration::from_millis(value)
}

/// Runs a future with a timeout.
/// If the future does not complete within the specified timeout, it returns an Elapsed error.
pub async fn run_in_time_buffer<F, T>(timeout_ms: u64, future: F) -> Result<T, Elapsed>
where
    F: Future<Output = T>,
{
    timeout(u64_to_duration(timeout_ms), future).await
}
