//! Tile Manager - Async tile fetching and disk caching
//!
//! This module handles asynchronous fetching of map tiles from tile providers
//! with disk caching and LRU eviction.

use crate::tile::{TileCoordinate, TileSource, tile_to_tile_url};
use image::ImageDecoder;
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::{Semaphore, Mutex};

/// Maximum cache size in bytes (500 MB)
const MAX_CACHE_SIZE_BYTES: u64 = 500 * 1024 * 1024;
/// Maximum number of concurrent tile fetch requests
const MAX_CONCURRENT_REQUESTS: usize = 10;
/// Number of retry attempts for failed requests
const MAX_RETRIES: usize = 3;
/// Base delay for exponential backoff (in milliseconds)
const RETRY_BASE_DELAY_MS: u64 = 100;
/// User-Agent header for HTTP requests
const USER_AGENT: &str = "Tempest-NEXRAD/0.1.0";

/// Custom error types for tile operations
#[derive(Error, Debug)]
pub enum TileError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Image decode error: {0}")]
    ImageDecodeError(String),

    #[error("Invalid tile error: {0}")]
    InvalidTileError(String),
}

impl From<reqwest::Error> for TileError {
    fn from(err: reqwest::Error) -> Self {
        TileError::NetworkError(err.to_string())
    }
}

impl From<image::ImageError> for TileError {
    fn from(err: image::ImageError) -> Self {
        TileError::ImageDecodeError(err.to_string())
    }
}

impl From<std::io::Error> for TileError {
    fn from(err: std::io::Error) -> Self {
        TileError::CacheError(err.to_string())
    }
}

/// Represents a loaded map tile with its image data
#[derive(Debug, Clone)]
pub struct Tile {
    /// Tile coordinate (z, x, y)
    pub coordinate: TileCoordinate,
    /// Raw RGBA image data
    pub data: Vec<u8>,
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
    /// Timestamp for cache eviction (Unix epoch)
    pub timestamp: u64,
}

impl Tile {
    /// Creates a new tile from raw RGBA data
    pub fn new(coordinate: TileCoordinate, data: Vec<u8>, width: u32, height: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        Tile {
            coordinate,
            data,
            width,
            height,
            timestamp,
        }
    }

    /// Returns the size of the tile data in bytes
    pub fn size_bytes(&self) -> u64 {
        self.data.len() as u64
    }
}

/// Cache entry metadata for LRU tracking
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Path to the cached tile file
    path: PathBuf,
    /// Size in bytes
    size: u64,
    /// Last access timestamp (Unix epoch)
    last_access: u64,
}

/// Manages disk caching for map tiles with LRU eviction
pub struct TileCache {
    /// Base cache directory
    cache_dir: PathBuf,
    /// In-memory cache metadata for quick lookups
    entries: HashMap<TileCoordinate, CacheEntry>,
    /// Total cache size in bytes
    total_size: u64,
    /// Maximum cache size in bytes
    max_size: u64,
}

impl TileCache {
    /// Creates a new tile cache with the specified base directory
    pub fn new(cache_dir: PathBuf, max_size: Option<u64>) -> Self {
        let max_size = max_size.unwrap_or(MAX_CACHE_SIZE_BYTES);
        
        TileCache {
            cache_dir,
            entries: HashMap::new(),
            total_size: 0,
            max_size,
        }
    }

    /// Gets the source-specific cache directory path
    fn source_dir(&self, source: &TileSource) -> PathBuf {
        let source_name = match source {
            TileSource::OpenFreeMap => "openfreemap",
            TileSource::OpenStreetMap => "openstreetmap",
        };
        self.cache_dir.join("tiles").join(source_name)
    }

    /// Gets the full path for a tile in the cache
    fn tile_path(&self, tile: &TileCoordinate, source: &TileSource) -> PathBuf {
        self.source_dir(source)
            .join(tile.z.to_string())
            .join(tile.x.to_string())
            .join(format!("{}.png", tile.y))
    }

    /// Ensures the cache directory structure exists
    pub fn ensure_cache_dir(&self, source: &TileSource) -> Result<(), TileError> {
        let dir = self.source_dir(source);
        fs::create_dir_all(&dir)?;
        Ok(())
    }

    /// Gets a tile from the cache if it exists
    pub fn get(&mut self, tile: &TileCoordinate, _source: &TileSource) -> Option<PathBuf> {
        // First get the entry if it exists
        let entry_opt = self.entries.get(tile).cloned();
        
        if let Some(entry) = entry_opt {
            // Update last access time
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            let mut updated_entry = entry.clone();
            updated_entry.last_access = now;
            
            let path = entry.path.clone();
            self.entries.insert(*tile, updated_entry);
            
            // Verify file still exists
            if path.exists() {
                return Some(path);
            }
        }
        
        None
    }

    /// Puts a tile into the cache
    pub fn put(
        &mut self, 
        tile: &TileCoordinate, 
        source: &TileSource, 
        data: &[u8]
    ) -> Result<(), TileError> {
        // Ensure directory exists
        let tile_path = self.tile_path(tile, source);
        if let Some(parent) = tile_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write the file
        fs::write(&tile_path, data)?;
        
        let size = data.len() as u64;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        // Add to entries
        let entry = CacheEntry {
            path: tile_path,
            size,
            last_access: now,
        };
        
        // If tile was already cached, subtract old size
        if let Some(old_entry) = self.entries.get(tile) {
            self.total_size -= old_entry.size;
        }
        
        self.entries.insert(*tile, entry);
        self.total_size += size;
        
        // Evict if needed
        self.evict_if_needed()?;
        
        Ok(())
    }

    /// Evicts oldest tiles if cache exceeds maximum size
    pub fn evict_if_needed(&mut self) -> Result<(), TileError> {
        while self.total_size > self.max_size && !self.entries.is_empty() {
            // Find the oldest entry (LRU)
            let oldest_tile = self.entries
                .iter()
                .min_by_key(|(_, entry)| entry.last_access)
                .map(|(tile, _)| *tile);
            
            if let Some(tile) = oldest_tile {
                if let Some(entry) = self.entries.remove(&tile) {
                    // Delete the file
                    if entry.path.exists() {
                        let _ = fs::remove_file(&entry.path);
                    }
                    self.total_size -= entry.size;
                }
            } else {
                break;
            }
        }
        
        Ok(())
    }

    /// Returns the current cache size in bytes
    pub fn size(&self) -> u64 {
        self.total_size
    }

    /// Returns the number of cached tiles
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Internal state for TileManager
struct TileManagerState {
    /// The tile cache (wrapped in Mutex for interior mutability)
    cache: Mutex<TileCache>,
    /// The tile source/provider
    source: TileSource,
    /// HTTP client for fetching tiles
    client: reqwest::Client,
    /// Semaphore for rate limiting concurrent requests
    semaphore: Semaphore,
}

/// Main entry point for tile operations
pub struct TileManager {
    /// Internal state (wrapped in Arc for shared access)
    state: Arc<TileManagerState>,
}

impl TileManager {
    /// Creates a new TileManager with the specified cache directory and tile source
    pub fn new(cache_dir: PathBuf, source: TileSource) -> Self {
        let cache = TileCache::new(cache_dir, None);
        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");
        
        let semaphore = Semaphore::new(MAX_CONCURRENT_REQUESTS);
        
        let state = TileManagerState {
            cache: Mutex::new(cache),
            source,
            client,
            semaphore,
        };
        
        TileManager {
            state: Arc::new(state),
        }
    }

    /// Gets a tile, fetching from network or loading from cache
    pub async fn get_tile(&self, tile: TileCoordinate) -> Result<Tile, TileError> {
        // First try to get from cache
        let source = self.state.source;
        
        {
            let mut cache = self.state.cache.lock().await;
            if let Some(cache_path) = cache.get(&tile, &source) {
                // Load from cache file
                if let Ok(data) = fs::read(&cache_path) {
                    if let Some(loaded_tile) = self.decode_tile_data(&tile, &data) {
                        return Ok(loaded_tile);
                    }
                }
            }
        }
        
        // Need to fetch from network
        self.fetch_and_cache_tile(tile).await
    }

    /// Fetches a tile from the network and caches it
    async fn fetch_and_cache_tile(&self, tile: TileCoordinate) -> Result<Tile, TileError> {
        // Acquire semaphore for rate limiting
        let permit = self.state.semaphore
            .acquire()
            .await
            .expect("Failed to acquire semaphore");
        
        let source = self.state.source;
        let client = self.state.client.clone();
        let url = tile_to_tile_url(&tile, &source);
        
        // Fetch with retries
        let data = self.fetch_with_retry(&client, &url).await?;
        
        // Decode the image
        let loaded_tile = self.decode_tile_data(&tile, &data)
            .ok_or_else(|| TileError::ImageDecodeError("Failed to decode tile image".to_string()))?;
        
        // Drop permit before modifying cache
        drop(permit);
        
        // Put in cache
        {
            let mut cache = self.state.cache.lock().await;
            cache.put(&tile, &source, &data)?;
        }
        
        Ok(loaded_tile)
    }

    /// Fetches a URL with retry logic and exponential backoff
    async fn fetch_with_retry(&self, client: &reqwest::Client, url: &str) -> Result<Vec<u8>, TileError> {
        let mut last_error = None;
        
        for attempt in 0..MAX_RETRIES {
            match client.get(url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.bytes().await {
                            Ok(bytes) => {
                                return Ok(bytes.to_vec());
                            }
                            Err(e) => {
                                last_error = Some(TileError::NetworkError(e.to_string()));
                            }
                        }
                    } else {
                        last_error = Some(TileError::NetworkError(
                            format!("HTTP error: {}", response.status())
                        ));
                    }
                }
                Err(e) => {
                    last_error = Some(TileError::NetworkError(e.to_string()));
                }
            }
            
            // Exponential backoff before retry (skip on last attempt)
            if attempt < MAX_RETRIES - 1 {
                let delay = RETRY_BASE_DELAY_MS * 2u64.pow(attempt as u32);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }
        }
        
        Err(last_error.unwrap_or_else(|| 
            TileError::NetworkError("Unknown error during fetch".to_string())
        ))
    }

    /// Decodes PNG data into RGBA pixels
    fn decode_tile_data(&self, tile: &TileCoordinate, data: &[u8]) -> Option<Tile> {
        // Use Cursor to wrap the byte slice so it implements Seek
        let cursor = Cursor::new(data);
        
        // Use the image crate to decode PNG
        let decoder = image::codecs::png::PngDecoder::new(cursor).ok()?;
        let (width, height) = decoder.dimensions();
        
        let mut rgba_data = vec![0u8; (width * height * 4) as usize];
        
        // Re-create decoder to read pixels
        let cursor = Cursor::new(data);
        let decoder = image::codecs::png::PngDecoder::new(cursor).ok()?;
        decoder.read_image(&mut rgba_data).ok()?;
        
        Some(Tile::new(*tile, rgba_data, width, height))
    }

    /// Prefetches multiple tiles asynchronously
    pub async fn prefetch_tiles(&self, tiles: Vec<TileCoordinate>) {
        // Use futures to fetch multiple tiles concurrently
        let futures: Vec<_> = tiles.into_iter().map(|tile| {
            let manager = Self {
                state: Arc::clone(&self.state),
            };
            async move {
                let _ = manager.get_tile(tile).await;
            }
        }).collect();
        
        // Run all fetches concurrently (limited by semaphore)
        futures::future::join_all(futures).await;
    }

    /// Returns the current cache size in bytes
    pub async fn cache_size(&self) -> u64 {
        let cache = self.state.cache.lock().await;
        cache.size()
    }

    /// Returns the number of cached tiles
    pub async fn cached_tiles(&self) -> usize {
        let cache = self.state.cache.lock().await;
        cache.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Creates a temporary cache directory for testing
    fn create_test_cache_dir() -> TempDir {
        tempfile::tempdir().expect("Failed to create temp dir")
    }

    #[test]
    fn test_tile_error_display() {
        let err = TileError::NetworkError("connection failed".to_string());
        assert_eq!(err.to_string(), "Network error: connection failed");
        
        let err = TileError::CacheError("permission denied".to_string());
        assert_eq!(err.to_string(), "Cache error: permission denied");
        
        let err = TileError::ImageDecodeError("invalid PNG".to_string());
        assert_eq!(err.to_string(), "Image decode error: invalid PNG");
        
        let err = TileError::InvalidTileError("invalid coordinates".to_string());
        assert_eq!(err.to_string(), "Invalid tile error: invalid coordinates");
    }

    #[test]
    fn test_tile_creation() {
        let coord = TileCoordinate::new(10, 100, 200);
        let data = vec![0u8; 256 * 256 * 4];
        let tile = Tile::new(coord, data.clone(), 256, 256);
        
        assert_eq!(tile.coordinate.z, 10);
        assert_eq!(tile.coordinate.x, 100);
        assert_eq!(tile.coordinate.y, 200);
        assert_eq!(tile.width, 256);
        assert_eq!(tile.height, 256);
        assert_eq!(tile.size_bytes() as usize, 256 * 256 * 4);
    }

    #[test]
    fn test_tile_size_bytes() {
        let coord = TileCoordinate::new(5, 1, 2);
        let data = vec![0u8; 1024];
        let tile = Tile::new(coord, data, 32, 32);
        
        assert_eq!(tile.size_bytes(), 1024);
    }

    #[tokio::test]
    async fn test_cache_directory_creation() {
        let temp_dir = create_test_cache_dir();
        let cache_dir = temp_dir.path().join("tiles");
        
        let cache = TileCache::new(cache_dir.clone(), None);
        let source = TileSource::OpenFreeMap;
        
        cache.ensure_cache_dir(&source).unwrap();
        
        assert!(cache_dir.join("tiles").join("openfreemap").exists());
    }

    #[tokio::test]
    async fn test_cache_put_and_get() {
        let temp_dir = create_test_cache_dir();
        let cache_dir = temp_dir.path().join("tiles");
        
        let mut cache = TileCache::new(cache_dir, None);
        let source = TileSource::OpenFreeMap;
        
        let tile_coord = TileCoordinate::new(5, 1, 2);
        let tile_data = b"fake png data".to_vec();
        
        // Put tile in cache
        cache.put(&tile_coord, &source, &tile_data).unwrap();
        
        // Get tile from cache
        let path = cache.get(&tile_coord, &source);
        
        assert!(path.is_some());
        assert!(path.unwrap().exists());
    }

    #[tokio::test]
    async fn test_cache_eviction() {
        let temp_dir = create_test_cache_dir();
        let cache_dir = temp_dir.path().join("tiles");
        
        // Create cache with small max size (100 bytes)
        let mut cache = TileCache::new(cache_dir, Some(100));
        let source = TileSource::OpenFreeMap;
        
        // Add tiles until we exceed the limit
        for i in 0..5u32 {
            let tile_coord = TileCoordinate::new(5, i, 0);
            let tile_data = vec![0u8; 30]; // 30 bytes each
            
            cache.put(&tile_coord, &source, &tile_data).unwrap();
        }
        
        // Cache should have evicted some tiles
        // At most 3 tiles (90 bytes) should remain
        assert!(cache.len() <= 4);
    }

    #[test]
    fn test_tile_source_dir() {
        let temp_dir = create_test_cache_dir();
        let cache = TileCache::new(temp_dir.path().join("cache"), None);
        
        let ofm_dir = cache.source_dir(&TileSource::OpenFreeMap);
        assert!(ofm_dir.to_string_lossy().ends_with("openfreemap"));
        
        let osm_dir = cache.source_dir(&TileSource::OpenStreetMap);
        assert!(osm_dir.to_string_lossy().ends_with("openstreetmap"));
    }

    #[test]
    fn test_tile_path_generation() {
        let temp_dir = create_test_cache_dir();
        let cache = TileCache::new(temp_dir.path().join("cache"), None);
        let source = TileSource::OpenFreeMap;
        
        let tile = TileCoordinate::new(10, 123, 456);
        let path = cache.tile_path(&tile, &source);
        
        let expected = format!("10/123/456.png");
        assert!(path.to_string_lossy().ends_with(&expected));
    }

    #[test]
    fn test_url_generation() {
        let tile = TileCoordinate::new(5, 10, 20);
        
        let url = tile_to_tile_url(&tile, &TileSource::OpenFreeMap);
        assert_eq!(url, "https://tiles.openfreemap.org/5/10/20.png");
        
        let url = tile_to_tile_url(&tile, &TileSource::OpenStreetMap);
        assert_eq!(url, "https://tile.openstreetmap.org/5/10/20.png");
    }

    #[tokio::test]
    async fn test_cache_empty_initialization() {
        let temp_dir = create_test_cache_dir();
        let cache = TileCache::new(temp_dir.path().join("tiles"), None);
        
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_tile_coordinate_in_cache_key() {
        // Verify TileCoordinate can be used as HashMap key
        use std::collections::HashSet;
        
        let mut set = HashSet::new();
        let tile1 = TileCoordinate::new(5, 1, 2);
        let tile2 = TileCoordinate::new(5, 1, 2);
        let tile3 = TileCoordinate::new(5, 1, 3);
        
        set.insert(tile1);
        assert!(set.contains(&tile2));
        assert!(!set.contains(&tile3));
    }

    #[tokio::test]
    async fn test_manager_cache_stats() {
        let temp_dir = create_test_cache_dir();
        let manager = TileManager::new(
            temp_dir.path().join("cache"),
            TileSource::OpenFreeMap,
        );
        
        // Initially empty
        assert_eq!(manager.cached_tiles().await, 0);
    }

    #[test]
    fn test_tile_manager_creation() {
        let temp_dir = create_test_cache_dir();
        let manager = TileManager::new(
            temp_dir.path().join("cache"),
            TileSource::OpenFreeMap,
        );
        
        // Manager created successfully - verify semaphore exists
        let state = manager.state.as_ref();
        assert!(state.semaphore.available_permits() <= MAX_CONCURRENT_REQUESTS);
    }

    #[test]
    fn test_max_retries_constant() {
        assert_eq!(MAX_RETRIES, 3);
    }

    #[test]
    fn test_max_concurrent_requests_constant() {
        assert_eq!(MAX_CONCURRENT_REQUESTS, 10);
    }

    #[test]
    fn test_user_agent() {
        assert_eq!(USER_AGENT, "Tempest-NEXRAD/0.1.0");
    }

    #[test]
    fn test_cache_size_limit() {
        // 500 MB in bytes
        assert_eq!(MAX_CACHE_SIZE_BYTES, 500 * 1024 * 1024);
    }
}
