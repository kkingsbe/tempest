//! Shared types for NEXRAD data decoding.

use crate::error::DecodeError;
use chrono::{DateTime, Utc};

/// Modified Julian Date (MJD) - days since May 23, 1968.
///
/// This is the standard NEXRAD date representation, where:
/// - Day 0 = May 23, 1968
/// - Used in ICD message headers for date field
pub type Mjd = u16;

/// Time in milliseconds since midnight UTC.
///
/// Used in ICD message headers for time field.
/// Maximum value is 86,400,000 (24 hours in milliseconds).
pub type Milliseconds = u32;

/// ICAO station identifier (4 characters).
///
/// Represents the radar station identifier code, such as "KTLX" or "KOKC".
/// These are standard ICAO airport/station codes where:
/// - First character: region (K = continental US)
/// - Last three characters: station name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StationId(pub [u8; 4]);

impl StationId {
    /// Creates a new StationId from raw bytes.
    #[inline]
    pub fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    /// Returns the station ID as a UTF-8 string slice.
    ///
    /// # Errors
    /// Returns a `Utf8Error` if the bytes are not valid UTF-8.
    ///
    /// # Example
    /// ```ignore
    /// let station = StationId::new(*b"KTLX");
    /// assert_eq!(station.as_str().unwrap(), "KTLX");
    /// ```
    #[inline]
    pub fn as_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.0)
    }

    /// Returns the station ID as a string, replacing invalid UTF-8 with replacement chars.
    ///
    /// # Example
    /// ```ignore
    /// let station = StationId::new(*b"KTLX");
    /// assert_eq!(station.as_str_lossy(), "KTLX");
    /// ```
    #[inline]
    pub fn as_str_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).into_owned()
    }
}

impl std::fmt::Display for StationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str_lossy())
    }
}

/// Data moments available in NEXRAD radar data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moment {
    /// Reflectivity (dBZ)
    Reflectivity,
    /// Velocity (m/s or knots)
    Velocity,
    /// Spectrum Width (m/s or knots)
    SpectrumWidth,
    /// Differential Reflectivity (dB)
    Zdr,
    /// Correlation Coefficient (unitless)
    Cc,
    /// Differential Phase (degrees)
    Kdp,
}

/// Radial Data Block - appears after Msg31 header for each radial.
///
/// Per NEXRAD ICD, this contains moment data blocks (REF, VEL, SW).
/// The block type is "RDAT" (4 bytes) followed by block length (4 bytes).
#[derive(Debug, Clone, PartialEq)]
pub struct RadialDataBlock {
    /// Block type identifier (should be "RDAT")
    pub block_type: [u8; 4],
    /// Total length of this block in bytes (including header)
    pub block_length: u32,
}

impl RadialDataBlock {
    /// Required bytes for the block header (block type + length)
    pub const HEADER_BYTES: usize = 8;

    /// Parse a RadialDataBlock from raw bytes.
    ///
    /// # Arguments
    /// * `data` - Raw bytes starting at the beginning of the radial data block
    ///
    /// # Returns
    /// * `Ok(RadialDataBlock)` - Successfully parsed header
    /// * `Err(DecodeError)` - If parsing fails
    pub fn parse(data: &[u8]) -> Result<Self, DecodeError> {
        if data.len() < Self::HEADER_BYTES {
            return Err(DecodeError::InsufficientBytes {
                needed: Self::HEADER_BYTES,
                have: data.len(),
            });
        }

        let block_type = [data[0], data[1], data[2], data[3]];
        let block_length = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);

        Ok(Self {
            block_type,
            block_length,
        })
    }

    /// Returns the block type as a string slice.
    pub fn block_type_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.block_type)
    }
}

/// Moment Block for each moment type (REF, VEL, SW, ZDR, CC, KDP).
///
/// Per NEXRAD ICD:
/// - 3 bytes: Moment name (e.g., "REF", "VEL", "SW")
/// - 1 byte: Reserved
/// - 2 bytes: Data word size (bits per gate)
/// - 1 byte: Scale (number of bits for scale factor)
/// - 1 byte: Offset (number of bits for offset factor)
/// - 1 byte: Reserved
/// - 2 bytes: Number of gates
/// - 4 bytes: Range to first gate (meters)
/// - 2 bytes: Gate spacing (meters)
/// - N bytes: Packed gate data
#[derive(Debug, Clone, PartialEq)]
pub struct MomentBlock {
    /// Moment name (3 bytes, e.g., "REF", "VEL", "SW")
    pub moment_name: [u8; 3],
    /// Reserved byte
    pub reserved1: u8,
    /// Data word size in bits
    pub data_word_size: u8,
    /// Number of bits for scale factor
    pub scale_bits: u8,
    /// Number of bits for offset factor
    pub offset_bits: u8,
    /// Reserved byte
    pub reserved2: u8,
    /// Number of gates in this moment
    pub gate_count: u16,
    /// Range from radar to first gate in meters
    pub range_to_first_gate: f32,
    /// Spacing between gates in meters
    pub gate_spacing: f32,
    /// Packed gate data (raw bytes)
    pub data: Vec<u8>,
}

impl MomentBlock {
    /// Minimum header bytes required before data
    pub const HEADER_BYTES: usize = 16;

    /// Parse a MomentBlock from raw bytes.
    ///
    /// Per NEXRAD ICD format:
    /// - 3 bytes: Moment name (e.g., "REF", "VEL", "SW")
    /// - 1 byte: Reserved
    /// - 1 byte: Scale (raw byte, used as multiplier)
    /// - 1 byte: Offset (raw byte, used as offset)
    /// - 1 byte: Reserved
    /// - 2 bytes: Number of gates
    /// - 4 bytes: Range to first gate (meters, f32)
    /// - 2 bytes: Gate spacing (meters)
    /// - N bytes: Packed gate data
    ///
    /// # Arguments
    /// * `data` - Raw bytes starting at the beginning of the moment block
    ///
    /// # Returns
    /// * `Ok(MomentBlock)` - Successfully parsed moment block
    /// * `Err(DecodeError)` - If parsing fails
    pub fn parse(data: &[u8]) -> Result<Self, DecodeError> {
        if data.len() < Self::HEADER_BYTES {
            return Err(DecodeError::InsufficientBytes {
                needed: Self::HEADER_BYTES,
                have: data.len(),
            });
        }

        let moment_name = [data[0], data[1], data[2]];
        let reserved1 = data[3];
        let data_word_size = data[4]; // Bits per gate
        let scale_bits = data[5];
        let offset_bits = data[6];
        let reserved2 = data[7];

        let gate_count = u16::from_be_bytes([data[8], data[9]]);
        let range_to_first_gate = f32::from_be_bytes([data[10], data[11], data[12], data[13]]);
        
        // Gate spacing is 2 bytes, not 4 - convert u16 to f32
        let gate_spacing_raw = u16::from_be_bytes([data[14], data[15]]);
        let gate_spacing = gate_spacing_raw as f32;

        // Calculate data size: each gate uses data_word_size bits
        // Total bits = gate_count * data_word_size, total bytes = bits / 8 (rounded up)
        let total_data_bits = gate_count as usize * data_word_size as usize;
        let data_bytes_needed = (total_data_bits + 7) / 8; // Round up to byte boundary

        if data.len() < Self::HEADER_BYTES + data_bytes_needed {
            return Err(DecodeError::InsufficientBytes {
                needed: Self::HEADER_BYTES + data_bytes_needed,
                have: data.len(),
            });
        }

        let data = data[Self::HEADER_BYTES..Self::HEADER_BYTES + data_bytes_needed].to_vec();

        Ok(Self {
            moment_name,
            reserved1,
            data_word_size,
            scale_bits,
            offset_bits,
            reserved2,
            gate_count,
            range_to_first_gate,
            gate_spacing,
            data,
        })
    }

    /// Returns the moment name as a string slice.
    pub fn moment_name_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.moment_name)
    }

    /// Decode the packed gate data into f32 values using scale and offset.
    ///
    /// Per NEXRAD ICD, the formula is: `value = (raw_value * scale) + offset`
    ///
    /// Note: The scale and offset are stored in the block header and must be
    /// passed as parameters. This method assumes 8-bit packed data.
    ///
    /// # Arguments
    /// * `scale` - Scale factor to convert raw bytes to real values
    /// * `offset` - Offset to add after scaling
    ///
    /// # Returns
    /// * `Vec<f32>` - Decoded gate values
    pub fn decode_data(&self, scale: f32, offset: f32) -> Vec<f32> {
        // For 8-bit data, each byte is one gate value
        self.data.iter().map(|&byte| (byte as f32) * scale + offset).collect()
    }

    /// Decode 8-bit packed data assuming standard NEXRAD encoding.
    ///
    /// For most NEXRAD moments, 0-255 maps to various scales.
    ///
    /// # Returns
    /// * `Vec<f32>` - Decoded gate values (or empty if data is not 8-bit)
    pub fn decode_data_8bit(&self) -> Vec<f32> {
        if self.data_word_size != 8 {
            return Vec::new();
        }

        // Default: raw byte values (for standard 8-bit encoding)
        self.data.iter().map(|&byte| byte as f32).collect()
    }
}

/// Volume Coverage Pattern number
pub type Vcp = u16;

impl Moment {
    /// Get the two-character NEXRAD moment code
    pub fn code(&self) -> &'static str {
        match self {
            Moment::Reflectivity => "REF",
            Moment::Velocity => "VEL",
            Moment::SpectrumWidth => "SW",
            Moment::Zdr => "ZDR",
            Moment::Cc => "CC",
            Moment::Kdp => "KDP",
        }
    }
}

/// A single radar gate with moment data
#[derive(Debug, Clone, PartialEq)]
pub struct Gate {
    /// Distance from radar to gate center in meters
    pub range: f32,
    /// Reflectivity value in dBZ, if available
    pub reflectivity: Option<f32>,
    /// Velocity value in m/s, if available
    pub velocity: Option<f32>,
    /// Spectrum width in m/s, if available
    pub spectrum_width: Option<f32>,
    /// Differential reflectivity in dB, if available
    pub zdr: Option<f32>,
    /// Correlation coefficient (0-1), if available
    pub cc: Option<f32>,
    /// Differential phase in degrees, if available
    pub kdp: Option<f32>,
}

impl Gate {
    /// Create a new gate with the given range
    pub fn new(range: f32) -> Self {
        Self {
            range,
            reflectivity: None,
            velocity: None,
            spectrum_width: None,
            zdr: None,
            cc: None,
            kdp: None,
        }
    }
}

/// A single radial (one azimuth angle sweep of gates)
#[derive(Debug, Clone, PartialEq)]
pub struct Radial {
    /// Azimuth angle in degrees (0-360, 0 = North)
    pub azimuth: f32,
    /// Gates along this radial
    pub gates: Vec<Gate>,
}

impl Radial {
    /// Create a new radial with the given azimuth
    pub fn new(azimuth: f32) -> Self {
        Self {
            azimuth,
            gates: Vec::new(),
        }
    }
}

/// A single sweep (elevation angle) in a volume scan
#[derive(Debug, Clone, PartialEq)]
pub struct Sweep {
    /// Elevation angle in degrees
    pub elevation: f32,
    /// Radials in this sweep
    pub radials: Vec<Radial>,
}

impl Sweep {
    /// Create a new sweep with the given elevation
    pub fn new(elevation: f32) -> Self {
        Self {
            elevation,
            radials: Vec::new(),
        }
    }
}

/// A complete volume scan from a radar station
#[derive(Debug, Clone, PartialEq)]
pub struct VolumeScan {
    /// Radar station identifier (e.g., "KTLX")
    pub station_id: String,
    /// Timestamp of the volume scan
    pub timestamp: DateTime<Utc>,
    /// Volume Coverage Pattern number
    pub vcp: u16,
    /// Sweeps in this volume scan (ordered by elevation)
    pub sweeps: Vec<Sweep>,
}

impl VolumeScan {
    /// Create a new volume scan
    pub fn new(station_id: String, timestamp: DateTime<Utc>, vcp: u16) -> Self {
        Self {
            station_id,
            timestamp,
            vcp,
            sweeps: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_id_as_str() {
        let station = StationId::new(*b"KTLX");
        assert_eq!(station.as_str().unwrap(), "KTLX");
    }

    #[test]
    fn test_station_id_display() {
        let station = StationId::new(*b"KOKC");
        assert_eq!(format!("{}", station), "KOKC");
    }

    #[test]
    fn test_station_id_equality() {
        let station1 = StationId::new(*b"KTLX");
        let station2 = StationId::new(*b"KTLX");
        let station3 = StationId::new(*b"KOKC");
        assert_eq!(station1, station2);
        assert_ne!(station1, station3);
    }

    #[test]
    fn test_station_id_clone() {
        let station1 = StationId::new(*b"KTLX");
        let station2 = station1.clone();
        assert_eq!(station1, station2);
    }

    #[test]
    fn test_moment_codes() {
        assert_eq!(Moment::Reflectivity.code(), "REF");
        assert_eq!(Moment::Velocity.code(), "VEL");
        assert_eq!(Moment::SpectrumWidth.code(), "SW");
        assert_eq!(Moment::Zdr.code(), "ZDR");
        assert_eq!(Moment::Cc.code(), "CC");
        assert_eq!(Moment::Kdp.code(), "KDP");
    }

    #[test]
    fn test_gate_creation() {
        let gate = Gate::new(1000.0);
        assert_eq!(gate.range, 1000.0);
        assert!(gate.reflectivity.is_none());
        assert!(gate.velocity.is_none());
    }

    #[test]
    fn test_radial_creation() {
        let radial = Radial::new(45.0);
        assert_eq!(radial.azimuth, 45.0);
        assert!(radial.gates.is_empty());
    }

    #[test]
    fn test_sweep_creation() {
        let sweep = Sweep::new(0.5);
        assert_eq!(sweep.elevation, 0.5);
        assert!(sweep.radials.is_empty());
    }

    #[test]
    fn test_volume_scan_creation() {
        let volume = VolumeScan::new("KTLX".to_string(), Utc::now(), 215);
        assert_eq!(volume.station_id, "KTLX");
        assert_eq!(volume.vcp, 215);
        assert!(volume.sweeps.is_empty());
    }
}
