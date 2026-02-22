//! Message Type 31 (Digital Radar Data) parsing.
//!
//! This module provides structures and parsing for NEXRAD Message Type 31,
//! which is the "Digital Radar Data" message used in Archive2 format.

use crate::error::DecodeError;
use crate::types::{Milliseconds, Mjd, MomentBlock, RadialDataBlock, StationId};

/// Message Type 31 header (first 12 bytes after message size).
///
/// According to NEXRAD ICD:
/// - ID: 2 bytes (should be 31 for this message type)
/// - Date: 2 bytes (Modified Julian Date)
/// - Time: 4 bytes (milliseconds since midnight UTC)
/// - Station ID: 4 bytes (ICAO station identifier)
#[derive(Debug, Clone, PartialEq)]
pub struct Msg31Header {
    /// Message type identifier (should be 31 for Digital Radar Data)
    pub id: u16,
    /// Date in Modified Julian Date format (days since May 23, 1968)
    pub date: Mjd,
    /// Time in milliseconds since midnight UTC
    pub time: Milliseconds,
    /// Radar station ICAO identifier
    pub station_id: StationId,
}

impl Msg31Header {
    /// Required minimum bytes for the header (after message size is consumed).
    pub const REQUIRED_BYTES: usize = 12;

    /// Parses the Message Type 31 header from raw bytes.
    ///
    /// The input bytes should NOT include the message size (first 4 bytes of the message).
    /// This method expects exactly 12 bytes: ID (2) + Date (2) + Time (4) + Station ID (4).
    ///
    /// # Arguments
    /// * `bytes` - Raw bytes starting from the message ID (after message size)
    ///
    /// # Returns
    /// * `Ok(Msg31Header)` - Successfully parsed header
    /// * `Err(DecodeError)` - If parsing fails (insufficient bytes or invalid data)
    ///
    /// # Example
    /// ```ignore
    /// use tempest_decode::Msg31Header;
    ///
    /// let bytes: &[u8] = &[
    ///     0x00, 0x1F,  // ID = 31
    ///     0xA2, 0x1B,  // Date (MJD)
    ///     0x00, 0x1C, 0x9C, 0x40, // Time in ms
    ///     b'K', b'T', b'L', b'X', // Station ID
    /// ];
    ///
    /// let header = Msg31Header::parse(bytes).unwrap();
    /// assert_eq!(header.id, 31);
    /// assert_eq!(header.station_id.as_str().unwrap(), "KTLX");
    /// ```
    pub fn parse(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() < Self::REQUIRED_BYTES {
            return Err(DecodeError::InsufficientBytes {
                needed: Self::REQUIRED_BYTES,
                have: bytes.len(),
            });
        }

        // Parse message ID (u16, big-endian/NEXRAD standard)
        let id = u16::from_be_bytes([bytes[0], bytes[1]]);

        // Validate message type is 31
        if id != 31 {
            return Err(DecodeError::InvalidMessageType(id));
        }

        // Parse date (MJD, u16, big-endian)
        let date = Mjd::from_be_bytes([bytes[2], bytes[3]]);

        // Parse time (milliseconds since midnight, u32, big-endian)
        let time = Milliseconds::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

        // Parse station ID (4 bytes, ASCII)
        let station_bytes = [bytes[8], bytes[9], bytes[10], bytes[11]];
        let station_id = StationId::new(station_bytes);

        // Validate station ID is valid UTF-8
        station_id.as_str()?;

        Ok(Self {
            id,
            date,
            time,
            station_id,
        })
    }

    /// Returns the station identifier as a string slice.
    ///
    /// # Errors
    /// Returns a `Utf8Error` if the station ID bytes are not valid UTF-8.
    #[inline]
    pub fn station_identifier(&self) -> Result<&str, std::str::Utf8Error> {
        self.station_id.as_str()
    }

    /// Returns the time as hours, minutes, seconds, and milliseconds.
    #[inline]
    pub fn time_hmsms(&self) -> (u8, u8, u8, u16) {
        let total_ms = self.time;
        let hours = (total_ms / 3_600_000) as u8;
        let minutes = ((total_ms % 3_600_000) / 60_000) as u8;
        let seconds = ((total_ms % 60_000) / 1_000) as u8;
        let milliseconds = (total_ms % 1_000) as u16;
        (hours, minutes, seconds, milliseconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_msg31_header() {
        // Sample bytes for KTLX station
        // ID = 31 (0x001F), Date = some MJD, Time = 1850176 ms
        let bytes: &[u8] = &[
            0x00, 0x1F, // ID = 31
            0xA2, 0x1B, // Date (MJD)
            0x00, 0x1C, 0x9C, 0x40, // Time in ms
            b'K', b'T', b'L', b'X', // Station ID
        ];

        let header = Msg31Header::parse(bytes).unwrap();
        assert_eq!(header.id, 31);
        assert_eq!(header.date, 0xA21B); // MJD big-endian = 41499
        assert_eq!(header.time, 0x001C9C40); // 1850176 ms in big-endian
        assert_eq!(header.station_id.as_str().unwrap(), "KTLX");
    }

    #[test]
    fn test_parse_msg31_header_kokc() {
        let bytes: &[u8] = &[
            0x00, 0x1F, // ID = 31
            0xB0, 0x1C, // Date (MJD)
            0x80, 0x96, 0x98, 0x00, // Time in ms
            b'K', b'O', b'K', b'C', // Station ID
        ];

        let header = Msg31Header::parse(bytes).unwrap();
        assert_eq!(header.id, 31);
        assert_eq!(header.station_id.as_str().unwrap(), "KOKC");
    }

    #[test]
    fn test_parse_insufficient_bytes() {
        let bytes: &[u8] = &[0x00, 0x1F]; // Only 2 bytes

        let result = Msg31Header::parse(bytes);
        assert!(matches!(
            result,
            Err(DecodeError::InsufficientBytes {
                needed: 12,
                have: 2
            })
        ));
    }

    #[test]
    fn test_parse_invalid_message_type() {
        let bytes: &[u8] = &[
            0x00, 0x05, // ID = 5 (not 31)
            0x00, 0x00, // Date
            0x00, 0x00, 0x00, 0x00, // Time
            b'K', b'T', b'L', b'X', // Station ID
        ];

        let result = Msg31Header::parse(bytes);
        assert!(matches!(result, Err(DecodeError::InvalidMessageType(5))));
    }

    #[test]
    fn test_time_hmsms() {
        // 12:00:00.000 = 12 * 3600000 = 43200000 ms
        let header = Msg31Header {
            id: 31,
            date: 20500,
            time: 43_200_000,
            station_id: StationId::new(*b"KTLX"),
        };

        let (hours, minutes, seconds, milliseconds) = header.time_hmsms();
        assert_eq!(hours, 12);
        assert_eq!(minutes, 0);
        assert_eq!(seconds, 0);
        assert_eq!(milliseconds, 0);
    }

    #[test]
    fn test_time_hmsms_complex() {
        // 14:30:45.123 = 14*3600000 + 30*60000 + 45*1000 + 123 = 52245123 ms
        let header = Msg31Header {
            id: 31,
            date: 20500,
            time: 52_245_123,
            station_id: StationId::new(*b"KTLX"),
        };

        let (hours, minutes, seconds, milliseconds) = header.time_hmsms();
        assert_eq!(hours, 14);
        assert_eq!(minutes, 30);
        assert_eq!(seconds, 45);
        assert_eq!(milliseconds, 123);
    }

    #[test]
    fn test_station_identifier() {
        let header = Msg31Header {
            id: 31,
            date: 20500,
            time: 0,
            station_id: StationId::new(*b"KTLX"),
        };

        assert_eq!(header.station_identifier().unwrap(), "KTLX");
    }
}

/// Radial information that follows theMsg31 header.
///
/// Per NEXRAD ICD, this contains the radial's azimuth and elevation angles.
#[derive(Debug, Clone, PartialEq)]
pub struct RadialHeader {
    /// Radial sequence number (1-3600)
    pub radial_number: u16,
    /// Calibration date/time (not fully parsed)
    pub calibration: u32,
    /// Azimuth angle in degrees (0-360)
    pub azimuth: f32,
    /// Radial status flags
    pub radial_status: u8,
    /// Elevation angle in degrees
    pub elevation: f32,
    /// Elevation number
    pub elevation_number: u8,
    /// Number of data blocks following
    pub data_blocks: u8,
}

impl RadialHeader {
    /// Required bytes for the radial header
    pub const REQUIRED_BYTES: usize = 12;

    /// Parse a radial header from raw bytes.
    ///
    /// # Arguments
    /// * `data` - Raw bytes starting at the radial header
    ///
    /// # Returns
    /// * `Ok(RadialHeader)` - Successfully parsed header
    /// * `Err(DecodeError)` - If parsing fails
    pub fn parse(data: &[u8]) -> Result<Self, DecodeError> {
        if data.len() < Self::REQUIRED_BYTES {
            return Err(DecodeError::InsufficientBytes {
                needed: Self::REQUIRED_BYTES,
                have: data.len(),
            });
        }

        let radial_number = u16::from_be_bytes([data[0], data[1]]);
        let calibration = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);
        let azimuth = f32::from_be_bytes([data[6], data[7], data[8], data[9]]);
        let radial_status = data[10];
        let elevation = f32::from_be_bytes([data[11], data[12], data[13], data[14]]);
        let elevation_number = data[15];
        let data_blocks = data[16];

        Ok(Self {
            radial_number,
            calibration,
            azimuth,
            radial_status,
            elevation,
            elevation_number,
            data_blocks,
        })
    }
}

/// Parse a single moment block from raw bytes.
///
/// Per NEXRAD ICD:
/// - 3 bytes: Moment name (e.g., "REF", "VEL", "SW")
/// - 1 byte: Reserved
/// - 1 byte: Data word size (bits per gate)
/// - 1 byte: Scale (number of bits for scale)
/// - 1 byte: Offset (number of bits for offset)
/// - 1 byte: Reserved
/// - 2 bytes: Number of gates
/// - 4 bytes: Range to first gate (meters)
/// - 2 bytes: Gate spacing (meters)
/// - N bytes: Packed gate data
///
/// # Arguments
/// * `data` - Raw bytes starting at the moment block
///
/// # Returns
/// * `Ok(MomentBlock)` - Successfully parsed moment block
/// * `Err(DecodeError)` - If parsing fails
pub fn parse_moment_block(data: &[u8]) -> Result<MomentBlock, DecodeError> {
    MomentBlock::parse(data)
}

/// Parse a complete radial data block with all its moment blocks.
///
/// A radial data block consists of:
/// - 4 bytes: Block type ("RDAT")
/// - 4 bytes: Block length
/// - Then moment blocks (REF, VEL, SW, etc.)
///
/// # Arguments
/// * `data` - Raw bytes starting at the radial data block
///
/// # Returns
/// * `Ok((RadialDataBlock, Vec<MomentBlock>))` - Parsed radial data block and its moments
/// * `Err(DecodeError)` - If parsing fails
pub fn parse_radial_data_block(
    data: &[u8],
) -> Result<(RadialDataBlock, Vec<MomentBlock>), DecodeError> {
    // Parse the radial data block header
    let rdb = RadialDataBlock::parse(data)?;

    let mut offset = RadialDataBlock::HEADER_BYTES;
    let mut moments = Vec::new();

    // Parse all moment blocks until we reach the end of the radial data block
    while offset + 8 <= rdb.block_length as usize
        && offset + MomentBlock::HEADER_BYTES <= data.len()
    {
        // Check if we have a valid moment block signature (3 ASCII characters)
        let moment_name = [&data[offset], &data[offset + 1], &data[offset + 2]];

        // Validate moment name is ASCII letters
        if !moment_name.iter().all(|&b| b.is_ascii_alphabetic()) {
            break;
        }

        match parse_moment_block(&data[offset..]) {
            Ok(moment) => {
                // Calculate bytes consumed by this moment block
                let moment_size = MomentBlock::HEADER_BYTES + moment.data.len();
                offset += moment_size;
                moments.push(moment);
            }
            Err(_) => {
                // If we can't parse, skip to next possible moment block
                break;
            }
        }
    }

    Ok((rdb, moments))
}

/// Decode packed moment data bytes into f32 values.
///
/// Per NEXRAD ICD, the formula is: `value = (raw_value * scale) + offset`
///
/// For most cases with 8-bit data:
/// - REF: value = (byte as f32) * 0.5 - 32.0 (dBZ)
/// - VEL: value = (byte as f32) * 2.0 - 64.0 (m/s)
/// - SW: value = (byte as f32) * 0.5 (m/s)
///
/// # Arguments
/// * `block` - The moment block containing the packed data
/// * `scale` - Scale factor
/// * `offset` - Offset factor
///
/// # Returns
/// * `Vec<f32>` - Decoded gate values
pub fn decode_moment_data(block: &MomentBlock, scale: f32, offset: f32) -> Vec<f32> {
    block.decode_data(scale, offset)
}

/// Decode REF (Reflectivity) data using standard NEXRAD encoding.
///
/// REF uses 8-bit encoding where:
/// - 0 = No data (below threshold)
/// - 1-255 = Reflectivity in dBZ = (value * 0.5) - 32.0
///
/// # Arguments
/// * `block` - The moment block containing REF data
///
/// # Returns
/// * `Vec<f32>` - Reflectivity values in dBZ (0.0 for no data)
pub fn decode_reflectivity(block: &MomentBlock) -> Vec<f32> {
    // Standard NEXRAD: scale = 0.5, offset = -32.0
    decode_moment_data(block, 0.5, -32.0)
}

/// Decode VEL (Velocity) data using standard NEXRAD encoding.
///
/// VEL uses 8-bit encoding where:
/// - 0 = No data
/// - 1-255 = Velocity in m/s = (value * 2.0) - 64.0
///
/// # Arguments
/// * `block` - The moment block containing VEL data
///
/// # Returns
/// * `Vec<f32>` - Velocity values in m/s (0.0 for no data)
pub fn decode_velocity(block: &MomentBlock) -> Vec<f32> {
    // Standard NEXRAD: scale = 2.0, offset = -64.0
    decode_moment_data(block, 2.0, -64.0)
}

/// Decode SW (Spectrum Width) data using standard NEXRAD encoding.
///
/// SW uses 8-bit encoding where:
/// - 0 = No data
/// - 1-255 = Spectrum width in m/s = (value * 0.5)
///
/// # Arguments
/// * `block` - The moment block containing SW data
///
/// # Returns
/// * `Vec<f32>` - Spectrum width values in m/s (0.0 for no data)
pub fn decode_spectrum_width(block: &MomentBlock) -> Vec<f32> {
    // Standard NEXRAD: scale = 0.5, offset = 0.0
    decode_moment_data(block, 0.5, 0.0)
}

#[cfg(test)]
mod radial_data_tests {
    use super::*;

    #[test]
    fn test_parse_radial_data_block_header() {
        // RDAT block header: "RDAT" (4 bytes) + length (4 bytes)
        let bytes: &[u8] = &[
            b'R', b'D', b'A', b'T', // Block type
            0x00, 0x00, 0x01, 0x00, // Block length = 256
        ];

        let result = RadialDataBlock::parse(bytes).unwrap();
        assert_eq!(result.block_type, *b"RDAT");
        assert_eq!(result.block_length, 256);
    }

    #[test]
    fn test_parse_moment_block_ref() {
        // REF moment block: name (3) + reserved (1) + word size (1) + scale (1) + offset (1) + reserved (1) + gates (2) + range (4) + spacing (2) + data
        let mut bytes = vec![0u8; 16 + 10]; // Header (16 bytes) + 10 gates of data
        bytes[0..3].copy_from_slice(b"REF"); // Moment name
        bytes[3] = 0; // Reserved
        bytes[4] = 8; // Data word size = 8 bits per gate
        bytes[5] = 0; // Scale bits (not used directly in this simple parse)
        bytes[6] = 0; // Offset bits
        bytes[7] = 0; // Reserved
        bytes[8] = 0; // Gate count high
        bytes[9] = 10; // Gate count low = 10
                       // Range to first gate (4 bytes) - 5000.0 as f32 = 0x459C4000
        bytes[10] = 0x45;
        bytes[11] = 0x9C;
        bytes[12] = 0x40;
        bytes[13] = 0x00;
        // Gate spacing (2 bytes) - 250 as u16 = 0x00FA
        bytes[14] = 0x00;
        bytes[15] = 0xFA;

        // Add some test data
        for i in 0..10 {
            bytes[16 + i] = (i * 10) as u8;
        }

        let result = MomentBlock::parse(&bytes).unwrap();
        assert_eq!(result.moment_name, *b"REF");
        assert_eq!(result.gate_count, 10);
        assert!((result.range_to_first_gate - 5000.0).abs() < 0.01);
        assert!((result.gate_spacing - 250.0).abs() < 0.01);
        assert_eq!(result.data.len(), 10);
    }

    #[test]
    fn test_decode_moment_data() {
        // Create a test moment block with known data
        let data = vec![64, 128, 192]; // Test values
        let block = MomentBlock {
            moment_name: *b"REF",
            reserved1: 0,
            data_word_size: 8,
            scale_bits: 0,
            offset_bits: 0,
            reserved2: 0,
            gate_count: 3,
            range_to_first_gate: 1000.0,
            gate_spacing: 250.0,
            data,
        };

        // Test with scale=0.5, offset=-32.0 (REF standard)
        let result = decode_moment_data(&block, 0.5, -32.0);
        assert_eq!(result.len(), 3);
        assert!((result[0] - (-0.0)).abs() < 0.01); // 64 * 0.5 - 32 = 0
        assert!((result[1] - 32.0).abs() < 0.01); // 128 * 0.5 - 32 = 32
        assert!((result[2] - 64.0).abs() < 0.01); // 192 * 0.5 - 32 = 64
    }

    #[test]
    fn test_decode_reflectivity() {
        let data = vec![64, 128, 192, 255]; // Test values
        let block = MomentBlock {
            moment_name: *b"REF",
            reserved1: 0,
            data_word_size: 8,
            scale_bits: 0,
            offset_bits: 0,
            reserved2: 0,
            gate_count: 4,
            range_to_first_gate: 1000.0,
            gate_spacing: 250.0,
            data,
        };

        let result = decode_reflectivity(&block);
        assert_eq!(result.len(), 4);
        // 64 * 0.5 - 32 = 0
        assert!((result[0] - 0.0).abs() < 0.01);
        // 255 * 0.5 - 32 = 95.5
        assert!((result[3] - 95.5).abs() < 0.01);
    }

    #[test]
    fn test_decode_velocity() {
        let data = vec![0, 64, 128, 192, 255];
        let block = MomentBlock {
            moment_name: *b"VEL",
            reserved1: 0,
            data_word_size: 8,
            scale_bits: 0,
            offset_bits: 0,
            reserved2: 0,
            gate_count: 5,
            range_to_first_gate: 1000.0,
            gate_spacing: 250.0,
            data,
        };

        let result = decode_velocity(&block);
        assert_eq!(result.len(), 5);
        // 64 * 2.0 - 64 = 64
        assert!((result[1] - 64.0).abs() < 0.01);
        // 128 * 2.0 - 64 = 192
        assert!((result[2] - 192.0).abs() < 0.01);
    }

    #[test]
    fn test_decode_spectrum_width() {
        let data = vec![0, 10, 20, 30];
        let block = MomentBlock {
            moment_name: *b"SW ",
            reserved1: 0,
            data_word_size: 8,
            scale_bits: 0,
            offset_bits: 0,
            reserved2: 0,
            gate_count: 4,
            range_to_first_gate: 1000.0,
            gate_spacing: 250.0,
            data,
        };

        let result = decode_spectrum_width(&block);
        assert_eq!(result.len(), 4);
        // 10 * 0.5 = 5
        assert!((result[1] - 5.0).abs() < 0.01);
        // 30 * 0.5 = 15
        assert!((result[3] - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_radial_header_parse() {
        // Radial header: radial_num (2) + calibration (4) + azimuth (4) + status (1) + elevation (4) + elev_num (1) + data_blocks (1)
        // Actually per ICD: 2 + 4 + 4 + 1 + 4 + 1 + 1 = 17 bytes
        let bytes: &[u8] = &[
            0x00, 0x01, // Radial number = 1
            0x00, 0x00, 0x00, 0x00, // Calibration = 0
            0x43, 0x34, 0x00, 0x00, // Azimuth = 180.0 degrees (0x43340000 = 180.0)
            0x01, // Radial status = 1
            0x43, 0x34, 0x00, 0x00, // Elevation = 180.0 degrees (0x43340000 = 180.0)
            0x01, // Elevation number = 1
            0x03, // 3 data blocks
        ];

        let result = RadialHeader::parse(bytes).unwrap();
        assert_eq!(result.radial_number, 1);
        assert!((result.azimuth - 180.0).abs() < 0.01);
        assert!((result.elevation - 180.0).abs() < 0.01);
        assert_eq!(result.elevation_number, 1);
        assert_eq!(result.data_blocks, 3);
    }

    #[test]
    fn test_radial_header_insufficient_bytes() {
        let bytes: &[u8] = &[0x00, 0x01]; // Only 2 bytes

        let result = RadialHeader::parse(bytes);
        assert!(matches!(
            result,
            Err(DecodeError::InsufficientBytes {
                needed: 12,
                have: 2
            })
        ));
    }

    #[test]
    fn test_parse_moment_block_insufficient_bytes() {
        let bytes: &[u8] = b"REF"; // Only 3 bytes

        let result = MomentBlock::parse(bytes);
        assert!(matches!(
            result,
            Err(DecodeError::InsufficientBytes {
                needed: 16,
                have: 3
            })
        ));
    }

    #[test]
    fn test_radial_data_block_insufficient_bytes() {
        let bytes: &[u8] = b"RDA"; // Only 3 bytes

        let result = RadialDataBlock::parse(bytes);
        assert!(matches!(
            result,
            Err(DecodeError::InsufficientBytes { needed: 8, have: 3 })
        ));
    }
}
