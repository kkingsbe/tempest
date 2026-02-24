//! Local disk cache with LRU eviction for radar data.

use crate::error::FetchError;
use crate::types::CacheStats;
use chrono::{DateTime, Utc};
use lru::LruCache;
use std::fs;
use std::io::{Read, Write};
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for the cache.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum size of the cache in bytes.
    pub max_size_bytes: u64,
    /// Directory to store cached files.
    pub cache_dir: PathBuf,
}

impl CacheConfig {
    /// Create a new CacheConfig.
    pub fn new(max_size_bytes: u64, cache_dir: PathBuf) -> Self {
        Self {
            max_size_bytes,
            cache_dir,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        // Get home directory, fallback to /tmp if not available
        let cache_dir = std::env::var("HOME")
            .map(|home| PathBuf::from(home).join(".config/tempest/cache"))
            .unwrap_or_else(|_| PathBuf::from("/tmp/tempest-cache"));

        Self {
            // Default cache size: 1GB
            max_size_bytes: 1_073_741_824,
            cache_dir,
        }
    }
}

/// Create a new Cache with default configuration.
///
/// Uses a default cache directory of `~/.config/tempest/cache/`
/// and a maximum cache size of 1GB.
///
/// # Errors
///
/// Returns an error if the cache directory cannot be created.
pub async fn cache_default() -> Result<Cache, FetchError> {
    Cache::new(CacheConfig::default()).await
}

/// A cache entry with metadata.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct CacheEntry {
    /// The key associated with this entry.
    key: String,
    /// Size of the data in bytes.
    size: u64,
    /// Last time this entry was accessed.
    last_accessed: DateTime<Utc>,
}

impl CacheEntry {
    /// Create a new CacheEntry.
    fn new(key: String, size: u64) -> Self {
        Self {
            key,
            size,
            last_accessed: Utc::now(),
        }
    }
}

/// Thread-safe LRU cache with disk persistence.
#[derive(Debug)]
pub struct Cache {
    config: CacheConfig,
    /// In-memory LRU index.
    lru: Arc<RwLock<LruCache<String, CacheEntry>>>,
    /// Current total size of cached data.
    current_size: Arc<RwLock<u64>>,
}

impl Cache {
    /// Create a new Cache instance.
    pub async fn new(config: CacheConfig) -> Result<Self, FetchError> {
        // Ensure cache directory exists
        fs::create_dir_all(&config.cache_dir)
            .map_err(|e| FetchError::cache(format!("Failed to create cache directory: {}", e)))?;

        // Create the LRU cache with a large capacity for the in-memory index
        let capacity = NonZeroUsize::new(10000)
            .ok_or_else(|| FetchError::cache("Capacity must be non-zero"))?;
        let lru = LruCache::new(capacity);

        Ok(Self {
            config,
            lru: Arc::new(RwLock::new(lru)),
            current_size: Arc::new(RwLock::new(0)),
        })
    }

    /// Get data from the cache by key.
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, FetchError> {
        let mut lru = self.lru.write().await;

        // Check if the key exists in the LRU cache
        if let Some(entry) = lru.get(key) {
            // Update the last accessed time
            let mut updated_entry = entry.clone();
            updated_entry.last_accessed = Utc::now();
            lru.put(key.to_string(), updated_entry);

            // Read from disk
            let path = self.cache_path(key);
            let mut file = fs::File::open(&path)
                .map_err(|e| FetchError::cache(format!("Failed to open cache file: {}", e)))?;

            let mut data = Vec::new();
            file.read_to_end(&mut data)
                .map_err(|e| FetchError::cache(format!("Failed to read cache file: {}", e)))?;

            tracing::debug!("Cache hit for key: {}", key);
            return Ok(Some(data));
        }

        tracing::debug!("Cache miss for key: {}", key);
        Ok(None)
    }

    /// Put data into the cache.
    pub async fn put(&mut self, key: &str, data: Vec<u8>) -> Result<(), FetchError> {
        let size = data.len() as u64;
        let mut current_size = self.current_size.write().await;

        // Evict entries until we have enough space
        while *current_size + size > self.config.max_size_bytes && !self.lru.read().await.is_empty()
        {
            drop(current_size);
            self.evict_lru().await?;
            current_size = self.current_size.write().await;
        }

        // Write to disk
        let path = self.cache_path(key);
        let mut file = fs::File::create(&path)
            .map_err(|e| FetchError::cache(format!("Failed to create cache file: {}", e)))?;

        file.write_all(&data)
            .map_err(|e| FetchError::cache(format!("Failed to write cache file: {}", e)))?;

        // Update LRU index
        let mut lru = self.lru.write().await;
        let key_owned = key.to_string();
        let entry = CacheEntry::new(key_owned.clone(), size);
        lru.put(key_owned, entry);

        // Update current size
        *current_size += size;

        tracing::debug!(
            "Cached key: {} ({} bytes, total: {} bytes)",
            key,
            size,
            *current_size
        );

        Ok(())
    }

    /// Check if the cache contains a key.
    pub async fn contains(&self, key: &str) -> bool {
        self.lru.read().await.contains(key)
    }

    /// Evict the least recently used entry from the cache.
    pub async fn evict_lru(&mut self) -> Result<(), FetchError> {
        // Get the oldest entry from the LRU cache
        // pop_lru removes the least recently used entry
        let mut lru = self.lru.write().await;

        if let Some((key, entry)) = lru.pop_lru() {
            // Delete the file from disk
            let path = self.cache_path(&key);
            if path.exists() {
                fs::remove_file(&path).map_err(|e| {
                    FetchError::cache(format!("Failed to remove cache file: {}", e))
                })?;
            }

            // Update the size counter
            let mut current_size = self.current_size.write().await;
            *current_size = current_size.saturating_sub(entry.size);

            tracing::debug!("Evicted LRU entry: {} ({} bytes)", key, entry.size);
        }

        Ok(())
    }

    /// Clear all entries from the cache (both memory and disk).
    pub async fn clear(&mut self) -> Result<(), FetchError> {
        // Clear disk
        let entries = fs::read_dir(&self.config.cache_dir)
            .map_err(|e| FetchError::cache(format!("Failed to read cache directory: {}", e)))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "cache") {
                fs::remove_file(&path).map_err(|e| {
                    FetchError::cache(format!("Failed to remove cache file: {}", e))
                })?;
            }
        }

        // Clear memory index
        let mut lru = self.lru.write().await;
        lru.clear();

        // Reset size
        let mut current_size = self.current_size.write().await;
        *current_size = 0;

        tracing::info!("Cache cleared");
        Ok(())
    }

    /// Get statistics about the cache.
    pub async fn stats(&self) -> CacheStats {
        let lru = self.lru.read().await;
        let current_size = *self.current_size.read().await;

        let entry_count = lru.len();

        // Find oldest and newest entries
        let mut oldest: Option<DateTime<Utc>> = None;
        let mut newest: Option<DateTime<Utc>> = None;

        for entry in lru.iter() {
            let accessed = entry.1.last_accessed;
            oldest = Some(oldest.map_or(accessed, |o| accessed.min(o)));
            newest = Some(newest.map_or(accessed, |n| accessed.max(n)));
        }

        CacheStats {
            total_size: current_size,
            entry_count,
            oldest,
            newest,
        }
    }

    /// Get the cache file path for a key.
    fn cache_path(&self, key: &str) -> PathBuf {
        // Sanitize the key to be a valid filename
        let safe_key = key.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        self.config.cache_dir.join(format!("{}.cache", safe_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_config(temp_dir: &TempDir) -> CacheConfig {
        CacheConfig::new(1024 * 1024, temp_dir.path().to_path_buf())
    }

    #[tokio::test]
    async fn test_cache_basic_get_put() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let mut cache = Cache::new(config).await.unwrap();

        // Put some data
        let key = "test_key";
        let data = b"Hello, World!".to_vec();

        cache.put(key, data.clone()).await.unwrap();

        // Get the data back
        let retrieved = cache.get(key).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), data);

        // Check stats
        let stats = cache.stats().await;
        assert_eq!(stats.entry_count, 1);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let cache = Cache::new(config).await.unwrap();

        let result = cache.get("nonexistent").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_cache_contains() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let mut cache = Cache::new(config).await.unwrap();

        let key = "test_key";
        let data = b"test data".to_vec();

        assert!(!cache.contains(key).await);

        cache.put(key, data).await.unwrap();

        assert!(cache.contains(key).await);
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let temp_dir = TempDir::new().unwrap();
        // Set a small max size
        let config = CacheConfig::new(100, temp_dir.path().to_path_buf());
        let mut cache = Cache::new(config).await.unwrap();

        // Add entries until we exceed the limit
        for i in 0..10 {
            let key = format!("key_{}", i);
            let data = vec![i as u8; 50]; // 50 bytes each
            cache.put(&key, data).await.unwrap();
        }

        // Should have evicted some entries
        let stats = cache.stats().await;
        assert!(stats.entry_count < 10);
        assert!(stats.total_size <= 100);
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let mut cache = Cache::new(config).await.unwrap();

        // Add some data
        cache.put("key1", b"data1".to_vec()).await.unwrap();
        cache.put("key2", b"data2".to_vec()).await.unwrap();

        // Clear the cache
        cache.clear().await.unwrap();

        // Check stats
        let stats = cache.stats().await;
        assert_eq!(stats.entry_count, 0);
        assert_eq!(stats.total_size, 0);
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(&temp_dir);
        let mut cache = Cache::new(config).await.unwrap();

        // Initially empty
        let stats = cache.stats().await;
        assert!(stats.is_empty());
        assert!(stats.oldest.is_none());
        assert!(stats.newest.is_none());

        // Add entries
        cache.put("key1", b"data1".to_vec()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        cache.put("key2", b"data2".to_vec()).await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entry_count, 2);
        assert!(stats.oldest.is_some());
        assert!(stats.newest.is_some());
    }

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();

        // Verify default max size is 1GB
        assert_eq!(config.max_size_bytes, 1_073_741_824);

        // Verify default cache directory contains expected path
        let cache_dir_str = config.cache_dir.to_string_lossy();
        assert!(
            cache_dir_str.contains(".config/tempest/cache")
                || cache_dir_str.contains("tempest-cache")
        );
    }

    #[tokio::test]
    async fn test_cache_default_creation() {
        // This test verifies we can create a Cache with default config
        // Use a temp directory to avoid polluting the real cache
        let temp_dir = TempDir::new().unwrap();
        let config = CacheConfig {
            max_size_bytes: CacheConfig::default().max_size_bytes,
            cache_dir: temp_dir.path().to_path_buf(),
        };

        let cache = Cache::new(config).await.unwrap();

        // Verify it's functional
        let stats = cache.stats().await;
        assert!(stats.is_empty());
    }
}
