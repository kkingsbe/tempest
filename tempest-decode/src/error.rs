//! Error types for NEXRAD data decoding.

use thiserror::Error;

/// Errors that can occur during NEXRAD message decoding.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Not enough bytes available to decode the required data.
    #[error("Insufficient bytes: need {needed}, have {have}")]
    InsufficientBytes { needed: usize, have: usize },

    /// Input data was truncated or incomplete
    #[error("Input data is truncated: {0}")]
    Truncated(String),

    /// Invalid magic bytes in the archive file
    #[error("Invalid magic bytes: expected {expected}, found {found}")]
    InvalidMagic { expected: String, found: String },

    /// Unsupported message type or version
    #[error("Unsupported version or message type: {0}")]
    UnsupportedVersion(String),

    /// Required data moment is missing
    #[error("Missing required moment: {0}")]
    MissingMoment(String),

    /// Invalid message type encountered.
    #[error("Invalid message type: expected 31, got {0}")]
    InvalidMessageType(u16),

    /// Invalid station identifier (ICAO code).
    #[error("Invalid station ID: {0}")]
    InvalidStationId(#[from] std::str::Utf8Error),

    /// I/O error during decompression
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insufficient_bytes_error() {
        let err = DecodeError::InsufficientBytes {
            needed: 16,
            have: 8,
        };
        assert_eq!(err.to_string(), "Insufficient bytes: need 16, have 8");
    }

    #[test]
    fn test_invalid_message_type_error() {
        let err = DecodeError::InvalidMessageType(5);
        assert_eq!(err.to_string(), "Invalid message type: expected 31, got 5");
    }
}
