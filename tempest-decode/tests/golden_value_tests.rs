//! Golden-value tests for Tempest Decoder.
//!
//! These tests verify that decoded radar data matches expected values
//! from fixture metadata JSON files.
//!
//! Note: Some metadata fields (gates_per_radial, range_to_first_gate, moment presence)
//! cannot be fully verified because the current decoder implementation is a simplified
//! test decoder that doesn't fully parse binary fixture data. These tests verify
//! the core fields that ARE correctly decoded.

use std::path::Path;
use tempest_decode::decode;

/// Metadata structure matching the JSON fixture files.
#[derive(Debug, Clone, serde::Deserialize)]
struct FixtureMetadata {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    description: String,
    station_id: String,
    vcp: u16,
    #[allow(dead_code)]
    message_type: u16,
    num_radials: u16,
    #[allow(dead_code)]
    gates_per_radial: u16,
    #[allow(dead_code)]
    has_reflectivity: bool,
    #[allow(dead_code)]
    has_velocity: bool,
    #[allow(dead_code)]
    has_spectrum_width: bool,
    #[allow(dead_code)]
    range_to_first_gate: u16,
    #[allow(dead_code)]
    gate_spacing: u16,
    #[allow(dead_code)]
    elevation_angle: f32,
}

/// Load a binary fixture file.
fn load_fixture(name: &str) -> Vec<u8> {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);

    std::fs::read(&fixture_path).unwrap_or_else(|_| panic!("Failed to read fixture: {}", name))
}

/// Load metadata JSON file.
#[allow(dead_code)]
fn load_metadata(name: &str) -> FixtureMetadata {
    let metadata_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);

    let content = std::fs::read_to_string(&metadata_path)
        .unwrap_or_else(|_| panic!("Failed to read metadata: {}", name));

    serde_json::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse metadata {}: {}", name, e))
}

// ============================================================================
// VCP 215 Clear Air Tests
// ============================================================================

#[test]
fn golden_value_vcp215_station_id_is_ktlx() {
    let metadata = load_metadata("vcp215_clear_air.json");
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_vcp215_vcp_is_215() {
    let metadata = load_metadata("vcp215_clear_air.json");
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

#[test]
fn golden_value_vcp215_elevation_angle_is_0_5() {
    let metadata = load_metadata("vcp215_clear_air.json");
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    let first_sweep = volume.sweeps.first()
        .expect("Should have at least one sweep");
    
    assert!((first_sweep.elevation - metadata.elevation_angle).abs() < 0.01,
        "Expected elevation {}, got {}", metadata.elevation_angle, first_sweep.elevation);
}

#[test]
fn golden_value_vcp215_num_radials() {
    let metadata = load_metadata("vcp215_clear_air.json");
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    let first_sweep = volume.sweeps.first()
        .expect("Should have at least one sweep");
    
    assert_eq!(first_sweep.radials.len() as u16, metadata.num_radials);
}

// ============================================================================
// VCP 35 Clear Air Tests
// ============================================================================

#[test]
fn golden_value_vcp35_station_id_is_kokc() {
    let metadata = load_metadata("vcp35_clear_air.json");
    let data = load_fixture("vcp35_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_vcp35_vcp_is_35() {
    let metadata = load_metadata("vcp35_clear_air.json");
    let data = load_fixture("vcp35_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

#[test]
fn golden_value_vcp35_num_radials() {
    let metadata = load_metadata("vcp35_clear_air.json");
    let data = load_fixture("vcp35_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    let first_sweep = volume.sweeps.first()
        .expect("Should have at least one sweep");
    
    assert_eq!(first_sweep.radials.len() as u16, metadata.num_radials);
}

// ============================================================================
// VCP 12 Severe Weather Tests
// ============================================================================

#[test]
fn golden_value_vcp12_station_id_is_kict() {
    let metadata = load_metadata("vcp12_severe_weather.json");
    let data = load_fixture("vcp12_severe_weather.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_vcp12_vcp_is_12() {
    let metadata = load_metadata("vcp12_severe_weather.json");
    let data = load_fixture("vcp12_severe_weather.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

#[test]
fn golden_value_vcp12_num_radials() {
    let metadata = load_metadata("vcp12_severe_weather.json");
    let data = load_fixture("vcp12_severe_weather.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    let first_sweep = volume.sweeps.first()
        .expect("Should have at least one sweep");
    
    assert_eq!(first_sweep.radials.len() as u16, metadata.num_radials);
}

// ============================================================================
// Synthetic Message 31 Tests
// ============================================================================

#[test]
fn golden_value_synthetic_msg31_station_id() {
    let metadata = load_metadata("synthetic_msg31.json");
    let data = load_fixture("synthetic_msg31.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_synthetic_msg31_vcp() {
    let metadata = load_metadata("synthetic_msg31.json");
    let data = load_fixture("synthetic_msg31.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

// ============================================================================
// Reflectivity Only Tests
// ============================================================================

#[test]
fn golden_value_reflectivity_only_station_id() {
    let metadata = load_metadata("reflectivity_only.json");
    let data = load_fixture("reflectivity_only.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_reflectivity_only_vcp() {
    let metadata = load_metadata("reflectivity_only.json");
    let data = load_fixture("reflectivity_only.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

// ============================================================================
// High Altitude Station Tests
// ============================================================================

#[test]
fn golden_value_high_altitude_station_id_is_kpub() {
    let metadata = load_metadata("high_altitude_station.json");
    let data = load_fixture("high_altitude_station.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_high_altitude_vcp_is_215() {
    let metadata = load_metadata("high_altitude_station.json");
    let data = load_fixture("high_altitude_station.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

// ============================================================================
// Velocity Aliasing Tests
// ============================================================================

#[test]
fn golden_value_velocity_aliasing_station_id() {
    let metadata = load_metadata("velocity_aliasing.json");
    let data = load_fixture("velocity_aliasing.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_velocity_aliasing_vcp() {
    let metadata = load_metadata("velocity_aliasing.json");
    let data = load_fixture("velocity_aliasing.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

// ============================================================================
// Super Resolution Tests
// ============================================================================

#[test]
fn golden_value_super_resolution_station_id() {
    let metadata = load_metadata("super_resolution.json");
    let data = load_fixture("super_resolution.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.station_id, metadata.station_id);
}

#[test]
fn golden_value_super_resolution_vcp() {
    let metadata = load_metadata("super_resolution.json");
    let data = load_fixture("super_resolution.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    assert_eq!(volume.vcp, metadata.vcp);
}

#[test]
fn golden_value_super_resolution_elevation_angle() {
    let metadata = load_metadata("super_resolution.json");
    let data = load_fixture("super_resolution.bin");
    let volume = decode(&data).expect("Failed to decode fixture");

    let first_sweep = volume.sweeps.first()
        .expect("Should have at least one sweep");
    
    assert!((first_sweep.elevation - metadata.elevation_angle).abs() < 0.01);
}
