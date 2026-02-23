//! Offline detection module for Tempest application.
//!
//! Provides simple network connectivity detection using TCP socket check.

use std::net::TcpStream;
use std::time::Duration;

/// Default host to check for connectivity
const DEFAULT_HOST: &str = "8.8.8.8";
/// Default port to use for connectivity check
const DEFAULT_PORT: u16 = 53;
/// Connection timeout in seconds
const TIMEOUT_SECS: u64 = 3;

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

    TcpStream::connect_timeout(
        &address.parse().expect("Invalid IP address"),
        Duration::from_secs(TIMEOUT_SECS),
    )
    .is_ok()
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
