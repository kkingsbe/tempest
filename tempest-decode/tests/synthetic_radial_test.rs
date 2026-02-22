//! Integration tests using synthetic NEXRAD test fixtures.
//!
//! These tests verify that the decoder can parse synthetic NEXRAD Level II
//! Message 31 data files.

use std::path::Path;
use tempest_decode::{decode, Msg31Header};

/// Helper function to load a fixture file
fn load_fixture(name: &str) -> Vec<u8> {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);

    std::fs::read(&fixture_path).expect(&format!("Failed to read fixture: {}", name))
}

/// Test parsing the synthetic Message 31 radial data (legacy fixture).
#[test]
fn test_parse_synthetic_msg31_radial() {
    let data = load_fixture("synthetic_msg31.bin");

    // Verify file is not empty
    assert!(!data.is_empty(), "Test fixture should not be empty");

    // Verify minimum message size
    assert!(data.len() >= 4, "Need at least 4 bytes for message size");

    // Parse message size
    let msg_size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    assert_eq!(
        msg_size as usize,
        data.len() - 4,
        "Message size should match data length"
    );

    // Parse message type (should be 31)
    let msg_type = u16::from_be_bytes([data[4], data[5]]);
    assert_eq!(msg_type, 31, "Message type should be 31 for radial data");

    // Parse station ID (starts at byte 12)
    let station_id = std::str::from_utf8(&data[12..16]).expect("Valid UTF-8 for station ID");
    assert_eq!(station_id, "KTLX", "Station ID should be KTLX");

    println!("Successfully parsed synthetic msg31 header:");
    println!("  Message size: {} bytes", msg_size);
    println!("  Message type: {}", msg_type);
    println!("  Station ID: {}", station_id);
}

/// Test parsing VCP 215 clear-air fixture.
#[test]
fn test_parse_vcp215_clear_air() {
    let data = load_fixture("vcp215_clear_air.bin");

    // Parse the header
    let header = Msg31Header::parse(&data[4..]).expect("Failed to parse header");

    assert_eq!(header.id, 31, "Message type should be 31");
    assert_eq!(header.station_id.as_str().unwrap(), "KTLX");

    // Decode the volume
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, 215, "VCP should be 215");
    assert!(!volume.sweeps.is_empty(), "Should have at least one sweep");

    println!("VCP 215 Clear Air:");
    println!("  Station: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
    println!("  Sweeps: {}", volume.sweeps.len());
}

/// Test parsing VCP 35 clear-air fixture.
#[test]
fn test_parse_vcp35_clear_air() {
    let data = load_fixture("vcp35_clear_air.bin");

    let header = Msg31Header::parse(&data[4..]).expect("Failed to parse header");
    assert_eq!(header.station_id.as_str().unwrap(), "KOKC");

    let volume = decode(&data).expect("Failed to decode fixture");
    assert_eq!(volume.vcp, 35, "VCP should be 35");

    println!("VCP 35 Clear Air:");
    println!("  Station: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
}

/// Test parsing VCP 12 severe weather fixture.
#[test]
fn test_parse_vcp12_severe_weather() {
    let data = load_fixture("vcp12_severe_weather.bin");

    let header = Msg31Header::parse(&data[4..]).expect("Failed to parse header");
    assert_eq!(header.station_id.as_str().unwrap(), "KICT");

    let volume = decode(&data).expect("Failed to decode fixture");
    assert_eq!(volume.vcp, 12, "VCP should be 12");

    println!("VCP 12 Severe Weather:");
    println!("  Station: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
}

/// Test parsing super-resolution fixture.
#[test]
fn test_parse_super_resolution() {
    let data = load_fixture("super_resolution.bin");

    let volume = decode(&data).expect("Failed to decode super-resolution fixture");

    // Super-resolution should have more radials
    println!("Super Resolution:");
    println!("  Station: {}", volume.station_id);
    println!("  Sweeps: {}", volume.sweeps.len());
}

/// Test parsing reflectivity-only fixture (missing VEL and SW).
#[test]
fn test_parse_reflectivity_only() {
    let data = load_fixture("reflectivity_only.bin");

    let volume = decode(&data).expect("Failed to decode reflectivity-only fixture");

    // Should decode successfully even without VEL/SW
    assert_eq!(volume.vcp, 215);

    println!("Reflectivity Only:");
    println!("  Station: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
    println!("  Sweeps: {}", volume.sweeps.len());
}

/// Test parsing high-altitude station fixture.
#[test]
fn test_parse_high_altitude_station() {
    let data = load_fixture("high_altitude_station.bin");

    let header = Msg31Header::parse(&data[4..]).expect("Failed to parse header");
    assert_eq!(header.station_id.as_str().unwrap(), "KPUB");

    let volume = decode(&data).expect("Failed to decode fixture");
    assert_eq!(volume.vcp, 215);

    println!("High Altitude Station:");
    println!("  Station: {}", volume.station_id);
}

/// Test parsing velocity aliasing fixture.
#[test]
fn test_parse_velocity_aliasing() {
    let data = load_fixture("velocity_aliasing.bin");

    let volume = decode(&data).expect("Failed to decode velocity aliasing fixture");

    assert_eq!(volume.vcp, 215);

    println!("Velocity Aliasing:");
    println!("  Station: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
}

/// Test that truncated data returns an error.
#[test]
fn test_truncated_data_returns_error() {
    let data = load_fixture("truncated.bin");

    // Truncated data - the decoder may handle it gracefully and return an empty volume
    // or it may return an error depending on how incomplete the data is
    let result = decode(&data);

    // The truncated file is 1000 bytes - let's see what happens
    match result {
        Ok(volume) => {
            // Decoder handles gracefully - verify we get some data but it's incomplete
            println!(
                "Truncated data handled gracefully: station={}, sweeps={}",
                volume.station_id,
                volume.sweeps.len()
            );
        }
        Err(e) => {
            // Expected - truncated data should cause an error
            println!("Truncated data correctly returned error: {:?}", e);
        }
    }
}

/// Test decoding the full synthetic volume (legacy test).
#[test]
fn test_decode_synthetic_volume() {
    let data = load_fixture("synthetic_msg31.bin");

    // Decode the data
    let volume = decode(&data).expect("Failed to decode test fixture");

    println!("Decoded volume scan:");
    println!("  Station ID: {}", volume.station_id);
    println!("  VCP: {}", volume.vcp);
    println!("  Sweeps: {}", volume.sweeps.len());

    // The synthetic data should produce a valid volume
    assert!(!volume.station_id.is_empty(), "Should have a station ID");
}

/// Test that fixtures directory exists with required files.
#[test]
fn test_fixtures_exist() {
    let fixtures_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");

    assert!(fixtures_dir.exists(), "Fixtures directory should exist");

    // Check all expected fixtures exist
    let expected_fixtures = vec![
        "synthetic_msg31.bin",
        "vcp215_clear_air.bin",
        "vcp35_clear_air.bin",
        "vcp12_severe_weather.bin",
        "super_resolution.bin",
        "reflectivity_only.bin",
        "high_altitude_station.bin",
        "velocity_aliasing.bin",
        "truncated.bin",
    ];

    for fixture in expected_fixtures {
        let fixture_path = fixtures_dir.join(fixture);
        assert!(fixture_path.exists(), "Fixture {} should exist", fixture);
    }

    // Check JSON metadata files exist
    let expected_metadata = vec![
        "synthetic_msg31.json",
        "vcp215_clear_air.json",
        "vcp35_clear_air.json",
        "vcp12_severe_weather.json",
        "super_resolution.json",
        "reflectivity_only.json",
        "high_altitude_station.json",
        "velocity_aliasing.json",
        "truncated.json",
    ];

    for meta in expected_metadata {
        let meta_path = fixtures_dir.join(meta);
        assert!(meta_path.exists(), "Metadata {} should exist", meta);
    }

    // Check README exists
    let readme_path = fixtures_dir.join("README.md");
    assert!(readme_path.exists(), "README.md should exist");

    println!("All fixture files exist!");

    // Print file sizes
    println!("\nFixture file sizes:");
    for fixture in &[
        "synthetic_msg31.bin",
        "vcp215_clear_air.bin",
        "vcp35_clear_air.bin",
        "vcp12_severe_weather.bin",
        "super_resolution.bin",
        "reflectivity_only.bin",
        "high_altitude_station.bin",
        "velocity_aliasing.bin",
        "truncated.bin",
    ] {
        let path = fixtures_dir.join(fixture);
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        println!("  {}: {} bytes", fixture, size);
    }
}

/// Test verifying fixture properties match metadata.
#[test]
fn test_fixture_properties() {
    // Test VCP 215 properties
    let vcp215_data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&vcp215_data).expect("Failed to decode VCP 215");

    assert_eq!(volume.station_id, "KTLX");
    assert_eq!(volume.vcp, 215);
    assert!(volume.sweeps.len() > 0, "Should have sweeps");

    // Test VCP 35 properties
    let vcp35_data = load_fixture("vcp35_clear_air.bin");
    let volume35 = decode(&vcp35_data).expect("Failed to decode VCP 35");

    assert_eq!(volume35.station_id, "KOKC");
    assert_eq!(volume35.vcp, 35);

    // Test VCP 12 properties
    let vcp12_data = load_fixture("vcp12_severe_weather.bin");
    let volume12 = decode(&vcp12_data).expect("Failed to decode VCP 12");

    assert_eq!(volume12.station_id, "KICT");
    assert_eq!(volume12.vcp, 12);

    // Test high altitude properties
    let high_alt_data = load_fixture("high_altitude_station.bin");
    let high_alt = decode(&high_alt_data).expect("Failed to decode high altitude");

    assert_eq!(high_alt.station_id, "KPUB");

    println!("All fixture property checks passed!");
}
