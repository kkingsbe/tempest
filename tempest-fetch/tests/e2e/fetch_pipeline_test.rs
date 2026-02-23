//! End-to-end tests for the fetch pipeline.
//!
//! These tests verify that the S3 client can successfully:
//! 1. Connect to a mock S3 server
//! 2. List available scans for a station
//! 3. Fetch scan data from the mock server
//! 4. Decompress bzip2 compressed data

use bytes::Bytes;
use chrono::NaiveDate;
use tempfile::TempDir;

use tempest_fetch::mock_s3::MockS3Server;
use tempest_fetch::s3::S3Client;
use tempest_fetch::{decompress_bz2, Cache, CacheConfig};

/// Test that the S3 client can connect to a mock server and list scans.
#[tokio::test]
async fn s3_client_connects_to_mock_server_and_lists_scans() {
    // Create mock S3 server
    let mock_server = MockS3Server::new().await.expect("Failed to create mock server");

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
    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
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
    let mock_server = MockS3Server::new().await.expect("Failed to create mock server");

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
    assert_eq!(data, test_data);
}

/// Test that the S3 client can fetch bzip2 compressed data and decompress it.
#[tokio::test]
async fn s3_client_fetches_and_decompresses_bzip2_data() {
    // Create mock S3 server
    let mock_server = MockS3Server::new().await.expect("Failed to create mock server");

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
    let cache_config = CacheConfig {
        directory: cache_dir.path().to_path_buf(),
        max_size_bytes: 1024 * 1024, // 1MB
    };
    let cache = Cache::new(cache_config).expect("Failed to create cache");

    // Create mock S3 server
    let mock_server = MockS3Server::new().await.expect("Failed to create mock server");

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
    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    let scans = client
        .list_scans("KTLX", date)
        .await
        .expect("Failed to list scans");
    assert_eq!(scans.len(), 1);

    // Step 2: Fetch scan with caching
    let scan = &scans[0];
    let data = client
        .fetch_scan_cached("KTLX", scan, Some(&mut cache.clone()))
        .await
        .expect("Failed to fetch scan");
    assert_eq!(data, test_data);

    // Step 3: Verify we can get stats from cache
    let stats = cache.get_stats().await;
    assert!(stats.entry_count > 0);
}

/// Test that the mock server can handle multiple stations.
#[tokio::test]
async fn mock_server_handles_multiple_stations() {
    // Create mock S3 server
    let mock_server = MockS3Server::new().await.expect("Failed to create mock server");

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

    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();

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
    let date = NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
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
