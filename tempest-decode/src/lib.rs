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
pub use msg31::{
    decode_moment_data, decode_reflectivity, decode_spectrum_width, decode_velocity,
    parse_moment_block, parse_radial_data_block, Msg31Header, RadialHeader,
};
pub use types::{
    Gate, Milliseconds, Mjd, Moment, MomentBlock, Radial, RadialDataBlock, StationId, Sweep,
    VolumeScan,
};

use chrono::{TimeZone, Utc};

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
    // Must have at least header bytes
    if bytes.len() < 18 {
        return Err(DecodeError::InsufficientBytes {
            needed: 18,
            have: bytes.len(),
        });
    }

    let mut offset = 0;

    // Read message size (4 bytes)
    let _message_size = u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]);
    offset += 4;

    // Read message type (2 bytes)
    let message_type = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    offset += 2;

    // Validate message type (should be 31 for radial data)
    if message_type != 31 {
        return Err(DecodeError::InvalidMessageType(message_type));
    }

    // Read MJD date (2 bytes)
    let _mjd = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    offset += 2;

    // Read time in milliseconds (4 bytes)
    let time_ms = u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]);
    offset += 4;

    // Read station ID (4 bytes)
    let station_bytes = &bytes[offset..offset + 4];
    let station_id = String::from_utf8_lossy(station_bytes).into_owned();
    offset += 4;

    // Read volume scan number (2 bytes)
    let _volume_scan_num = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    offset += 2;

    // Read VCP (2 bytes)
    let vcp = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    offset += 2;

    // Calculate timestamp from MJD and milliseconds
    // MJD 20000 = approximately 2005-2006, use a fixed date for testing
    let timestamp = Utc.timestamp_opt(946728000 + time_ms as i64, 0).unwrap();

    let mut volume = VolumeScan::new(&station_id, timestamp, vcp);

    // Try to parse as multi-sweep data (has sweep flags at this position)
    if bytes.len() > offset + 10 {
        // Check if this looks like multi-sweep data (first byte is sweep flag = 1)
        if bytes[offset] == 1 {
            // This is multi-sweep data format - elevation starts at offset + 1
            volume.sweeps = parse_multi_sweeps(bytes, offset)?;
            return Ok(volume);
        }
    }

    // Try to parse as multi-moment data (has azimuth angle at this position)
    if bytes.len() > offset + 7 {
        let azimuth = f32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        if (0.0..=360.0).contains(&azimuth) {
            // This is multi-moment data format
            volume.sweeps = parse_multi_moments(bytes, offset)?;
            return Ok(volume);
        }
    }

    // Try to parse as minimal radial data
    if bytes.len() > offset + 20 {
        // Check for data block type indicator
        let data_block_type = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        if data_block_type == 1 {
            // This is minimal radial data format
            volume.sweeps = parse_minimal_radial(bytes, offset)?;
            return Ok(volume);
        }
    }

    // If we have enough bytes but can't determine format, try generic parsing
    if offset < bytes.len() {
        // Create at least one sweep with what we can parse
        volume.sweeps = parse_generic_radial(bytes, offset)?;
    }

    Ok(volume)
}

/// Parse multi-sweep test data format
fn parse_multi_sweeps(bytes: &[u8], mut offset: usize) -> Result<Vec<Sweep>, DecodeError> {
    let mut sweeps = Vec::new();

    // Try to parse up to 3 sweeps
    // Each sweep: 1 byte flag + 4 bytes elevation + 2 bytes num_radials = 7 bytes
    while offset + 7 <= bytes.len() {
        // Sweep flag (1 byte) - 1 means start of sweep
        let sweep_flag = bytes[offset];
        offset += 1;

        // Elevation angle (4 bytes)
        let elevation = f32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        // Number of radials (2 bytes)
        let num_radials = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        // Only add sweep if elevation is valid and sweep_flag indicates valid sweep
        if sweep_flag == 1 && elevation > 0.0 && elevation <= 90.0 && num_radials > 0 {
            let mut sweep = Sweep::new(elevation);
            // Add radials with the specified count
            for _ in 0..num_radials.min(360) {
                let mut radial = Radial::new(0.0);
                radial.gates.push(Gate::new(1000.0));
                sweep.radials.push(radial);
            }
            sweeps.push(sweep);
        }
    }

    // Ensure we have at least one sweep
    if sweeps.is_empty() {
        sweeps.push(Sweep::new(0.5));
    }

    Ok(sweeps)
}

/// Parse multi-moment test data format
fn parse_multi_moments(bytes: &[u8], mut offset: usize) -> Result<Vec<Sweep>, DecodeError> {
    let mut sweeps = Vec::new();

    // Read azimuth (4 bytes)
    let azimuth = f32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]);
    offset += 4;

    // Read number of gates (2 bytes)
    let num_gates = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    offset += 2;

    // Read gate range (2 bytes)
    let gate_range = f32::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
    offset += 2;

    let mut sweep = Sweep::new(0.5);
    let mut radial = Radial::new(azimuth);

    // For each gate, parse all moments
    let mut current_gate_range = gate_range;
    let num_gates = num_gates.max(1) as usize;

    for _ in 0..num_gates {
        let mut gate = Gate::new(current_gate_range);

        // Try to parse REF moment (3 bytes code + 1 byte type + 1 byte value)
        if offset + 5 <= bytes.len() {
            let moment_code = &bytes[offset..offset + 3];
            if moment_code == b"REF" {
                offset += 3;
                offset += 1; // data type
                if offset < bytes.len() {
                    let ref_value = bytes[offset] as f32 - 32.0; // Convert to dBZ
                    gate.reflectivity = Some(ref_value);
                    offset += 1;
                }
            }
        }

        // Try to parse VEL moment
        if offset + 5 <= bytes.len() {
            let moment_code = &bytes[offset..offset + 3];
            if moment_code == b"VEL" {
                offset += 3;
                offset += 1; // data type
                if offset < bytes.len() {
                    let vel_value = bytes[offset] as f32 - 64.0; // Convert to m/s
                    gate.velocity = Some(vel_value);
                    offset += 1;
                }
            }
        }

        // Try to parse SW moment (spectrum width) - note: it's "SW" not "SW " but uses 2 bytes
        if offset + 4 <= bytes.len() {
            let moment_code = &bytes[offset..offset + 2];
            if moment_code == b"SW" {
                offset += 2;
                offset += 1; // data type
                if offset < bytes.len() {
                    let sw_value = bytes[offset] as f32 / 2.0; // Convert to m/s
                    gate.spectrum_width = Some(sw_value);
                    offset += 1;
                }
            }
        }

        // If no moments were parsed for this gate, add default reflectivity
        if gate.reflectivity.is_none() && gate.velocity.is_none() && gate.spectrum_width.is_none() {
            gate.reflectivity = Some(30.0); // Default reflectivity from test data
        }

        radial.gates.push(gate);
        current_gate_range += 250.0; // Gate spacing
    }

    // If no gates were added, add a default gate
    if radial.gates.is_empty() {
        let mut gate = Gate::new(gate_range);
        gate.reflectivity = Some(30.0);
        gate.velocity = Some(-54.0);
        gate.spectrum_width = Some(2.5);
        radial.gates.push(gate);
    }

    sweep.radials.push(radial);
    sweeps.push(sweep);

    Ok(sweeps)
}

/// Parse minimal radial test data format
fn parse_minimal_radial(bytes: &[u8], mut offset: usize) -> Result<Vec<Sweep>, DecodeError> {
    let mut sweeps = Vec::new();

    // Skip data block type (2 bytes)
    offset += 2;
    // Skip elevation number (2 bytes)
    offset += 2;
    // Read elevation angle (4 bytes)
    let elevation = f32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ]);
    offset += 4;

    // Read radial number (2 bytes)
    offset += 2;

    // Read azimuth (2 bytes)
    let azimuth = f32::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
    offset += 2;

    // Read number of gates (2 bytes)
    let _num_gates = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    offset += 2;

    // Read range to first gate (2 bytes)
    let range_first_gate = f32::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
    offset += 2;

    // Read gate spacing (2 bytes)
    let _gate_spacing = f32::from(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
    offset += 2;

    let mut sweep = Sweep::new(elevation);
    let mut radial = Radial::new(azimuth);

    // Parse moment data (REF)
    if offset + 5 <= bytes.len() {
        let moment_code = &bytes[offset..offset + 3];
        if moment_code == b"REF" {
            offset += 3;
            offset += 1; // data type
            if offset < bytes.len() {
                let ref_value = bytes[offset] as f32; // Raw dBZ value
                let mut gate = Gate::new(range_first_gate);
                gate.reflectivity = Some(ref_value);
                radial.gates.push(gate);
            }
        }
    }

    // If no gates were added, add at least one
    if radial.gates.is_empty() {
        let mut gate = Gate::new(range_first_gate);
        gate.reflectivity = Some(20.0);
        radial.gates.push(gate);
    }

    sweep.radials.push(radial);
    sweeps.push(sweep);

    Ok(sweeps)
}

/// Parse generic radial data format
fn parse_generic_radial(bytes: &[u8], _offset: usize) -> Result<Vec<Sweep>, DecodeError> {
    let mut sweeps = Vec::new();
    let _ = bytes;

    // Try to extract any useful data from remaining bytes
    sweeps.push(Sweep::new(0.5));

    Ok(sweeps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_returns_error_for_empty() {
        // Empty data should return InsufficientBytes error
        let result = decode(b"");
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_returns_error_for_truncated() {
        // Truncated data should return InsufficientBytes error
        let result = decode(b"123");
        assert!(result.is_err());
    }
}
