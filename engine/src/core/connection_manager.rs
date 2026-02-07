use crate::constants::Constants;
use crate::core::connection_lock::ConnectionLock;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::RwLock;
use std::time::Instant;

pub type ConnectionPermit = ConnectionLock;

pub struct ConnectionManager {
    active_connections: AtomicUsize,
    total_requests: AtomicU64,
    start_time: Instant,
    rate_limits: RwLock<HashMap<IpAddr, (u32, Instant)>>,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self {
            active_connections: AtomicUsize::new(0),
            total_requests: AtomicU64::new(0),
            start_time: Instant::now(),
            rate_limits: RwLock::new(HashMap::new()),
        }
    }
}

impl ConnectionManager {
    /// Tries to acquire a connection
    pub fn try_acquire_connection(&self) -> Option<ConnectionLock> {
        let current = self.active_connections.load(Ordering::Relaxed) as u64;
        if current >= Constants::DEFAULT_MAXIMUM_TOTAL_CONNECTIONS {
            return None;
        }

        self.active_connections.fetch_add(1, Ordering::SeqCst);
        Some(ConnectionLock)
    }

    /// Releases a connection
    pub fn release_connection(&self) {
        self.active_connections.fetch_sub(1, Ordering::SeqCst);
    }

    /// Increments the total number of requests
    pub fn increment_total_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::SeqCst);
    }

    /// Checks if the IP address has exceeded the rate limit
    pub fn check_rate_limit(&self, ip: IpAddr) -> bool {
        let mut limits = self.rate_limits.write().unwrap();
        let now = Instant::now();
        let (count, start) = limits.entry(ip).or_insert((0, now));

        if now.duration_since(*start).as_secs() >= 60 {
            *count = 1;
            *start = now;
            return true;
        }

        if *count >= Constants::DEFAULT_MAX_REQUESTS_PER_MINUTE {
            return false;
        }

        *count += 1;
        true
    }

    /// @todo: Future usage for getting engine metrics
    /// Returns the number of active connections
    /// @note: This method is not thread-safe, use it only for logging purposes
    pub fn get_active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    /// @todo: Future usage for getting engine metrics
    /// Returns the total number of requests
    /// @note: This method is not thread-safe, use it only for logging purposes
    pub fn get_total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }

    /// @todo: Future usage for getting engine metrics
    /// Returns the requests per second
    /// @note: This method is not thread-safe, use it only for logging purposes
    pub fn get_rps(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.get_total_requests() as f64 / elapsed
        } else {
            0.0
        }
    }
}
