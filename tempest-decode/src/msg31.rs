//! Message Type 31 (Digital Radar Data) parsing.
//!
//! This module provides structures and parsing for NEXRAD Message Type 31,
//! which is the "Digital Radar Data" message used in Archive2 format.

use crate::error::DecodeError;
use crate::types::{Milliseconds, Mjd, StationId};

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
