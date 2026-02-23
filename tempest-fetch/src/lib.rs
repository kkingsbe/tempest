//! Tempest Fetch - Phase 3: S3 data pipeline and local cache
//!
//! This crate provides functionality for fetching NEXRAD weather radar data
//! from AWS S3 and managing a local cache of downloaded files.
//!
//! # Key Features
//!
//! - Station discovery and metadata lookup for NEXRAD radars
//! - S3 integration for enumerating and downloading radar scans
//! - Local file caching to reduce bandwidth usage
//!
//! # Example
//!
//! ```rust
//! use tempest_fetch::{get_station, list_all_stations};
//!
//! // Get a specific station by ID
//! if let Some(station) = get_station("KTLX") {
//!     println!("Found station: {} at ({}, {})",
//!         station.name, station.lat, station.lon);
//! }
//!
//! // List all available stations
//! for station in list_all_stations() {
//!     println!("{}", station.id);
//! }
//! ```

// Modules
mod cache;
mod decompress;
mod error;
pub mod mock_s3;
mod poll;
pub mod prefetch;
mod prefetch_service;
mod retry;
pub mod s3;
mod station;
mod stations_data;
mod types;

// Public exports
pub use cache::{cache_default, Cache, CacheConfig};
pub use decompress::decompress_bz2;
pub use decompress::decompress_gzip;
pub use error::FetchError;
pub use poll::{poll_latest, poll_latest_default, PollConfig};
pub use prefetch::{
    PlaybackDirection, PlaybackState, PrefetchConfig, PrefetchPrediction, Prefetcher,
};
pub use prefetch_service::PrefetchService;
pub use s3::{fetch_scan, list_scans, S3Client};
pub use station::{get_station, list_all_stations, registry, Station, StationRegistry};
pub use types::{CacheStats, ScanMeta};

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use super::{
        cache_default, decompress_bz2, decompress_gzip, fetch_scan, get_station, list_all_stations,
        list_scans, poll_latest, poll_latest_default, Cache, CacheConfig, CacheStats, FetchError,
        PlaybackDirection, PlaybackState, PollConfig, PrefetchConfig, PrefetchPrediction,
        PrefetchService, Prefetcher, S3Client, ScanMeta, Station, StationRegistry,
    };
}
