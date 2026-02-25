//! Offline detection module for Tempest application.
//!
//! Provides network connectivity detection using TCP socket check.
//! Supports both one-shot and periodic connectivity checking.

use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

/// Default host to check for connectivity
const DEFAULT_HOST: &str = "8.8.8.8";
/// Default port to use for connectivity check
const DEFAULT_PORT: u16 = 53;
/// Connection timeout in seconds
const TIMEOUT_SECS: u64 = 3;
/// Default interval for periodic checks in seconds
#[allow(dead_code)]
const DEFAULT_CHECK_INTERVAL_SECS: u64 = 5;

/// Check if the system is currently online by attempting to connect to a known host.
///
/// Uses a simple TCP connection check to Google's DNS server (8.8.8.8:53).
/// This is a reliable and fast way to detect basic network connectivity.
///
/// # Returns
/// * `true` if connection successful (online)
/// * `false` if connection fails (offline)
pub fn is_online() -> bool {
    check_connectivity(DEFAULT_HOST, DEFAULT_PORT)
}

/// Check connectivity to a specific host and port.
///
/// # Arguments
/// * `host` - The hostname or IP address to check
/// * `port` - The port to connect to
///
/// # Returns
/// * `true` if connection successful (online)
/// * `false` if connection fails (offline)
pub fn check_connectivity(host: &str, port: u16) -> bool {
    let address = format!("{}:{}", host, port);

    match address.parse().ok() {
        Some(socket_addr) => {
            TcpStream::connect_timeout(&socket_addr, Duration::from_secs(TIMEOUT_SECS)).is_ok()
        }
        None => false,
    }
}

/// Periodic connectivity checker that runs checks at regular intervals.
///
/// This struct manages a background thread that periodically checks
/// network connectivity and updates an atomic boolean flag.
///
/// # Example
/// ```
/// use tempest_app::offline_detection::PeriodicConnectivityChecker;
///
/// // Create a new checker with 5-second intervals
/// let checker = PeriodicConnectivityChecker::new(5);
/// checker.start();
///
/// // Check current status (from any thread)
/// if checker.is_online() {
///     println!("Network is available");
/// } else {
///     println!("Network is offline");
/// }
///
/// // Stop the checker when done
/// checker.stop();
/// ```
#[allow(dead_code)]
pub struct PeriodicConnectivityChecker {
    /// Shared online status - true means online, false means offline
    is_online: Arc<AtomicBool>,
    /// Interval between connectivity checks in seconds
    check_interval_secs: u64,
    /// Flag to control the background thread
    running: Arc<AtomicBool>,
}

#[allow(dead_code)]
impl PeriodicConnectivityChecker {
    /// Create a new periodic connectivity checker.
    ///
    /// # Arguments
    /// * `interval_secs` - Interval between connectivity checks in seconds
    ///
    /// # Returns
    /// A new PeriodicConnectivityChecker instance
    #[allow(dead_code)]
    pub fn new(interval_secs: u64) -> Self {
        Self {
            is_online: Arc::new(AtomicBool::new(false)),
            check_interval_secs: interval_secs,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the periodic connectivity check loop.
    ///
    /// This spawns a background thread that checks connectivity
    /// at the configured interval.
    #[allow(dead_code)]
    pub fn start(&self) {
        // Don't start if already running
        if self.running.load(Ordering::SeqCst) {
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        let is_online = Arc::clone(&self.is_online);
        let running = Arc::clone(&self.running);
        let interval_secs = self.check_interval_secs;

        thread::spawn(move || {
            // Do initial check
            let current_status = check_connectivity(DEFAULT_HOST, DEFAULT_PORT);
            is_online.store(current_status, Ordering::SeqCst);

            let mut last_check = Instant::now();

            while running.load(Ordering::SeqCst) {
                let elapsed = last_check.elapsed().as_secs();

                if elapsed >= interval_secs {
                    let current_status = check_connectivity(DEFAULT_HOST, DEFAULT_PORT);
                    is_online.store(current_status, Ordering::SeqCst);
                    last_check = Instant::now();
                }

                // Small sleep to prevent CPU spinning
                thread::sleep(Duration::from_millis(100));
            }
        });
    }

    /// Stop the periodic connectivity check loop.
    #[allow(dead_code)]
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the system is currently online.
    ///
    /// This can be called from any thread and returns the last
    /// known connectivity status.
    #[allow(dead_code)]
    pub fn is_online(&self) -> bool {
        self.is_online.load(Ordering::SeqCst)
    }

    /// Get the current connectivity status, performing a fresh check.
    ///
    /// This performs an immediate synchronous check rather than
    /// returning the cached status.
    #[allow(dead_code)]
    pub fn check_now(&self) -> bool {
        let status = check_connectivity(DEFAULT_HOST, DEFAULT_PORT);
        self.is_online.store(status, Ordering::SeqCst);
        status
    }
}

impl Default for PeriodicConnectivityChecker {
    fn default() -> Self {
        Self::new(DEFAULT_CHECK_INTERVAL_SECS)
    }
}

impl Drop for PeriodicConnectivityChecker {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_online_returns_bool() {
        // This test just verifies the function returns a boolean
        // Actual result depends on network state
        let _ = is_online();
    }

    #[test]
    fn test_check_connectivity_invalid_host() {
        // Invalid IP should return false
        let result = check_connectivity("192.0.2.1", 80); // TEST-NET-1, always unreachable
        assert!(!result);
    }
}
