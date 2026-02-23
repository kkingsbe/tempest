//! End-to-End test harness for the complete radar data pipeline.
//!
//! These tests verify the full pipeline: S3 fetch → decode → render data preparation
//! using a mock S3 server and real fixture data.

use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use chrono::{TimeZone, Utc};
use tempfile::TempDir;

use tempest_decode::{decode, VolumeScan};
use tempest_fetch::mock_s3::MockS3Server;
use tempest_fetch::{decompress_bz2, Cache, CacheConfig, S3Client};
use tempest_render_core::{
    colorize, get_station, polar_to_latlng, radar_color_table, types::LatLng, RadarMoment,
    RadarSentinel, Rgba,
};

// ============================================================================
// Test Harness State
// ============================================================================

/// Internal state for the test harness.
///
/// Exposes internal state for assertions in tests.
pub struct TestHarnessState {
    /// Current loaded station ID
    pub current_station: Option<String>,
    /// Number of decoded sweeps
    pub decoded_sweep_count: usize,
    /// Available moments (REF, VEL, SW, etc.)
    pub available_moments: Vec<String>,
    /// Timeline scan count
    pub timeline_scan_count: usize,
    /// Volume scans from pipeline
    pub volume_scans: Vec<VolumeScan>,
}

impl Default for TestHarnessState {
    fn default() -> Self {
        Self {
            current_station: None,
            decoded_sweep_count: 0,
            available_moments: Vec::new(),
            timeline_scan_count: 0,
            volume_scans: Vec::new(),
        }
    }
}

/// Test harness for the complete radar pipeline.
///
/// Provides utilities for setting up mock S3, fetching data,
/// decoding, and verifying render data.
pub struct AppTestHarness {
    /// Mock S3 server
    mock_server: Option<MockS3Server>,
    /// S3 client connected to mock server
    client: Option<S3Client>,
    /// Cache for fetched data
    cache: Option<Cache>,
    /// Temporary cache directory
    cache_dir: Option<TempDir>,
    /// Internal state for assertions
    pub state: Mutex<TestHarnessState>,
}

impl AppTestHarness {
    /// Create a new test harness.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mock_server = MockS3Server::new().await?;

        // Create S3 client pointing to mock server
        let client = S3Client::with_base_url(mock_server.url())?;

        // Create temporary cache directory
        let cache_dir = TempDir::new()?;
        let cache_config = CacheConfig::new(1024 * 1024, cache_dir.path().to_path_buf());
        let cache = Cache::new(cache_config).await?;

        Ok(Self {
            mock_server: Some(mock_server),
            client: Some(client),
            cache: Some(cache),
            cache_dir: Some(cache_dir),
            state: Mutex::new(TestHarnessState::default()),
        })
    }

    /// Get the mock server URL.
    pub fn mock_url(&self) -> String {
        self.mock_server.as_ref().expect("Mock server not initialized").url()
    }

    /// Register scan list for a station.
    pub fn register_scan_list(
        &mut self,
        station: &str,
        year: i32,
        month: u32,
        day: u32,
        scans: &[&str],
    ) {
        self.mock_server
            .as_mut()
            .expect("Mock server not initialized")
            .register_list_scans_response(station, year, month, day, scans);
    }

    /// Register raw scan data for download.
    pub fn register_scan_data(
        &mut self,
        station: &str,
        year: i32,
        month: u32,
        day: u32,
        filename: &str,
        data: Vec<u8>,
    ) {
        self.mock_server
            .as_mut()
            .expect("Mock server not initialized")
            .register_scan_data(station, year, month, day, filename, data);
    }

    /// Register compressed (bzip2) scan data.
    pub fn register_compressed_scan(
        &mut self,
        station: &str,
        year: i32,
        month: u32,
        day: u32,
        filename: &str,
        data: Vec<u8>,
    ) {
        self.mock_server
            .as_mut()
            .expect("Mock server not initialized")
            .register_compressed_scan_data(station, year, month, day, filename, data);
    }

    /// Fetch and decode a scan.
    pub async fn fetch_and_decode(
        &mut self,
        station: &str,
        filename: &str,
    ) -> Result<VolumeScan, Box<dyn std::error::Error>> {
        let client = self.client.as_ref().expect("S3 client not initialized");
        let cache = self.cache.as_mut().expect("Cache not initialized");

        // Create scan meta
        let scan_meta = self
            .mock_server
            .as_ref()
            .expect("Mock server not initialized")
            .create_scan_meta(station, filename);

        // Fetch from cache or mock server
        let data = client
            .fetch_scan_cached(station, &scan_meta, Some(cache))
            .await?;

        // Decode the data
        let volume = decode(&data)?;

        // Update state
        let mut state = self.state.lock().unwrap();
        state.current_station = Some(station.to_string());
        state.decoded_sweep_count = volume.sweeps.len();
        state.volume_scans.push(volume.clone());

        // Extract available moments
        let mut moments = Vec::new();
        if let Some(first_sweep) = volume.sweeps.first() {
            if let Some(first_radial) = first_sweep.radials.first() {
                if let Some(_gate) = first_radial.gates.first() {
                    // Check for various moments
                    if first_radial.gates.iter().any(|g| g.reflectivity.is_some()) {
                        moments.push("REF".to_string());
                    }
                    if first_radial.gates.iter().any(|g| g.velocity.is_some()) {
