//! Real-time polling for new volume scans from S3.
//!
//! This module provides functionality to continuously poll an S3 bucket for a given
//! radar station and yield new volume scans as they become available.
//!
//! # Example
//!
//! ```ignore
//! use tokio_stream::StreamExt;
//! use std::time::Duration;
//! use tempest_fetch::poll::{poll_latest, PollConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = PollConfig::default();
//!     let mut stream = poll_latest("KTLX", config);
//!
//!     while let Some(scan) = stream.next().await {
//!         println!("New scan: {:?}", scan.filename);
//!     }
//! }
//! ```

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use tokio::sync::RwLock;
use tokio_stream::Stream;
use tracing::{debug, warn};

use crate::error::FetchError;
use crate::retry::{with_retry, RetryConfig};
use crate::s3::S3Client;
use crate::types::ScanMeta;

/// Configuration for polling behavior.
#[derive(Debug, Clone)]
pub struct PollConfig {
    /// Interval between polling attempts.
    pub poll_interval: Duration,
    /// Maximum number of retries for failed S3 requests.
    pub max_retries: u32,
}

impl Default for PollConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(60),
            max_retries: 3,
        }
    }
}

/// Internal state for tracking seen scans.
#[derive(Debug, Clone)]
struct SeenScans {
    /// Set of filenames that have already been seen.
    filenames: HashSet<String>,
}

impl Default for SeenScans {
    fn default() -> Self {
        Self::new()
    }
}

impl SeenScans {
    /// Create a new empty SeenScans tracker.
    fn new() -> Self {
        Self {
            filenames: HashSet::new(),
        }
    }

    /// Check if a scan has been seen before.
    fn is_seen(&self, filename: &str) -> bool {
        self.filenames.contains(filename)
    }

    /// Mark a scan as seen.
    fn mark_seen(&mut self, filename: String) {
        self.filenames.insert(filename);
    }
}

/// Poll for new volume scans for a given station.
///
/// This function continuously polls the S3 bucket for the specified station
/// and yields new scans as they appear. It uses the configured poll interval
/// to control how often to check for new data.
///
/// # Arguments
///
/// * `station` - Radar station identifier (e.g., "KTLX")
/// * `config` - Polling configuration
///
/// # Returns
///
/// Returns a Stream that yields new `ScanMeta` items as they appear.
///
/// # Example
///
/// ```ignore
/// use tokio_stream::StreamExt;
/// use tempest_fetch::poll::{poll_latest, PollConfig};
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() {
///     let config = PollConfig {
///         poll_interval: Duration::from_secs(30),
///         max_retries: 3,
///     };
///     let mut stream = poll_latest("KTLX", config);
///
///     while let result = stream.next().await {
///         match result {
///             Some(Ok(scan)) => println!("New scan: {}", scan.filename),
///             Some(Err(e)) => eprintln!("Error: {}", e),
///             None => break,
///         }
///     }
/// }
/// ```
pub fn poll_latest(
    station: &str,
    config: PollConfig,
) -> impl Stream<Item = Result<ScanMeta, FetchError>> {
    let station = station.to_uppercase();
    let client = S3Client::new().expect("Failed to create S3 client");
    let seen_scans: Arc<RwLock<SeenScans>> = Arc::new(RwLock::new(SeenScans::new()));
    let retry_config = RetryConfig {
        max_retries: config.max_retries,
        base_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(30),
    };

    // Create a stream that yields new scans
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(config.poll_interval);
        
        // Skip the first immediate tick
        interval.tick().await;
        
        loop {
            // Get current date for listing
            let today = Utc::now().date_naive();
            let current_station = station.clone();
            let current_retry_config = retry_config.clone();

            // Try to list scans from S3 with retry
            match with_retry(current_retry_config, || {
                let station = current_station.clone();
                let client = client.clone();
                async move { client.list_scans(&station, today).await }
            }).await {
                Ok(scans) => {
                    let mut seen = seen_scans.write().await;

                    for scan in scans {
                        if !seen.is_seen(&scan.filename) {
                            debug!("Found new scan: {}", scan.filename);
                            seen.mark_seen(scan.filename.clone());
                            yield Ok(scan);
                        }
                    }
                }
                Err(e) => {
                    warn!("Error polling S3 for station {}: {}", station, e);
                    yield Err(e);
                }
            }

            // Wait for next poll interval
            interval.tick().await;
        }
    };

    stream
}

/// Convenience function to create a polling stream with default configuration.
///
/// Uses a default poll interval of 60 seconds and 3 retries.
pub fn poll_latest_default(station: &str) -> impl Stream<Item = Result<ScanMeta, FetchError>> {
    poll_latest(station, PollConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poll_config_default() {
        let config = PollConfig::default();
        assert_eq!(config.poll_interval, Duration::from_secs(60));
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_seen_scans_tracking() {
        let mut seen = SeenScans::new();

        assert!(!seen.is_seen("scan1"));

        seen.mark_seen("scan1".to_string());
        assert!(seen.is_seen("scan1"));
        assert!(!seen.is_seen("scan2"));
    }

    #[test]
    fn test_poll_config_custom() {
        let config = PollConfig {
            poll_interval: Duration::from_secs(30),
            max_retries: 5,
        };
        assert_eq!(config.poll_interval, Duration::from_secs(30));
        assert_eq!(config.max_retries, 5);
    }

    #[tokio::test]
    async fn test_poll_latest_creates_stream() {
        // This test just verifies the function can be called and returns a stream
        // The actual polling would require network access
        let config = PollConfig {
            poll_interval: Duration::from_millis(100),
            max_retries: 1,
        };
        let _stream = poll_latest("KTLX", config);
    }

    #[test]
    fn test_seen_scans_default() {
        let seen = SeenScans::default();
        assert!(!seen.is_seen("anything"));
    }
}
