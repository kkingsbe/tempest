//! Message Type 1 (Message Header Segment) parsing.
//!
//! This module provides structures and parsing for NEXRAD Message Type 1,
//! which is the "Message Header Segment" used in Level II data.

use crate::error::DecodeError;
use crate::types::{Milliseconds, Mjd, StationId};

/// Message Type 1 header (Message Header Segment).
///
/// According to NEXRAD ICD:
/// - ID: 2 bytes (should be 1 for Message Header Segment)
/// - Date: 2 bytes (Modified Julian Date)
/// - Time: 4 bytes (milliseconds since midnight UTC)
/// - Station ID: 4 bytes (ICAO station identifier)
/// - ICS: 1 byte (Industrial/Commercial/Standard indicator)
/// - East-West Velocity: 2 bytes (signed, 0.5 m/s resolution)
/// - South-North Velocity: 2 bytes (signed, 0.5 m/s resolution)
/// - Elevation: 2 bytes (0.5° resolution)
#[derive(Debug, Clone, PartialEq)]
pub struct Msg1Header {
    /// Message type identifier (should be 1 for Message Header Segment)
    pub id: u16,
    /// Date in Modified Julian Date format (days since May 23, 1968)
    pub date: Mjd,
    /// Time in milliseconds since midnight UTC
    pub time: Milliseconds,
    /// Radar station ICAO identifier
    pub station_id: StationId,
    /// ICS field (Industrial/Commercial/Standard indicator)
    pub ics: u8,
    /// East-West velocity component (0.5 m/s resolution)
    pub east_west: i16,
    /// South-North velocity component (0.5 m/s resolution)
    pub south_north: i16,
    /// Elevation angle in degrees (0.5° resolution)
    pub elevation: f32,
}

impl Msg1Header {
    /// Required minimum bytes for the header (after message size is consumed).
    pub const REQUIRED_BYTES: usize = 19;

    /// Parses the Message Type 1 header from raw bytes.
    ///
    /// The input bytes should NOT include the message size (first 4 bytes of the message).
    /// This method expects exactly 18 bytes: ID (2) + Date (2) + Time (4) + Station ID (4)
    /// + ICS (1) + East-West (2) + South-North (2) + Elevation (2).
    ///
    /// # Arguments
    /// * `bytes` - Raw bytes starting from the message ID (after message size)
    ///
    /// # Returns
    /// * `Ok(Msg1Header)` - Successfully parsed header
    /// * `Err(DecodeError)` - If parsing fails (insufficient bytes or invalid data)
    ///
    /// # Example
    /// ```ignore
    /// use tempest_decode::Msg1Header;
    ///
    /// let bytes: &[u8] = &[
    ///     0x00, 0x01,  // ID = 1
    ///     0xA2, 0x1B,  // Date (MJD)
    ///     0x00, 0x1C, 0x9C, 0x40, // Time in ms
    ///     b'K', b'T', b'L', b'X', // Station ID
    ///     0x00,       // ICS
    ///     0x00, 0x00, // East-West velocity
    ///     0x00, 0x00, // South-North velocity
    ///     0x00, 0x64, // Elevation (25.0 degrees)
    /// ];
    ///
    /// let header = Msg1Header::parse(bytes).unwrap();
    /// assert_eq!(header.id, 1);
    /// assert_eq!(header.station_identifier().unwrap(), "KTLX");
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

        // Validate message type is 1
        if id != 1 {
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

        // Parse ICS field (u8)
        let ics = bytes[12];

        // Parse East-West velocity (i16, big-endian, 0.5 m/s resolution)
        let east_west = i16::from_be_bytes([bytes[13], bytes[14]]);

        // Parse South-North velocity (i16, big-endian, 0.5 m/s resolution)
        let south_north = i16::from_be_bytes([bytes[15], bytes[16]]);

        // Parse elevation (u16, big-endian, 0.5 degree resolution)
        let elevation_raw = u16::from_be_bytes([bytes[17], bytes[18]]);
        let elevation = (elevation_raw as f32) * 0.5;

        Ok(Self {
            id,
            date,
            time,
            station_id,
            ics,
            east_west,
            south_north,
            elevation,
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

    /// Returns the East-West velocity in m/s.
    ///
    /// The stored value has 0.5 m/s resolution.
    #[inline]
    pub fn east_west_ms(&self) -> f32 {
        self.east_west as f32 * 0.5
    }

    /// Returns the South-North velocity in m/s.
    ///
    /// The stored value has 0.5 m/s resolution.
    #[inline]
    pub fn south_north_ms(&self) -> f32 {
        self.south_north as f32 * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_msg1_header() {
        // Sample bytes for KTLX station
        // ID = 1 (0x0001), Date = some MJD, Time = 1850176 ms
        let bytes: &[u8] = &[
            0x00, 0x01, // ID = 1
            0xA2, 0x1B, // Date (MJD)
            0x00, 0x1C, 0x9C, 0x40, // Time in ms
            b'K', b'T', b'L', b'X', // Station ID
            0x00, // ICS = 0 (Standard)
            0x00, 0x00, // East-West velocity = 0
            0x00, 0x00, // South-North velocity = 0
            0x00, 0x32, // Elevation = 50 * 0.5 = 25.0 degrees
        ];

        let header = Msg1Header::parse(bytes).unwrap();
        assert_eq!(header.id, 1);
        assert_eq!(header.date, 0xA21B); // MJD big-endian = 41499
        assert_eq!(header.time, 0x001C9C40); // 1850176 ms in big-endian
        assert_eq!(header.station_id.as_str().unwrap(), "KTLX");
        assert_eq!(header.ics, 0);
        assert_eq!(header.east_west, 0);
        assert_eq!(header.south_north, 0);
        assert_eq!(header.elevation, 25.0);
    }

    #[test]
    fn test_parse_msg1_header_kokc() {
        let bytes: &[u8] = &[
            0x00, 0x01, // ID = 1
            0xB0, 0x1C, // Date (MJD)
            0x80, 0x96, 0x98, 0x00, // Time in ms
            b'K', b'O', b'K', b'C', // Station ID
            0x01, // ICS = 1 (Industrial)
            0x00, 0x10, // East-West velocity = 16 * 0.5 = 8 m/s
            0xFF, 0xF0, // South-North velocity = -16 * 0.5 = -8 m/s
            0x00, 0x0A, // Elevation = 10 * 0.5 = 5.0 degrees
        ];

        let header = Msg1Header::parse(bytes).unwrap();
        assert_eq!(header.id, 1);
        assert_eq!(header.station_id.as_str().unwrap(), "KOKC");
        assert_eq!(header.ics, 1);
        assert_eq!(header.east_west, 16);
        assert_eq!(header.south_north, -16);
        assert_eq!(header.elevation, 5.0);
    }

    #[test]
    fn test_parse_insufficient_bytes() {
        let bytes: &[u8] = &[0x00, 0x01]; // Only 2 bytes

        let result = Msg1Header::parse(bytes);
        assert!(matches!(
            result,
            Err(DecodeError::InsufficientBytes {
                needed: 19,
                have: 2
            })
        ));
    }

    #[test]
    fn test_parse_invalid_message_type() {
        let bytes: &[u8] = &[
            0x00, 0x1F, // ID = 31 (not 1)
            0x00, 0x00, // Date
            0x00, 0x00, 0x00, 0x00, // Time
            b'K', b'T', b'L', b'X', // Station ID
            0x00, // ICS
            0x00, 0x00, // East-West
            0x00, 0x00, // South-North
            0x00, 0x00, // Elevation
        ];

        let result = Msg1Header::parse(bytes);
        assert!(matches!(result, Err(DecodeError::InvalidMessageType(31))));
    }

    #[test]
    fn test_parse_invalid_message_type_5() {
        let bytes: &[u8] = &[
            0x00, 0x05, // ID = 5 (not 1)
            0x00, 0x00, // Date
            0x00, 0x00, 0x00, 0x00, // Time
            b'K', b'T', b'L', b'X', // Station ID
            0x00, // ICS
            0x00, 0x00, // East-West
            0x00, 0x00, // South-North
            0x00, 0x00, // Elevation
        ];

        let result = Msg1Header::parse(bytes);
        assert!(matches!(result, Err(DecodeError::InvalidMessageType(5))));
    }

    #[test]
    fn test_ics_field_parsing() {
        // Test different ICS values
        let bytes: &[u8] = &[
            0x00, 0x01, // ID = 1
            0x00, 0x00, // Date
            0x00, 0x00, 0x00, 0x00, // Time
            b'K', b'T', b'L', b'X', // Station ID
            0x02, // ICS = 2 (Commercial)
            0x00, 0x00, // East-West
            0x00, 0x00, // South-North
            0x00, 0x00, // Elevation
        ];

        let header = Msg1Header::parse(bytes).unwrap();
        assert_eq!(header.ics, 2);
    }

    #[test]
    fn test_velocity_field_parsing() {
        // Test positive and negative velocity values
        let bytes: &[u8] = &[
            0x00, 0x01, // ID = 1
            0x00, 0x00, // Date
            0x00, 0x00, 0x00, 0x00, // Time
            b'K', b'T', b'L', b'X', // Station ID
            0x00, // ICS
            0x00, 0x14, // East-West = 20 (10 m/s with 0.5 resolution)
            0xFF, 0xEC, // South-North = -20 (-10 m/s with 0.5 resolution)
            0x00, 0x00, // Elevation
        ];

        let header = Msg1Header::parse(bytes).unwrap();
        assert_eq!(header.east_west, 20);
        assert_eq!(header.south_north, -20);
        assert_eq!(header.east_west_ms(), 10.0);
        assert_eq!(header.south_north_ms(), -10.0);
    }

    #[test]
    fn test_elevation_parsing() {
        // Test elevation field with various values
        let bytes: &[u8] = &[
            0x00, 0x01, // ID = 1
            0x00, 0x00, // Date
            0x00, 0x00, 0x00, 0x00, // Time
            b'K', b'T', b'L', b'X', // Station ID
            0x00, // ICS
            0x00, 0x00, // East-West
            0x00, 0x00, // South-North
            0x01, 0xF4, // Elevation = 500 * 0.5 = 250.0 degrees
        ];

        let header = Msg1Header::parse(bytes).unwrap();
        // Raw value 500 -> 250.0 degrees
        assert_eq!(header.elevation, 250.0);
    }

    #[test]
    fn test_time_hmsms() {
        // 12:00:00.000 = 12 * 3600000 = 43200000 ms
        let header = Msg1Header {
            id: 1,
            date: 20500,
            time: 43_200_000,
            station_id: StationId::new(*b"KTLX"),
            ics: 0,
            east_west: 0,
            south_north: 0,
            elevation: 0.0,
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
        let header = Msg1Header {
            id: 1,
            date: 20500,
            time: 52_245_123,
            station_id: StationId::new(*b"KTLX"),
            ics: 0,
            east_west: 0,
            south_north: 0,
            elevation: 0.0,
        };

        let (hours, minutes, seconds, milliseconds) = header.time_hmsms();
        assert_eq!(hours, 14);
        assert_eq!(minutes, 30);
        assert_eq!(seconds, 45);
        assert_eq!(milliseconds, 123);
    }

    #[test]
    fn test_station_identifier() {
        let header = Msg1Header {
            id: 1,
            date: 20500,
            time: 0,
            station_id: StationId::new(*b"KTLX"),
            ics: 0,
            east_west: 0,
            south_north: 0,
            elevation: 0.0,
        };

        assert_eq!(header.station_identifier().unwrap(), "KTLX");
    }

    #[test]
    fn test_velocity_conversion_methods() {
        let header = Msg1Header {
            id: 1,
            date: 20500,
            time: 0,
            station_id: StationId::new(*b"KTLX"),
            ics: 0,
            east_west: 10,   // 5 m/s
            south_north: -6, // -3 m/s
            elevation: 0.0,
        };

        assert_eq!(header.east_west_ms(), 5.0);
        assert_eq!(header.south_north_ms(), -3.0);
    }
}
