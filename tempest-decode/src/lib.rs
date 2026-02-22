//! Tempest Decode - NEXRAD Archive2 decoder library.
//!
//! This library provides parsing for NEXRAD Level II (Archive2) radar data messages.

pub mod error;
pub mod msg1;
pub mod msg31;
pub mod types;

#[cfg(test)]
mod radial_tests;

// Re-export commonly used types
pub use error::DecodeError;
pub use msg1::Msg1Header;
pub use msg31::Msg31Header;
pub use types::{Gate, Milliseconds, Mjd, Moment, Radial, StationId, Sweep, VolumeScan};

/// Decode NEXRAD Archive2 binary data into a VolumeScan
///
/// # Arguments
///
/// * `bytes` - Raw NEXRAD Archive2 binary data
///
/// # Returns
///
/// * `Ok(VolumeScan)` - Successfully decoded radar data
/// * `Err(DecodeError)` - Error during decoding
///
/// # Examples
///
/// ```ignore
/// use tempest_decode::decode;
///
/// let data = std::fs::read("radar_data.ar2v").unwrap();
/// let volume = decode(&data).expect("Failed to decode radar data");
/// println!("Station: {}", volume.station_id);
/// println!("Sweeps: {}", volume.sweeps.len());
/// ```
pub fn decode(bytes: &[u8]) -> Result<VolumeScan, DecodeError> {
    // Placeholder implementation - return an error for TDD
    // Actual parsing logic will be implemented in subsequent tasks
    let _ = bytes;
    Err(DecodeError::Truncated(
        "Decoder not yet implemented".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_returns_error() {
        // This test verifies that decode returns an error (placeholder behavior)
        // In TDD, we expect this to fail once we implement actual parsing
        let result = decode(b"");
        assert!(result.is_err());

        // Verify it's the expected Truncated error
        match result {
            Err(DecodeError::Truncated(msg)) => {
                assert_eq!(msg, "Decoder not yet implemented");
            }
            _ => panic!("Expected DecodeError::Truncated"),
        }
    }
}
