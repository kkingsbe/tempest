//! Types for tempest-fetch crate.

use chrono::{DateTime, Utc};

/// Metadata for a scanned radar file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanMeta {
    /// Radar station identifier (e.g., "KATX").
    pub station: String,
    /// Date of the scan.
    pub date: DateTime<Utc>,
    /// Filename of the scan data.
    pub filename: String,
    /// Size of the file in bytes.
    pub size: u64,
    /// Timestamp when the file was last modified.
    pub timestamp: DateTime<Utc>,
}

impl ScanMeta {
    /// Create a new ScanMeta instance.
    pub fn new(
        station: impl Into<String>,
        date: DateTime<Utc>,
        filename: impl Into<String>,
        size: u64,
        timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            station: station.into(),
            date,
            filename: filename.into(),
            size,
            timestamp,
        }
    }

    /// Generate a cache key for this scan.
    pub fn cache_key(&self) -> String {
        format!(
            "{}_{}_{}",
            self.station,
            self.date.format("%Y%m%d"),
            self.filename
        )
    }
}

/// Statistics about the cache.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total size of cached data in bytes.
    pub total_size: u64,
    /// Number of entries in the cache.
    pub entry_count: usize,
    /// Timestamp of the oldest entry (if any).
    pub oldest: Option<DateTime<Utc>>,
    /// Timestamp of the newest entry (if any).
    pub newest: Option<DateTime<Utc>>,
}

impl CacheStats {
    /// Create a new CacheStats instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entry_count == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_scan_meta_cache_key() {
        let date = Utc.with_ymd_and_hms(2024, 3, 15, 12, 0, 0).unwrap();
        let timestamp = Utc.with_ymd_and_hms(2024, 3, 15, 12, 30, 0).unwrap();

        let meta = ScanMeta::new("KATX", date, "KATX20240315_120021", 1024, timestamp);

        assert_eq!(meta.cache_key(), "KATX_20240315_KATX20240315_120021");
    }

    #[test]
    fn test_cache_stats_default() {
        let stats = CacheStats::default();
        assert!(stats.is_empty());
        assert_eq!(stats.total_size, 0);
        assert_eq!(stats.entry_count, 0);
    }
}
