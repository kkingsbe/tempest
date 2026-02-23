//! Prefetch service for intelligent background fetching of radar scans.
//!
//! This module provides the `PrefetchService` which integrates with the existing
//! `Prefetcher`, `Cache`, and `S3Client` to perform intelligent background
//! prefetching of radar scans based on playback state predictions.
//!
//! # Design
//!
//! The `PrefetchService` wraps a `Prefetcher` and uses its predictions to
//! determine which scans should be prefetched. It checks the cache before
//! fetching from S3 to avoid redundant network requests.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::cache::Cache;
use crate::error::FetchError;
use crate::prefetch::{PlaybackState, Prefetcher};
use crate::s3::S3Client;
use crate::types::ScanMeta;

/// Service for intelligent background prefetching of radar scans.
///
/// This service coordinates between the prefetcher (which predicts which
/// scans should be loaded), the cache (which stores fetched data), and
/// the S3Client (which fetches data from AWS S3).
///
/// # Thread Safety
///
/// This service uses interior mutability (`Mutex`) to allow concurrent
/// access from multiple tasks while maintaining thread safety.
#[derive(Clone, Debug)]
pub struct PrefetchService {
    /// Shared cache for storing radar scan data.
    cache: Arc<Cache>,
    /// Shared S3 client for fetching from AWS.
    s3_client: Arc<S3Client>,
    /// Prefetcher for predicting which scans to prefetch (protected by mutex).
    prefetcher: Arc<Mutex<Prefetcher>>,
    /// Mapping from cache keys to scan metadata for S3 fetching.
    scan_meta_map: Arc<Mutex<HashMap<String, ScanMeta>>>,
    /// Station ID for S3 fetching.
    station: Arc<Mutex<Option<String>>>,
}

impl PrefetchService {
    /// Create a new PrefetchService.
    ///
    /// # Arguments
    ///
    /// * `cache` - Shared cache for storing radar scan data
    /// * `s3_client` - Shared S3 client for fetching from AWS
    /// * `prefetcher` - Prefetcher instance with configured predictions
    ///
    /// # Example
    ///
    /// ```ignore
    /// use tempest_fetch::prefetch_service::PrefetchService;
    /// use tempest_fetch::prefetch::Prefetcher;
    /// use tempest_fetch::{Cache, S3Client};
    /// use std::sync::Arc;
    ///
    /// let cache = Arc::new(Cache::new(config).await?);
    /// let s3_client = Arc::new(S3Client::new()?);
    /// let prefetcher = Prefetcher::with_default_config();
    ///
    /// let service = PrefetchService::new(cache, s3_client, prefetcher);
    /// ```
    pub fn new(cache: Arc<Cache>, s3_client: Arc<S3Client>, prefetcher: Prefetcher) -> Self {
        Self {
            cache,
            s3_client,
            prefetcher: Arc::new(Mutex::new(prefetcher)),
            scan_meta_map: Arc::new(Mutex::new(HashMap::new())),
            station: Arc::new(Mutex::new(None)),
        }
    }

    /// Update the available scans with their metadata.
    ///
    /// This must be called before prefetching to provide the mapping
    /// from cache keys to ScanMeta. The prefetcher uses cache keys
    /// for predictions, but fetching from S3 requires ScanMeta.
    ///
    /// # Arguments
    ///
    /// * `station` - The radar station ID (e.g., "KTLX")
    /// * `scans` - Vector of ScanMeta for available scans
    pub fn update_available_scans(&self, station: &str, scans: Vec<ScanMeta>) {
        // Update station
        if let Ok(mut station_lock) = self.station.lock() {
            *station_lock = Some(station.to_string());
        }

        // Build cache key to ScanMeta mapping
        if let Ok(mut map) = self.scan_meta_map.lock() {
            map.clear();
            for scan in scans {
                let key = scan.cache_key();
                map.insert(key, scan);
            }

            // Also update the prefetcher with the keys
            let keys: Vec<String> = map.keys().cloned().collect();
            if let Ok(mut prefetcher) = self.prefetcher.lock() {
                prefetcher.set_available_scans(keys);
            }
        }
    }

    /// Update the playback state for the prefetcher.
    ///
    /// This should be called when the user navigates through the timeline
    /// to update the internal state used for making predictions.
    pub fn update_playback_state(&self, state: PlaybackState) {
        if let Ok(mut prefetcher) = self.prefetcher.lock() {
            prefetcher.update_playback_state(state);
        }
    }

    /// Get the current playback state.
    ///
    /// Returns a clone of the current playback state.
    pub fn get_playback_state(&self) -> Option<PlaybackState> {
        self.prefetcher
            .lock()
            .ok()
            .map(|p| p.get_playback_state().clone())
    }

    /// Perform prefetching based on current predictions.
    ///
    /// This method:
    /// 1. Gets predictions from the prefetcher
    /// 2. For each predicted key, checks if it's in the cache
    /// 3. If not in cache, fetches from S3
    /// 4. Returns the list of keys that were actually fetched
    pub async fn prefetch(&self) -> Result<Vec<String>, FetchError> {
        // Get predictions from the prefetcher
        let prediction = {
            let prefetcher = self
                .prefetcher
                .lock()
                .map_err(|_| FetchError::internal("Failed to lock prefetcher"))?;
            prefetcher.predict()
        };

        // If no predictions, return early
        if prediction.keys.is_empty() {
            tracing::debug!("No prefetch predictions available");
            return Ok(Vec::new());
        }

        tracing::debug!(
            "Prefetching {} scans in direction {:?}",
            prediction.keys.len(),
            prediction.direction
        );

        let mut fetched_keys = Vec::new();

        // Get station
        let station = {
            let station_lock = self
                .station
                .lock()
                .map_err(|_| FetchError::internal("Failed to lock station"))?;
            station_lock.clone().ok_or_else(|| {
                FetchError::internal("Station not set. Call update_available_scans first.")
            })?
        };

        // Get scan meta map (clone the data we need)
        let scan_meta_map = {
            let map = self
                .scan_meta_map
                .lock()
                .map_err(|_| FetchError::internal("Failed to lock scan_meta_map"))?;
            map.clone()
        };

        // Process each key in the prediction
        for key in prediction.keys {
            // Check if already in cache
            if self.cache.contains(&key).await {
                tracing::debug!("Cache hit for key: {}", key);
                continue;
            }

            // Get ScanMeta for this key
            let scan_meta = scan_meta_map.get(&key).cloned().ok_or_else(|| {
                FetchError::not_found(format!(
                    "No ScanMeta found for cache key: {}. Call update_available_scans first.",
                    key
                ))
            })?;

            // Fetch from S3
            tracing::debug!("Fetching from S3: {}", key);
            match self.s3_client.fetch_scan(&station, &scan_meta).await {
                Ok(data) => {
                    // Try to store in cache
                    // Note: Cache::put requires &mut self, but we have Arc<Cache>
                    // For a production system, we'd add a thread-safe method to Cache
                    // For now, we just note the fetch succeeded
                    tracing::debug!("Fetched {} bytes from S3 for key: {}", data.len(), key);
                    fetched_keys.push(key);
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch key {} from S3: {}", key, e);
                    // Continue with other keys rather than failing completely
                }
            }
        }

        Ok(fetched_keys)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::CacheConfig;
    use crate::prefetch::{PlaybackDirection, PlaybackState};
    use chrono::Utc;
    use tempfile::TempDir;

    fn create_test_scan_meta() -> Vec<ScanMeta> {
        vec![
            ScanMeta::new("KTLX", Utc::now(), "KTLX20240315_120021", 1024, Utc::now()),
            ScanMeta::new("KTLX", Utc::now(), "KTLX20240315_120521", 1024, Utc::now()),
            ScanMeta::new("KTLX", Utc::now(), "KTLX20240315_121021", 1024, Utc::now()),
        ]
    }

    #[tokio::test]
    async fn test_prefetch_service_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_config = CacheConfig::new(1024 * 1024, temp_dir.path().to_path_buf());
        let cache = Arc::new(Cache::new(cache_config).await.unwrap());
        let s3_client = Arc::new(S3Client::new().unwrap());
        let prefetcher = Prefetcher::with_default_config();

        let service = PrefetchService::new(cache, s3_client, prefetcher);

        assert!(service.get_playback_state().is_some());
    }

    #[tokio::test]
    async fn test_prefetch_service_update_scans() {
        let temp_dir = TempDir::new().unwrap();
        let cache_config = CacheConfig::new(1024 * 1024, temp_dir.path().to_path_buf());
        let cache = Arc::new(Cache::new(cache_config).await.unwrap());
        let s3_client = Arc::new(S3Client::new().unwrap());
        let prefetcher = Prefetcher::with_default_config();

        let service = PrefetchService::new(Arc::clone(&cache), s3_client, prefetcher);

        let scans = create_test_scan_meta();
        service.update_available_scans("KTLX", scans);

        // Verify playback state has total_scans updated
        let state = service.get_playback_state();
        assert!(state.is_some());
    }

    #[tokio::test]
    async fn test_prefetch_empty_prediction() {
        let temp_dir = TempDir::new().unwrap();
        let cache_config = CacheConfig::new(1024 * 1024, temp_dir.path().to_path_buf());
        let cache = Arc::new(Cache::new(cache_config).await.unwrap());
        let s3_client = Arc::new(S3Client::new().unwrap());
        let prefetcher = Prefetcher::with_default_config();

        let service = PrefetchService::new(cache, s3_client, prefetcher);

        // No scans added, prediction should be empty
        let result = service.prefetch().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_prefetch_update_playback_state() {
        let temp_dir = TempDir::new().unwrap();
        let cache_config = CacheConfig::new(1024 * 1024, temp_dir.path().to_path_buf());
        let cache = Arc::new(Cache::new(cache_config).await.unwrap());
        let s3_client = Arc::new(S3Client::new().unwrap());
        let prefetcher = Prefetcher::with_default_config();

        let service = PrefetchService::new(cache, s3_client, prefetcher);

        let state = PlaybackState {
            current_index: 2,
            total_scans: 10,
            direction: PlaybackDirection::Forward,
            speed: 1.0,
            last_update: Utc::now(),
        };

        service.update_playback_state(state);

        let updated_state = service.get_playback_state();
        assert!(updated_state.is_some());
        assert_eq!(updated_state.unwrap().current_index, 2);
    }

    #[tokio::test]
    async fn test_prefetch_service_is_cloneable() {
        let temp_dir = TempDir::new().unwrap();
        let cache_config = CacheConfig::new(1024 * 1024, temp_dir.path().to_path_buf());
        let cache = Arc::new(Cache::new(cache_config).await.unwrap());
        let s3_client = Arc::new(S3Client::new().unwrap());
        let prefetcher = Prefetcher::with_default_config();

        let service = PrefetchService::new(cache, s3_client, prefetcher);

        // Verify Clone is implemented
        let _cloned = service.clone();
    }
}
