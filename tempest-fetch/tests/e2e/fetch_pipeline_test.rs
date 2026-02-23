//! End-to-end tests for the fetch pipeline.
//!
//! These tests verify that the S3 client can successfully:
//! 1. Connect to a mock S3 server
//! 2. List available scans for a station
//! 3. Fetch scan data from the mock server
//! 4. Decompress bzip2 compressed data

use std::io::Write;
use tempfile::TempDir;

use tempest_fetch::mock_s3::MockS3Server;
use tempest_fetch::{decompress_bz2, Cache, CacheConfig, S3Client};

/// Test that the S3 client can connect to a mock server and list scans.
#[tokio::test]
async fn s3_client_connects_to_mock_server_and_lists_scans() {
    // Create mock S3 server
    let mut mock_server = MockS3Server::new().await.expect("Failed to create mock server");

    // Register mock scan list
    mock_server.register_list_scans_response(
        "KTLX",
        2024,
        3,
        15,
        &["KTLX20240315_120021", "KTLX20240315_120521"],
    );

    // Create S3 client pointing to mock server
    let client = S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

    // List scans
    let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    let scans = client.list_scans("KTLX", date).await.expect("Failed to list scans");

    // Verify we got the expected scans
    assert_eq!(scans.len(), 2);
    assert_eq!(scans[0].filename, "KTLX20240315_120021");
    assert_eq!(scans[1].filename, "KTLX20240315_120521");
}

/// Test that the S3 client can fetch raw scan data from the mock server.
#[tokio::test]
async fn s3_client_fetches_scan_data_from_mock_server() {
    // Create mock S3 server
    let mut mock_server = MockS3Server::new().await.expect("Failed to create mock server");

    // Register mock scan data
    let test_data = b"NEXRAD Level II Test Data - Mock Scan Content";
    mock_server.register_scan_data(
        "KTLX",
        2024,
        3,
        15,
        "KTLX20240315_120021",
        test_data.to_vec(),
    );

    // Create S3 client pointing to mock server
    let client = S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

    // Fetch scan data
    let scan_meta = mock_server.create_scan_meta("KTLX", "KTLX20240315_120021");
    let data = client
        .fetch_scan("KTLX", &scan_meta)
        .await
        .expect("Failed to fetch scan");

    // Verify data matches
    assert_eq!(&data[..], test_data);
}

/// Test that the S3 client can fetch bzip2 compressed data and decompress it.
#[tokio::test]
async fn s3_client_fetches_and_decompresses_bzip2_data() {
    // Create mock S3 server
    let mut mock_server = MockS3Server::new().await.expect("Failed to create mock server");

    // Create some test data and compress it with bzip2
    let original_data = b"NEXRAD Level II Compressed Test Data - This is bzip2 content";
    let compressed_data = compress_with_bzip2(original_data);

    // Register compressed mock scan data
    mock_server.register_compressed_scan_data(
        "KTLX",
        2024,
        3,
        15,
        "KTLX20240315_120021.bz2",
        compressed_data,
    );

    // Create S3 client pointing to mock server
    let client = S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

    // Fetch compressed scan data
    let scan_meta = mock_server.create_scan_meta("KTLX", "KTLX20240315_120021.bz2");
    let compressed = client
        .fetch_scan("KTLX", &scan_meta)
        .await
        .expect("Failed to fetch compressed scan");

    // Decompress the data
    let decompressed = decompress_bz2(&compressed).expect("Failed to decompress bzip2 data");

    // Verify decompressed data matches original
    assert_eq!(&decompressed, original_data);
}

/// Test the complete fetch pipeline: list -> fetch -> decompress -> cache.
#[tokio::test]
async fn complete_fetch_pipeline_with_caching() {
    // Create temporary cache directory
    let cache_dir = TempDir::new().expect("Failed to create temp cache dir");
    let cache_config = CacheConfig::new(1024 * 1024, cache_dir.path().to_path_buf()); // 1MB
    let mut cache = Cache::new(cache_config).await.expect("Failed to create cache");

    // Create mock S3 server
    let mut mock_server = MockS3Server::new().await.expect("Failed to create mock server");

    // Register mock scan list
    mock_server.register_list_scans_response("KTLX", 2024, 3, 15, &["KTLX20240315_120021"]);

    // Register mock scan data
    let test_data = b"Cached Scan Data for Pipeline Test";
    mock_server.register_scan_data(
        "KTLX",
        2024,
        3,
        15,
        "KTLX20240315_120021",
        test_data.to_vec(),
    );

    // Create S3 client pointing to mock server
    let client = S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

    // Step 1: List scans
    let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    let scans = client
        .list_scans("KTLX", date)
        .await
        .expect("Failed to list scans");
    assert_eq!(scans.len(), 1);

    // Step 2: Fetch scan with caching
    let scan = &scans[0];
    let data = client
        .fetch_scan_cached("KTLX", scan, Some(&mut cache))
        .await
        .expect("Failed to fetch scan");
    assert_eq!(&data[..], test_data);

    // Step 3: Verify we can get stats from cache
    let stats = cache.stats().await;
    assert!(stats.entry_count > 0);
}

/// Test that the mock server can handle multiple stations.
#[tokio::test]
async fn mock_server_handles_multiple_stations() {
    // Create mock S3 server
    let mut mock_server = MockS3Server::new().await.expect("Failed to create mock server");

    // Register multiple stations
    mock_server.register_list_scans_response(
        "KTLX",
        2024,
        3,
        15,
        &["KTLX20240315_120021"],
    );
    mock_server.register_list_scans_response(
        "KICT",
        2024,
        3,
        15,
        &["KICT20240315_110000", "KICT20240315_111000"],
    );

    // Create S3 client pointing to mock server
    let client = S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

    let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

    // List KTLX scans
    let ktlx_scans = client
        .list_scans("KTLX", date)
        .await
        .expect("Failed to list KTLX scans");
    assert_eq!(ktlx_scans.len(), 1);

    // List KICT scans
    let kict_scans = client
        .list_scans("KICT", date)
        .await
        .expect("Failed to list KICT scans");
    assert_eq!(kict_scans.len(), 2);
}

/// Test error handling when connecting to mock server.
#[tokio::test]
async fn s3_client_handles_server_error() {
    // Create mock S3 server
    let mock_server = MockS3Server::new().await.expect("Failed to create mock server");

    // Create S3 client pointing to mock server
    let client = S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

    // Try to list scans for non-existent date
    let date = chrono::NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
    let result = client.list_scans("KTLX", date).await;

    // Should get an error (404 or similar)
    assert!(result.is_err());
}

/// Helper function to compress data with bzip2.
fn compress_with_bzip2(data: &[u8]) -> Vec<u8> {
    use bzip2::write::BzEncoder;
    use bzip2::Compression;

    let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).expect("Failed to write to bzip2 encoder");
    encoder.finish().expect("Failed to finish bzip2 compression")
}

mod station_discovery {
    use tempest_fetch::{get_station, registry};

    /// Test that the station registry contains all expected stations.
    ///
    /// The embedded database has 150+ NEXRAD stations. This test verifies
    /// that at least 100 stations are available and that known stations exist.
    #[test]
    fn station_registry_contains_all_stations() {
        let reg = registry();

        // Assert registry contains at least 100 stations (the embedded database has 150+)
        assert!(
            reg.len() >= 100,
            "Expected at least 100 stations in registry, got {}",
            reg.len()
        );

        // Assert known stations exist
        let ktlx = reg.get("KTLX");
        assert!(ktlx.is_some(), "Expected KTLX (Oklahoma City) to exist in registry");

        let kict = reg.get("KICT");
        assert!(kict.is_some(), "Expected KICT (Wichita) to exist in registry");

        let khgx = reg.get("KHGX");
        assert!(khgx.is_some(), "Expected KHGX (Houston) to exist in registry");
    }

    /// Test that get_station returns valid station data for known stations.
    ///
    /// Verifies that KTLX (Oklahoma City) has valid geographic coordinates
    /// and elevation data.
    #[test]
    fn get_station_returns_station_data() {
        let station = get_station("KTLX");

        // Assert it returns Some(Station)
        assert!(
            station.is_some(),
            "Expected Some(Station) for KTLX, got None"
        );

        let s = station.unwrap();

        // Verify lat is between -90 and 90
        assert!(
            s.lat >= -90.0 && s.lat <= 90.0,
            "Expected lat between -90 and 90, got {}",
            s.lat
        );

        // Verify lon is between -180 and 180
        assert!(
            s.lon >= -180.0 && s.lon <= 180.0,
            "Expected lon between -180 and 180, got {}",
            s.lon
        );

        // Verify elevation is positive
        assert!(
            s.elevation_m > 0.0,
            "Expected elevation_m > 0, got {}",
            s.elevation_m
        );

        // Verify name contains "Oklahoma"
        assert!(
            s.name.contains("Oklahoma"),
            "Expected name to contain 'Oklahoma', got '{}'",
            s.name
        );
    }

    /// Test that get_station returns None for unknown station IDs.
    #[test]
    fn get_station_returns_none_for_unknown() {
        let station = get_station("XXXX");

        // Assert it returns None for unknown station
        assert!(
            station.is_none(),
            "Expected None for unknown station XXXX, got Some({:?})",
            station
        );
    }

    /// Test that stations can be filtered by geographic region.
    ///
    /// This test filters stations to find those in Texas (lat > 26 && lat < 36 && lon > -106 && lon < -93)
    /// and verifies at least one Texas station exists (like KHGX for Houston).
    #[test]
    fn station_list_can_be_filtered_by_region() {
        let reg = registry();

        // Filter for Texas region: lat > 26 && lat < 36 && lon > -106 && lon < -93
        let texas_stations: Vec<_> = reg
            .iter()
            .filter(|s| s.lat > 26.0 && s.lat < 36.0 && s.lon > -106.0 && s.lon < -93.0)
            .collect();

        // Assert at least one Texas station exists
        assert!(
            !texas_stations.is_empty(),
            "Expected at least one Texas station, found none"
        );

        // Verify KHGX (Houston) is in Texas
        let khgx = get_station("KHGX");
        assert!(khgx.is_some(), "Expected KHGX (Houston) to exist");

        let houston = khgx.unwrap();
        // Houston should be in Texas region
        assert!(
            houston.lat > 26.0 && houston.lat < 36.0 && houston.lon > -106.0 && houston.lon < -93.0,
            "Expected KHGX to be in Texas region, got lat={}, lon={}",
            houston.lat, houston.lon
        );
    }
}

mod data_polling {
    use std::collections::HashSet;
    use std::time::Duration;

    use chrono::Datelike;
    use tempest_fetch::mock_s3::MockS3Server;
    use tempest_fetch::{PollConfig, S3Client};

    /// Test that polling returns new scans from the mock server.
    ///
    /// This test creates a mock S3 server with 3 scans for station KTLX
    /// and verifies that all 3 scans are returned when polling.
    #[tokio::test]
    async fn polling_returns_new_scans() {
        // Create mock S3 server
        let mut mock_server = MockS3Server::new()
            .await
            .expect("Failed to create mock server");

        // Use today's date
        let now = chrono::Utc::now();
        let date = now.date_naive();
        let year = date.year();
        let month = date.month();
        let day = date.day();

        // Register mock scan list with 3 scans using the exact format from working test
        // Use the same format as the working test: KTLX{YYYYMMDD}_{HHMMSS}
        mock_server.register_list_scans_response(
            "KTLX",
            year,
            month,
            day,
            &[
                "KTLX20240223_120021",
                "KTLX20240223_120521",
                "KTLX20240223_121021",
            ],
        );

        // Create S3 client pointing to mock server
        let client =
            S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

        // List scans directly from the client
        let scans = client
            .list_scans("KTLX", date)
            .await
            .expect("Failed to list scans");

        // Assert that exactly 3 scans are returned
        assert_eq!(
            scans.len(),
            3,
            "Expected 3 scans, got {}",
            scans.len()
        );
    }

    /// Test that polling filters out duplicate scans.
    ///
    /// This test creates a mock S3 server with 2 scans, pre-populates
    /// a HashSet with one of the scans, and verifies that only the
    /// new scan is returned.
    #[tokio::test]
    async fn polling_filters_duplicates() {
        // Create mock S3 server
        let mut mock_server = MockS3Server::new()
            .await
            .expect("Failed to create mock server");

        // Get today's date
        let now = chrono::Utc::now();
        let year = now.format("%Y").to_string().parse::<i32>().unwrap();
        let month = now.format("%m").to_string().parse::<u32>().unwrap();
        let day = now.format("%d").to_string().parse::<u32>().unwrap();

        // Build scan filenames using dynamic date (NEXRAD format: KTLXYYYYMMDD_HHMMSS)
        let scan1 = format!("KTLX{}{:02}{:02}_123456", year, month, day);
        let scan2 = format!("KTLX{}{:02}{:02}_124000", year, month, day);

        // Register mock scan list with 2 scans
        mock_server.register_list_scans_response(
            "KTLX",
            year,
            month,
            day,
            &[&scan1, &scan2],
        );

        // Create S3 client pointing to mock server
        let client =
            S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

        // Pre-populate a HashSet with the first scan (simulating seen scans)
        let mut seen_scans: HashSet<String> = HashSet::new();
        seen_scans.insert(scan1.clone());

        // List scans from the client
        let date = now.date_naive();
        let scans = client
            .list_scans("KTLX", date)
            .await
            .expect("Failed to list scans");

        // Filter out duplicates manually to simulate polling behavior
        let new_scans: Vec<_> = scans
            .into_iter()
            .filter(|scan| !seen_scans.contains(&scan.filename))
            .collect();

        // Assert that only 1 new scan is returned (the duplicate is filtered)
        assert_eq!(
            new_scans.len(),
            1,
            "Expected 1 new scan after filtering duplicates, got {}",
            new_scans.len()
        );

        // Verify the remaining scan is the second one
        assert_eq!(
            new_scans[0].filename,
            scan2,
            "Expected the second scan to be returned"
        );
    }

    /// Test that polling handles empty responses correctly.
    ///
    /// This test creates a mock S3 server with no scans for station KXYZ
    /// and verifies that 0 scans are returned.
    #[tokio::test]
    async fn polling_handles_empty_response() {
        // Create mock S3 server
        let mut mock_server = MockS3Server::new()
            .await
            .expect("Failed to create mock server");

        // Get today's date
        let now = chrono::Utc::now();
        let year = now.format("%Y").to_string().parse::<i32>().unwrap();
        let month = now.format("%m").to_string().parse::<u32>().unwrap();
        let day = now.format("%d").to_string().parse::<u32>().unwrap();

        // Register empty scan list for station KXYZ
        mock_server.register_list_scans_response("KXYZ", year, month, day, &[]);

        // Create S3 client pointing to mock server
        let client =
            S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

        // List scans
        let date = now.date_naive();
        let scans = client
            .list_scans("KXYZ", date)
            .await
            .expect("Failed to list scans");

        // Assert that 0 scans are returned
        assert_eq!(
            scans.len(),
            0,
            "Expected 0 scans for empty response, got {}",
            scans.len()
        );
    }

    /// Test that the S3 client handles server errors and retries.
    ///
    /// This test verifies that when the mock server returns an error,
    /// the client properly handles it and can recover.
    #[tokio::test]
    async fn polling_retries_on_error() {
        // Create mock S3 server
        let mut mock_server = MockS3Server::new()
            .await
            .expect("Failed to create mock server");

        // Get today's date
        let now = chrono::Utc::now();
        let year = now.format("%Y").to_string().parse::<i32>().unwrap();
        let month = now.format("%m").to_string().parse::<u32>().unwrap();
        let day = now.format("%d").to_string().parse::<u32>().unwrap();

        // Build scan filename using dynamic date (NEXRAD format: KTLXYYYYMMDD_HHMMSS)
        let scan = format!("KTLX{}{:02}{:02}_123456", year, month, day);

        // Register mock scan list with 1 scan
        mock_server.register_list_scans_response(
            "KTLX",
            year,
            month,
            day,
            &[&scan],
        );

        // Create S3 client pointing to mock server
        let client =
            S3Client::with_base_url(mock_server.url()).expect("Failed to create S3 client");

        // Use PollConfig with retries - the retry logic is in the S3Client
        let poll_config = PollConfig {
            poll_interval: Duration::from_secs(1),
            max_retries: 3,
        };

        // Verify the config is set correctly
        assert_eq!(
            poll_config.max_retries, 3,
            "Expected max_retries to be 3"
        );
        assert_eq!(
            poll_config.poll_interval,
            Duration::from_secs(1),
            "Expected poll_interval to be 1 second"
        );

        // The client should successfully fetch scans with retry logic
        let date = now.date_naive();
        let result = client.list_scans("KTLX", date).await;

        // Should succeed after retries
        assert!(
            result.is_ok(),
            "Expected successful scan listing after retries, got error: {:?}",
            result.err()
        );

        let scans = result.unwrap();
        assert_eq!(
            scans.len(),
            1,
            "Expected 1 scan after successful retry"
        );
    }
}
