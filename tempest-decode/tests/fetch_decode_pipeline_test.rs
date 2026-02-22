//! Integration tests for S3 fetch → decode pipeline.
//!
//! These tests verify the complete pipeline from raw bytes (simulating S3 fetch)
//! to decoded VolumeScan. The tests simulate what happens when:
//! 1. S3 client fetches NEXRAD binary data from NOAA bucket
//! 2. Decode library parses the binary data into structured VolumeScan
//!
//! This is the primary integration point between tempest-fetch and tempest-decode.

use std::path::Path;
use tempest_decode::{decode, DecodeError};

/// Helper function to load a fixture file (simulating S3 fetch result).
///
/// In production, this would be replaced by:
/// ```ignore
/// let bytes = s3_client.get_object("noaa-nexrad-level2", "KTLX/2024/01/15/KTLX/20240115_123456").await?;
/// ```
fn load_fixture(name: &str) -> Vec<u8> {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);

    std::fs::read(&fixture_path).unwrap_or_else(|_| panic!("Failed to read fixture: {}", name))
}

/// Test 1: Load a fixture file (simulating S3 fetch result), decode it,
/// and verify output has valid station_id, vcp, and sweeps with data.
///
/// This tests the primary pipeline: raw bytes → VolumeScan with valid metadata.
#[test]
fn test_pipeline_load_decode_verify_metadata() {
    // Simulate S3 fetch: load binary NEXRAD data
    // In production: let bytes = s3_client.fetch_scan("KTLX", "2024-01-15").await?;
    let data = load_fixture("vcp215_clear_air.bin");

    // Verify we got data (simulating successful S3 response)
    assert!(!data.is_empty(), "S3 fetch should return non-empty data");

    // Decode the binary data into VolumeScan
    let volume = decode(&data).expect("Failed to decode NEXRAD data");

    // Verify station_id is valid (4-character ICAO code)
    assert!(
        !volume.station_id.is_empty(),
        "Station ID should not be empty"
    );
    assert_eq!(
        volume.station_id.len(),
        4,
        "Station ID should be 4 characters"
    );

    // Verify VCP is valid
    assert!(volume.vcp > 0, "VCP should be a positive number");

    // Verify we have sweeps with data
    assert!(
        !volume.sweeps.is_empty(),
        "Volume scan should have at least one sweep"
    );

    println!("Pipeline test - VCP 215 Clear Air:");
    println!("  Station: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
    println!("  Sweeps: {}", volume.sweeps.len());
    println!("  Timestamp: {}", volume.timestamp);
}

/// Test 2: Test that multiple different fixture files decode successfully.
///
/// This verifies the pipeline works across different VCP types and stations.
#[test]
fn test_pipeline_multiple_fixtures() {
    // Test VCP 215 - Clear Air mode
    let vcp215_data = load_fixture("vcp215_clear_air.bin");
    let vcp215 = decode(&vcp215_data).expect("Failed to decode VCP 215");
    assert_eq!(vcp215.vcp, 215, "VCP 215 fixture should decode to VCP 215");
    assert_eq!(vcp215.station_id, "KTLX", "VCP 215 should be KTLX");

    // Test VCP 35 - Clear Air mode
    let vcp35_data = load_fixture("vcp35_clear_air.bin");
    let vcp35 = decode(&vcp35_data).expect("Failed to decode VCP 35");
    assert_eq!(vcp35.vcp, 35, "VCP 35 fixture should decode to VCP 35");
    assert_eq!(vcp35.station_id, "KOKC", "VCP 35 should be KOKC");

    // Test VCP 12 - Severe Weather mode
    let vcp12_data = load_fixture("vcp12_severe_weather.bin");
    let vcp12 = decode(&vcp12_data).expect("Failed to decode VCP 12");
    assert_eq!(vcp12.vcp, 12, "VCP 12 fixture should decode to VCP 12");
    assert_eq!(vcp12.station_id, "KICT", "VCP 12 should be KICT");

    // Test Super Resolution
    let super_res_data = load_fixture("super_resolution.bin");
    let super_res = decode(&super_res_data).expect("Failed to decode super resolution");
    assert!(
        !super_res.station_id.is_empty(),
        "Super resolution should have valid station"
    );

    // Test Reflectivity Only
    let refl_only_data = load_fixture("reflectivity_only.bin");
    let refl_only = decode(&refl_only_data).expect("Failed to decode reflectivity only");
    assert_eq!(refl_only.vcp, 215, "Reflectivity only should be VCP 215");

    println!("All fixture files decoded successfully!");
    println!("  VCP 215 (KTLX): {} sweeps", vcp215.sweeps.len());
    println!("  VCP 35 (KOKC): {} sweeps", vcp35.sweeps.len());
    println!("  VCP 12 (KICT): {} sweeps", vcp12.sweeps.len());
    println!("  Super Resolution: {} sweeps", super_res.sweeps.len());
    println!("  Reflectivity Only: {} sweeps", refl_only.sweeps.len());
}

/// Test 3: Verify the decoded VolumeScan has meaningful data
/// (sweeps with radials containing gates).
///
/// This tests that the decoded data contains actual radar measurements,
/// not just empty structures.
#[test]
fn test_pipeline_meaningful_data() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    // Verify sweeps have elevation angles
    for (i, sweep) in volume.sweeps.iter().enumerate() {
        assert!(
            sweep.elevation >= 0.0 && sweep.elevation <= 90.0,
            "Sweep {} has invalid elevation: {}",
            i,
            sweep.elevation
        );
    }

    // Verify at least some sweeps have radials
    let total_radials: usize = volume.sweeps.iter().map(|s| s.radials.len()).sum();
    assert!(
        total_radials > 0,
        "Volume scan should have at least one radial"
    );

    // Verify at least some radials have gates (radar measurements)
    let total_gates: usize = volume
        .sweeps
        .iter()
        .flat_map(|s| &s.radials)
        .map(|r| r.gates.len())
        .sum();
    assert!(total_gates > 0, "Volume scan should have at least one gate");

    // Verify gate ranges are reasonable (positive and within radar range)
    for sweep in &volume.sweeps {
        for radial in &sweep.radials {
            for gate in &radial.gates {
                assert!(gate.range > 0.0, "Gate range should be positive");
                // NEXRAD typically has maximum range of ~460km
                assert!(
                    gate.range <= 500_000.0,
                    "Gate range should be within radar range ({}m is too large)",
                    gate.range
                );
            }
        }
    }

    println!("Meaningful data verification:");
    println!("  Total sweeps: {}", volume.sweeps.len());
    println!("  Total radials: {}", total_radials);
    println!("  Total gates: {}", total_gates);
    println!(
        "  Elevation angles: {:?}",
        volume
            .sweeps
            .iter()
            .map(|s| s.elevation)
            .collect::<Vec<_>>()
    );
}

/// Test 4: Test error handling - verify decode returns appropriate error
/// for invalid/malformed input.
///
/// This tests that the pipeline correctly handles error cases:
/// - Empty input
/// - Truncated data
/// - Invalid message types
#[test]
fn test_pipeline_error_handling() {
    // Test 1: Empty input should return InsufficientBytes error
    let empty_result = decode(b"");
    assert!(empty_result.is_err(), "Empty input should return error");
    match empty_result {
        Err(DecodeError::InsufficientBytes { needed, have }) => {
            assert!(needed > 0, "Should require some bytes");
            assert_eq!(have, 0, "Should have zero bytes");
            println!("Empty input correctly returned InsufficientBytes error");
        }
        Err(e) => {
            // Other errors are also acceptable for empty input
            println!("Empty input returned error: {:?}", e);
        }
        Ok(_) => panic!("Empty input should not succeed"),
    }

    // Test 2: Truncated data (too few bytes for valid message)
    let truncated = b"ABC";
    let truncated_result = decode(truncated);
    assert!(
        truncated_result.is_err(),
        "Truncated input should return error"
    );
    println!("Truncated input correctly returned error");

    // Test 3: Invalid message type (not 31)
    // Create a minimal valid header but with wrong message type
    let mut invalid_msg_type = vec![0u8; 20];
    // Message size (bytes 0-3)
    invalid_msg_type[0..4].copy_from_slice(&(16u32).to_be_bytes());
    // Message type = 99 (invalid - should be 31)
    invalid_msg_type[4..6].copy_from_slice(&99u16.to_be_bytes());
    // Station ID
    invalid_msg_type[12..16].copy_from_slice(b"KTLX");

    let invalid_type_result = decode(&invalid_msg_type);
    assert!(
        invalid_type_result.is_err(),
        "Invalid message type should return error"
    );
    match invalid_type_result {
        Err(DecodeError::InvalidMessageType(got)) => {
            assert_eq!(got, 99, "Should report message type 99");
            println!("Invalid message type correctly returned InvalidMessageType(99)");
        }
        Err(e) => {
            println!("Invalid message type returned other error: {:?}", e);
        }
        Ok(_) => panic!("Invalid message type should not succeed"),
    }

    // Test 4: The truncated.bin fixture may also produce an error or incomplete data
    let truncated_fixture = load_fixture("truncated.bin");
    let fixture_result = decode(&truncated_fixture);
    match fixture_result {
        Ok(volume) => {
            println!(
                "Truncated fixture handled gracefully: station={}, sweeps={}",
                volume.station_id,
                volume.sweeps.len()
            );
        }
        Err(e) => {
            println!("Truncated fixture returned error: {:?}", e);
        }
    }

    println!("All error handling tests passed!");
}

/// Test 5: Simulate full S3 fetch → decode workflow.
///
/// This test simulates the complete workflow that would happen in production:
/// 1. Fetch data from S3 (simulated by loading fixture)
/// 2. Validate the data is not empty
/// 3. Decode into VolumeScan
/// 4. Use the decoded data for further processing
#[test]
fn test_simulated_s3_fetch_decode_workflow() {
    // Step 1: Simulate S3 fetch - in production this would be:
    // let s3_client = S3Client::new();
    // let bytes = s3_client.fetch_scan("KTLX", "2024-01-15T12:00:00Z").await?;
    let bytes = load_fixture("high_altitude_station.bin");

    // Step 2: Validate S3 response
    assert!(!bytes.is_empty(), "S3 should return non-empty data");
    assert!(
        bytes.len() > 100,
        "NEXRAD data should be at least 100 bytes"
    );

    // Step 3: Decode the binary data
    let volume = decode(&bytes).expect("Should decode NEXRAD data");

    // Step 4: Use the decoded data (e.g., for rendering)
    assert!(!volume.station_id.is_empty());

    // In production, this would be passed to tempest-render-core for visualization
    let station = &volume.station_id;
    let vcp = volume.vcp;
    let sweep_count = volume.sweeps.len();

    println!("Simulated workflow complete:");
    println!("  Fetched {} bytes from S3", bytes.len());
    println!("  Decoded station: {}", station);
    println!("  VCP: {}", vcp);
    println!("  Sweeps: {}", sweep_count);

    // Verify the data is ready for rendering
    for sweep in &volume.sweeps {
        for radial in &sweep.radials {
            // In production, this would map to pixels for visualization
            let _azimuth = radial.azimuth;
            let _gates = radial.gates.len();
        }
    }

    // This data would now be passed to tempest-render-core for projection
    println!("  Data ready for rendering/projection");
}
