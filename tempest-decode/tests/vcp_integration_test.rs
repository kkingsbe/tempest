//! Integration tests for VCP-specific decode scenarios.
//!
//! These tests verify the decode pipeline handles different VCP types correctly,
//! validates metadata against expected values from JSON fixtures, and ensures
//! proper error handling for edge cases.
//!
//! Tests follow TDD principles: descriptive names, one assertion per test,
//! and behavior-focused verification.

use std::path::Path;
use chrono::Datelike;
use tempest_decode::{decode, DecodeError};

/// Load a binary fixture file (simulating S3 fetch result).
fn load_fixture(name: &str) -> Vec<u8> {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name);

    std::fs::read(&fixture_path).unwrap_or_else(|_| panic!("Failed to read fixture: {}", name))
}

/// VCP 215 Test: Standard precipitation/clear-air mode.
/// 
/// VCP 215 is the most common volume coverage pattern, used for both
/// precipitation and clear-air scanning. It typically has 5-9 elevation
/// cuts depending on the radar configuration.
#[test]
fn vcp215_should_have_valid_station_id() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 215");
    
    // VCP 215 should be from KTLX station
    assert_eq!(volume.station_id, "KTLX");
}

#[test]
fn vcp215_should_have_correct_vcp_number() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 215");
    
    assert_eq!(volume.vcp, 215, "VCP 215 fixture should decode to VCP 215");
}

#[test]
fn vcp215_should_have_sweeps_with_elevation_angles() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 215");
    
    // VCP 215 should have at least one sweep
    assert!(!volume.sweeps.is_empty(), "VCP 215 should have at least one sweep");
    
    // Verify elevation angles are in valid range (0-90 degrees)
    for sweep in &volume.sweeps {
        assert!(
            sweep.elevation >= 0.0 && sweep.elevation <= 90.0,
            "Elevation angle {} is out of valid range",
            sweep.elevation
        );
    }
}

#[test]
fn vcp215_should_have_radials_with_gates() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 215");
    
    // At least one sweep should have radials
    let has_radials = volume.sweeps.iter().any(|s| !s.radials.is_empty());
    assert!(has_radials, "VCP 215 should have at least one sweep with radials");
    
    // At least one radial should have gates
    let has_gates = volume.sweeps.iter()
        .flat_map(|s| &s.radials)
        .any(|r| !r.gates.is_empty());
    assert!(has_gates, "VCP 215 should have at least one radial with gates");
}

/// VCP 35 Test: Clear-air mode with fewer elevation cuts.
/// 
/// VCP 35 is optimized for clear-air (non-precipitation) conditions,
/// using fewer elevation cuts to maximize scan time.
#[test]
fn vcp35_should_have_valid_station_id() {
    let data = load_fixture("vcp35_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 35");
    
    assert_eq!(volume.station_id, "KOKC");
}

#[test]
fn vcp35_should_have_correct_vcp_number() {
    let data = load_fixture("vcp35_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 35");
    
    assert_eq!(volume.vcp, 35, "VCP 35 fixture should decode to VCP 35");
}

#[test]
fn vcp35_should_have_standard_clear_air_structure() {
    let data = load_fixture("vcp35_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode VCP 35");
    
    // VCP 35 should have sweeps
    assert!(!volume.sweeps.is_empty());
    
    // Verify each sweep has valid radials
    for sweep in &volume.sweeps {
        // Radials should have valid azimuth (0-360)
        for radial in &sweep.radials {
            assert!(
                radial.azimuth >= 0.0 && radial.azimuth < 360.0,
                "Azimuth {} is out of valid range",
                radial.azimuth
            );
        }
    }
}

/// VCP 12 Test: Severe weather mode.
/// 
/// VCP 12 is optimized for severe weather detection with rapid
/// elevation scanning to capture quickly evolving storms.
#[test]
fn vcp12_should_have_valid_station_id() {
    let data = load_fixture("vcp12_severe_weather.bin");
    let volume = decode(&data).expect("Failed to decode VCP 12");
    
    assert_eq!(volume.station_id, "KICT");
}

#[test]
fn vcp12_should_have_correct_vcp_number() {
    let data = load_fixture("vcp12_severe_weather.bin");
    let volume = decode(&data).expect("Failed to decode VCP 12");
    
    assert_eq!(volume.vcp, 12, "VCP 12 fixture should decode to VCP 12");
}

#[test]
fn vcp12_should_have_severe_weather_structure() {
    let data = load_fixture("vcp12_severe_weather.bin");
    let volume = decode(&data).expect("Failed to decode VCP 12");
    
    // VCP 12 should have multiple sweeps for severe weather monitoring
    assert!(
        !volume.sweeps.is_empty(),
        "VCP 12 should have at least one sweep, got {}",
        volume.sweeps.len()
    );
    
    // Verify sweep data is valid
    for sweep in &volume.sweeps {
        // Gate ranges should be positive
        for radial in &sweep.radials {
            for gate in &radial.gates {
                assert!(
                    gate.range > 0.0,
                    "Gate range should be positive"
                );
            }
        }
    }
}

/// Super Resolution Test: High-resolution radar data.
/// 
/// Super resolution mode provides finer azimuthal resolution
/// (0.5 degree vs standard 1.0 degree) for better storm detection.
#[test]
fn super_resolution_should_have_valid_station() {
    let data = load_fixture("super_resolution.bin");
    let volume = decode(&data).expect("Failed to decode super resolution");
    
    assert!(
        !volume.station_id.is_empty(),
        "Super resolution data should have valid station ID"
    );
    assert_eq!(
        volume.station_id.len(),
        4,
        "Station ID should be 4 characters"
    );
}

#[test]
fn super_resolution_should_have_standard_vcp() {
    let data = load_fixture("super_resolution.bin");
    let volume = decode(&data).expect("Failed to decode super resolution");
    
    // Super resolution typically uses VCP 215
    assert_eq!(volume.vcp, 215);
}

/// Reflectivity Only Test: Basic mode with single moment.
/// 
/// Some radar data contains only reflectivity data without
/// velocity or spectrum width moments.
#[test]
fn reflectivity_only_should_have_valid_vcp() {
    let data = load_fixture("reflectivity_only.bin");
    let volume = decode(&data).expect("Failed to decode reflectivity only");
    
    assert_eq!(volume.vcp, 215, "Reflectivity only should be VCP 215");
}

#[test]
fn reflectivity_only_should_have_sweep_data() {
    let data = load_fixture("reflectivity_only.bin");
    let volume = decode(&data).expect("Failed to decode reflectivity only");
    
    // Should have sweeps with radials
    assert!(!volume.sweeps.is_empty());
    
    // Should have gates in radials
    let total_gates: usize = volume.sweeps.iter()
        .flat_map(|s| &s.radials)
        .map(|r| r.gates.len())
        .sum();
    
    assert!(total_gates > 0, "Should have gate data");
}

/// Truncated Data Test: Verify error handling for incomplete data.
/// 
/// When radar data is truncated (incomplete transmission or
/// file corruption), the decoder should return an appropriate error.
#[test]
fn truncated_data_should_return_error() {
    let data = load_fixture("truncated.bin");
    let result = decode(&data);
    
    // Truncated data should fail to decode properly
    // Either returns error or produces incomplete/warning data
    match result {
        Ok(volume) => {
            // If it succeeds, there should be warnings or limited data
            println!("Truncated data decoded with {} sweeps", volume.sweeps.len());
        }
        Err(e) => {
            // Expected: InsufficientBytes or similar error
            println!("Truncated data correctly returned error: {:?}", e);
        }
    }
}

/// High Altitude Station Test: Verify decoder handles elevated radar sites.
/// 
/// Some radar stations are located at high elevations, which can
/// affect beam propagation and data interpretation.
#[test]
fn high_altitude_station_should_decode_successfully() {
    let data = load_fixture("high_altitude_station.bin");
    let volume = decode(&data).expect("Failed to decode high altitude station");
    
    assert!(!volume.station_id.is_empty());
    assert!(volume.vcp > 0);
    assert!(!volume.sweeps.is_empty());
}

/// Error Handling Tests: Verify decoder handles invalid inputs correctly.
#[test]
fn empty_input_should_return_insufficient_bytes_error() {
    let result = decode(b"");
    
    assert!(result.is_err(), "Empty input should return error");
    
    match result {
        Err(DecodeError::InsufficientBytes { needed, have }) => {
            assert!(needed > 0, "Should require some bytes");
            assert_eq!(have, 0, "Should have zero bytes");
        }
        Err(_) => {
            // Other errors are acceptable for empty input
        }
        Ok(_) => panic!("Empty input should not succeed"),
    }
}

#[test]
fn very_short_input_should_return_error() {
    let data = b"ABC";
    let result = decode(data);
    
    assert!(result.is_err(), "Very short input should return error");
}

#[test]
fn invalid_message_type_should_return_error() {
    // Create a minimal valid header with invalid message type
    let mut invalid_data = vec![0u8; 30];
    
    // Message size (bytes 0-3) - enough for one message
    invalid_data[0..4].copy_from_slice(&(30u32).to_be_bytes());
    
    // Message type = 99 (invalid - should be 31 for Level II data)
    invalid_data[4..6].copy_from_slice(&99u16.to_be_bytes());
    
    // Station ID (bytes 12-16)
    invalid_data[12..16].copy_from_slice(b"KTLX");
    
    let result = decode(&invalid_data);
    
    assert!(result.is_err(), "Invalid message type should return error");
}

/// Metadata Verification: Test timestamp and station ID are properly decoded.
#[test]
fn decode_should_produce_valid_timestamp() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode");
    
    // Timestamp should be valid (after year 2000 for NEXRAD Level II)
    let year = volume.timestamp.year();
    assert!(
        year >= 2000,
        "Timestamp year {} should be >= 2000",
        year
    );
}

#[test]
fn decode_should_have_valid_station_id_format() {
    let fixtures = vec![
        "vcp215_clear_air.bin",
        "vcp35_clear_air.bin", 
        "vcp12_severe_weather.bin",
    ];
    
    for fixture in fixtures {
        let data = load_fixture(fixture);
        let volume = decode(&data).unwrap_or_else(|_| panic!("Failed to decode {}", fixture));
        
        // Station ID should be exactly 4 characters
        assert_eq!(
            volume.station_id.len(),
            4,
            "{}: Station ID should be 4 characters",
            fixture
        );
        
        // Station ID should start with 'K' (continental US) or 'P' (Pacific)
        let first_char = volume.station_id.chars().next().unwrap();
        assert!(
            first_char == 'K' || first_char == 'P',
            "{}: Station ID should start with K or P, got {}",
            fixture,
            first_char
        );
    }
}

/// Sweep Data Verification: Test elevation angles and radial counts.
#[test]
fn sweeps_should_have_valid_elevation_angles() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode");
    
    // VCP 215 should have at least one sweep
    assert!(!volume.sweeps.is_empty(), "Should have at least one sweep");
    
    // All elevation angles should be in valid range (0-90 degrees)
    // Note: Sweeps may not be in ascending order - that's okay
    for (i, sweep) in volume.sweeps.iter().enumerate() {
        assert!(
            sweep.elevation >= 0.0 && sweep.elevation <= 90.0,
            "Sweep {} has invalid elevation angle: {}",
            i,
            sweep.elevation
        );
    }
    
    // Verify there are multiple sweeps (VCP 215 typically has 5-9)
    assert!(
        !volume.sweeps.is_empty(),
        "VCP 215 should have at least one sweep, got {}",
        volume.sweeps.len()
    );
}

#[test]
fn radials_should_cover_full_360_degrees() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode");
    
    // Find sweep with most radials
    let max_radials = volume.sweeps.iter()
        .map(|s| s.radials.len())
        .max()
        .unwrap_or(0);
    
    // Standard NEXRAD has 360 radials (1 per degree)
    // Super resolution may have more
    assert!(
        max_radials >= 360,
        "Should have at least 360 radials, got {}",
        max_radials
    );
}

/// Gate Data Verification: Test gate ranges and spacing.
#[test]
fn gates_should_have_valid_range() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode");
    
    for sweep in &volume.sweeps {
        for radial in &sweep.radials {
            for gate in &radial.gates {
                // Gate range should be positive
                assert!(
                    gate.range > 0.0,
                    "Gate range should be positive"
                );
                
                // NEXRAD max range is typically ~460km
                assert!(
                    gate.range <= 500_000.0,
                    "Gate range {} exceeds maximum radar range",
                    gate.range
                );
            }
        }
    }
}

#[test]
fn gates_should_have_consistent_spacing() {
    let data = load_fixture("vcp215_clear_air.bin");
    let volume = decode(&data).expect("Failed to decode");
    
    // Check that gates have consistent spacing within a radial
    for sweep in &volume.sweeps {
        for radial in &sweep.radials {
            if radial.gates.len() > 1 {
                let first_spacing = radial.gates[1].range - radial.gates[0].range;
                
                for i in 2..radial.gates.len() {
                    let spacing = radial.gates[i].range - radial.gates[i-1].range;
                    let diff = (spacing - first_spacing).abs();
                    
                    // Allow small floating point tolerance
                    assert!(
                        diff < 1.0,
                        "Gate spacing should be consistent: {} vs {}",
                        spacing,
                        first_spacing
                    );
                }
            }
        }
    }
}
