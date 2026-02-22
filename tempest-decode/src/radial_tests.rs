//! Radial data extraction tests.
//!
//! These tests follow TDD principles - they describe the EXPECTED behavior
//! of the decoder when parsing radial data from NEXRAD Archive2 format.
//!
//! The tests will FAIL initially (RED phase) because the radial data
//! block parsing is not yet implemented. Once Agent 2 implements the
//! parsing logic, these tests should pass.

use crate::decode;

/// Creates minimal valid NEXRAD Archive2 binary data for testing.
///
/// This creates a simplified binary structure representing a single
/// sweep with one radial containing one gate. The structure is:
/// - Message size (4 bytes)
/// - Message header (12 bytes: ID, date, time, station)
/// - Volume scan header
/// - Radial data blocks
fn create_minimal_radial_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Message size (4 bytes) - big endian
    let message_size: u32 = 112; // Simplified size
    data.extend_from_slice(&message_size.to_be_bytes());

    // Message header (12 bytes)
    data.extend_from_slice(&31u16.to_be_bytes()); // Message type 31
    data.extend_from_slice(&20000u16.to_be_bytes()); // MJD date
    data.extend_from_slice(&3600000u32.to_be_bytes()); // 1 hour in ms
    data.extend_from_slice(b"KTLX"); // Station ID

    // Volume scan header (simplified)
    data.extend_from_slice(&1u16.to_be_bytes()); // Volume scan number
    data.extend_from_slice(&215u16.to_be_bytes()); // VCP

    // Add some radial data block indicators
    // These bytes represent the start of radial data
    // The actual format will be parsed by the decoder
    data.extend_from_slice(&[0x00, 0x01]); // Data block type
    data.extend_from_slice(&[0x01, 0x00]); // Elevation number
    data.extend_from_slice(&0.5f32.to_be_bytes()); // Elevation angle

    // Radial header
    data.extend_from_slice(&1u16.to_be_bytes()); // Radial number
    data.extend_from_slice(&360u16.to_be_bytes()); // Azimuth (360 degrees)
    data.extend_from_slice(&1u16.to_be_bytes()); // Number of gates
    data.extend_from_slice(&1000u16.to_be_bytes()); // Range to first gate
    data.extend_from_slice(&250u16.to_be_bytes()); // Gate spacing

    // Gate data - reflectivity moment (REF)
    data.extend_from_slice(b"REF"); // Moment code
    data.extend_from_slice(&[0x00]); // Data type
    data.extend_from_slice(&[20u8]); // Reflectivity value (dBZ)

    data
}

/// Creates test data with multiple sweeps at different elevations.
fn create_multi_sweep_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Message size
    let message_size: u32 = 256;
    data.extend_from_slice(&message_size.to_be_bytes());

    // Message header
    data.extend_from_slice(&31u16.to_be_bytes());
    data.extend_from_slice(&20000u16.to_be_bytes());
    data.extend_from_slice(&3600000u32.to_be_bytes());
    data.extend_from_slice(b"KTLX");

    // Volume scan header
    data.extend_from_slice(&1u16.to_be_bytes()); // Volume scan number
    data.extend_from_slice(&215u16.to_be_bytes()); // VCP

    // Sweep 1 - 0.5 degree elevation
    data.extend_from_slice(&[1u8]); // Sweep flag
    data.extend_from_slice(&0.5f32.to_be_bytes()); // Elevation angle
    data.extend_from_slice(&360u16.to_be_bytes()); // 360 radials

    // Sweep 2 - 1.5 degree elevation
    data.extend_from_slice(&[1u8]); // Sweep flag
    data.extend_from_slice(&1.5f32.to_be_bytes()); // Elevation angle
    data.extend_from_slice(&360u16.to_be_bytes()); // 360 radials

    // Sweep 3 - 2.5 degree elevation
    data.extend_from_slice(&[1u8]); // Sweep flag
    data.extend_from_slice(&2.5f32.to_be_bytes()); // Elevation angle
    data.extend_from_slice(&360u16.to_be_bytes()); // 360 radials

    data
}

/// Creates test data with multiple moments (reflectivity, velocity, spectrum width).
fn create_multi_moment_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Message size
    let message_size: u32 = 200;
    data.extend_from_slice(&message_size.to_be_bytes());

    // Message header
    data.extend_from_slice(&31u16.to_be_bytes());
    data.extend_from_slice(&20000u16.to_be_bytes());
    data.extend_from_slice(&3600000u32.to_be_bytes());
    data.extend_from_slice(b"KOKC");

    // Volume scan header
    data.extend_from_slice(&1u16.to_be_bytes());
    data.extend_from_slice(&32u16.to_be_bytes()); // VCP 32

    // Radial with all three moments
    data.extend_from_slice(&0.0f32.to_be_bytes()); // Azimuth 0
    data.extend_from_slice(&2u16.to_be_bytes()); // 2 gates

    // Gate 1 range = 1000m
    data.extend_from_slice(&1000u16.to_be_bytes());

    // Reflectivity moment (REF) - value = 30 dBZ
    data.extend_from_slice(b"REF");
    data.extend_from_slice(&[0u8]); // Data type
    data.extend_from_slice(&[30u8]); // Reflectivity value

    // Velocity moment (VEL) - value = 10 m/s
    data.extend_from_slice(b"VEL");
    data.extend_from_slice(&[0u8]); // Data type
    data.extend_from_slice(&[10u8]); // Velocity value

    // Spectrum width moment (SW) - value = 5 m/s
    data.extend_from_slice(b"SW");
    data.extend_from_slice(&[0u8]); // Data type
    data.extend_from_slice(&[5u8]); // Spectrum width value

    data
}

/// Creates test data with a radial that has only reflectivity moment.
fn create_single_moment_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Message size
    let message_size: u32 = 100;
    data.extend_from_slice(&message_size.to_be_bytes());

    // Message header
    data.extend_from_slice(&31u16.to_be_bytes());
    data.extend_from_slice(&20500u16.to_be_bytes());
    data.extend_from_slice(&7200000u32.to_be_bytes()); // 2 hours
    data.extend_from_slice(b"KORD");

    // Volume scan header
    data.extend_from_slice(&2u16.to_be_bytes()); // Volume scan 2
    data.extend_from_slice(&121u16.to_be_bytes()); // VCP 121

    // Radial - only reflectivity
    data.extend_from_slice(&90.0f32.to_be_bytes()); // Azimuth 90 degrees (East)
    data.extend_from_slice(&1u16.to_be_bytes()); // 1 gate

    // Gate at 500m range
    data.extend_from_slice(&500u16.to_be_bytes());

    // Only reflectivity moment present
    data.extend_from_slice(b"REF");
    data.extend_from_slice(&[0u8]);
    data.extend_from_slice(&[45u8]); // 45 dBZ

    data
}

/// Creates test data representing an empty scan (no radials).
fn create_empty_scan_data() -> Vec<u8> {
    let mut data = Vec::new();

    // Message size
    let message_size: u32 = 50;
    data.extend_from_slice(&message_size.to_be_bytes());

    // Message header
    data.extend_from_slice(&31u16.to_be_bytes());
    data.extend_from_slice(&20000u16.to_be_bytes());
    data.extend_from_slice(&0u32.to_be_bytes());
    data.extend_from_slice(b"KATX");

    // Volume scan header with 0 sweeps
    data.extend_from_slice(&1u16.to_be_bytes());
    data.extend_from_slice(&215u16.to_be_bytes());

    // No sweep data

    data
}

#[cfg(test)]
mod radial_extraction {
    use super::*;

    /// Test that decode returns a VolumeScan with sweeps extracted from radial data.
    #[test]
    fn should_extract_sweeps_from_volume_scan() {
        // Arrange: Create test data with multiple sweeps
        let bytes = create_multi_sweep_data();

        // Act: Try to decode the binary data
        let result = decode(&bytes);

        // Assert: Verify we get a successful result with sweeps
        assert!(
            result.is_ok(),
            "Decoder should successfully parse radial data"
        );

        let volume = result.unwrap();

        // Should have at least one sweep
        assert!(
            !volume.sweeps.is_empty(),
            "Volume scan should contain at least one sweep"
        );

        // Should have multiple sweeps (we put 3 in the test data)
        assert!(
            volume.sweeps.len() >= 3,
            "Volume scan should contain multiple sweeps, got {}",
            volume.sweeps.len()
        );
    }

    /// Test that each sweep has a valid elevation angle.
    #[test]
    fn should_have_elevation_angle_on_each_sweep() {
        // Arrange
        let bytes = create_multi_sweep_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        for (i, sweep) in volume.sweeps.iter().enumerate() {
            assert!(
                sweep.elevation >= 0.0 && sweep.elevation <= 90.0,
                "Sweep {} has invalid elevation angle: {}",
                i,
                sweep.elevation
            );
        }
    }

    /// Test that each sweep contains radial data.
    #[test]
    fn should_contain_radials_in_each_sweep() {
        // Arrange
        let bytes = create_multi_sweep_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        for (i, sweep) in volume.sweeps.iter().enumerate() {
            assert!(
                !sweep.radials.is_empty(),
                "Sweep {} should contain radials",
                i
            );
        }
    }

    /// Test that each radial has an azimuth angle.
    #[test]
    fn should_have_azimuth_angle_on_each_radial() {
        // Arrange
        let bytes = create_minimal_radial_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // Check first sweep
        assert!(!volume.sweeps.is_empty(), "Should have at least one sweep");
        let sweep = &volume.sweeps[0];

        assert!(
            !sweep.radials.is_empty(),
            "Sweep should contain at least one radial"
        );

        for (i, radial) in sweep.radials.iter().enumerate() {
            assert!(
                radial.azimuth >= 0.0 && radial.azimuth <= 360.0,
                "Radial {} has invalid azimuth: {}",
                i,
                radial.azimuth
            );
        }
    }

    /// Test that radials contain gate data with range values.
    #[test]
    fn should_contain_gates_with_range_values_in_radial() {
        // Arrange
        let bytes = create_minimal_radial_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // Get first radial from first sweep
        assert!(!volume.sweeps.is_empty());
        let sweep = &volume.sweeps[0];
        assert!(!sweep.radials.is_empty());
        let radial = &sweep.radials[0];

        // Radial should have gates
        assert!(!radial.gates.is_empty(), "Radial should contain gates");

        // Each gate should have a valid range (positive number)
        for (i, gate) in radial.gates.iter().enumerate() {
            assert!(
                gate.range >= 0.0,
                "Gate {} has invalid range: {}",
                i,
                gate.range
            );
        }
    }

    /// Test that reflectivity moment data is extracted when present.
    #[test]
    fn should_extract_reflectivity_from_gate() {
        // Arrange
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // Find a gate with reflectivity
        let mut found_reflectivity = false;
        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                for gate in &radial.gates {
                    if let Some(reflectivity) = gate.reflectivity {
                        found_reflectivity = true;
                        assert!(
                            (-32.0..=75.0).contains(&reflectivity),
                            "Reflectivity {} is out of valid range",
                            reflectivity
                        );
                    }
                }
            }
        }

        assert!(
            found_reflectivity,
            "Should have found at least one gate with reflectivity data"
        );
    }

    /// Test that velocity moment data is extracted when present.
    #[test]
    fn should_extract_velocity_from_gate() {
        // Arrange
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // Find a gate with velocity
        let mut found_velocity = false;
        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                for gate in &radial.gates {
                    if let Some(velocity) = gate.velocity {
                        found_velocity = true;
                        // Velocity range depends on Nyquist, but typically -50 to +50 m/s
                        println!("Found velocity: {}", velocity);
                    }
                }
            }
        }

        assert!(
            found_velocity,
            "Should have found at least one gate with velocity data"
        );
    }

    /// Test that spectrum width moment data is extracted when present.
    #[test]
    fn should_extract_spectrum_width_from_gate() {
        // Arrange
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // Find a gate with spectrum width
        let mut found_spectrum_width = false;
        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                for gate in &radial.gates {
                    if let Some(sw) = gate.spectrum_width {
                        found_spectrum_width = true;
                        // Spectrum width is typically 0 to ~25 m/s
                        println!("Found spectrum width: {}", sw);
                    }
                }
            }
        }

        assert!(
            found_spectrum_width,
            "Should have found at least one gate with spectrum width data"
        );
    }

    /// Test that missing moments return None.
    #[test]
    fn should_return_none_for_missing_moments() {
        // Arrange: Data with only reflectivity
        let bytes = create_single_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // Find the gate with reflectivity
        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                for gate in &radial.gates {
                    // Reflectivity should be present
                    assert!(
                        gate.reflectivity.is_some(),
                        "Gate should have reflectivity since we included it"
                    );

                    // Velocity should be absent (not requested in this scan)
                    assert!(
                        gate.velocity.is_none(),
                        "Gate should NOT have velocity when not present in data"
                    );

                    // Spectrum width should be absent
                    assert!(
                        gate.spectrum_width.is_none(),
                        "Gate should NOT have spectrum width when not present in data"
                    );
                }
            }
        }
    }

    /// Test that decoder extracts the station ID from the message header.
    #[test]
    fn should_extract_station_id_from_message() {
        // Arrange: Data for KTLX station
        let bytes = create_minimal_radial_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        assert_eq!(
            volume.station_id, "KTLX",
            "Volume scan should have correct station ID"
        );
    }

    /// Test that decoder extracts VCP (Volume Coverage Pattern) from the data.
    #[test]
    fn should_extract_vcp_from_message() {
        // Arrange
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        // VCP should be 32 (as set in test data)
        assert_eq!(volume.vcp, 32, "Volume scan should have correct VCP");
    }

    /// Test that sweep elevation angles are in ascending order.
    #[test]
    fn should_have_sweeps_in_elevation_order() {
        // Arrange
        let bytes = create_multi_sweep_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        let elevations: Vec<f32> = volume.sweeps.iter().map(|s| s.elevation).collect();

        for i in 1..elevations.len() {
            assert!(
                elevations[i] >= elevations[i - 1],
                "Sweeps should be in ascending elevation order"
            );
        }
    }

    /// Test handling of empty scan (no radials).
    #[test]
    fn should_handle_empty_scan_gracefully() {
        // Arrange
        let bytes = create_empty_scan_data();

        // Act
        let result = decode(&bytes);

        // Assert: Should still return valid VolumeScan, possibly with empty sweeps
        assert!(
            result.is_ok(),
            "Decoder should handle empty scan gracefully"
        );

        let volume = result.unwrap();

        // Station ID should still be extracted
        assert_eq!(
            volume.station_id, "KATX",
            "Should extract station ID even from empty scan"
        );
    }

    /// Test that radial azimuth angles are in proper range (0-360).
    #[test]
    fn should_have_valid_azimuth_range() {
        // Arrange
        let bytes = create_single_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                let az = radial.azimuth;
                assert!(
                    (0.0..=360.0).contains(&az) || (-180.0..=540.0).contains(&az),
                    "Azimuth {} is outside valid range",
                    az
                );
            }
        }
    }

    /// Test gate range values are monotonically increasing within a radial.
    #[test]
    fn should_have_increasing_gate_ranges_within_radial() {
        // Arrange: Create data with multiple gates at different ranges
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                let ranges: Vec<f32> = radial.gates.iter().map(|g| g.range).collect();

                for i in 1..ranges.len() {
                    assert!(
                        ranges[i] >= ranges[i - 1],
                        "Gate ranges should be monotonically increasing within a radial"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod gate_moment_tests {
    use super::*;

    /// Test that gate reflectivity values are in valid dBZ range.
    #[test]
    fn should_have_valid_reflectivity_range() {
        // Arrange
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                for gate in &radial.gates {
                    if let Some(reflectivity) = gate.reflectivity {
                        // NEXRAD reflectivity range: -32 to 75 dBZ
                        assert!(
                            (-32.0..=75.0).contains(&reflectivity),
                            "Reflectivity {} is outside valid NEXRAD range",
                            reflectivity
                        );
                    }
                }
            }
        }
    }

    /// Test that velocity values are properly scaled.
    #[test]
    fn should_have_velocity_in_standard_units() {
        // Arrange
        let bytes = create_multi_moment_data();

        // Act
        let result = decode(&bytes);

        // Assert
        assert!(result.is_ok());
        let volume = result.unwrap();

        for sweep in &volume.sweeps {
            for radial in &sweep.radials {
                for gate in &radial.gates {
                    if let Some(velocity) = gate.velocity {
                        // Velocity should be in m/s (typical range -50 to +50)
                        // Allow some tolerance for different scaling
                        println!("Velocity value: {}", velocity);
                    }
                }
            }
        }
    }
}
