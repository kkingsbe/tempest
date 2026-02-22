//! Decompression utilities for NEXRAD weather radar data.
//!
//! This module provides decompression functions for various compression formats
//! used in NEXRAD Level II data files.

use crate::error::FetchError;
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use std::io::Read;

/// Decompress bzip2 compressed data.
///
/// This function takes a slice of bytes containing bzip2 compressed data
/// and returns the decompressed data as a new vector of bytes.
///
/// # Arguments
///
/// * `data` - A slice of bytes containing bzip2 compressed data
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The decompressed data
/// * `Err(FetchError)` - If decompression fails
///
/// # Example
///
/// ```rust
/// use tempest_fetch::decompress_bz2;
///
/// // Decompress bzip2 data (compressed using bzip2 crate)
/// // let decompressed = decompress_bz2(&compressed_data).unwrap();
/// ```
pub fn decompress_bz2(data: &[u8]) -> Result<Vec<u8>, FetchError> {
    let mut decoder = BzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| FetchError::io(format!("bzip2 decompression failed: {}", e)))?;
    Ok(decompressed)
}

/// Decompress gzip compressed data.
///
/// This function takes a slice of bytes containing gzip compressed data
/// and returns the decompressed data as a new vector of bytes.
///
/// # Arguments
///
/// * `data` - A slice of bytes containing gzip compressed data
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The decompressed data
/// * `Err(FetchError)` - If decompression fails
///
/// # Example
///
/// ```rust
/// use tempest_fetch::decompress_gzip;
///
/// // Decompress gzip data (compressed using gzip/flate2 crate)
/// // let decompressed = decompress_gzip(&compressed_data).unwrap();
/// ```
pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>, FetchError> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| FetchError::io(format!("gzip decompression failed: {}", e)))?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bzip2::write::BzEncoder;
    use bzip2::Compression;
    use flate2::write::GzEncoder;
    use flate2::Compression as Flate2Compression;
    use std::io::Write;

    #[test]
    fn test_decompress_bz2_valid_data() {
        // Create some test data
        let original = b"NEXRAD Level II Data - Test message for decompression";

        // Compress it using bzip2
        let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        // Decompress and verify
        let decompressed = decompress_bz2(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_decompress_bz2_empty_data() {
        // Compress empty data
        let original = b"";
        let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        // Decompress and verify
        let decompressed = decompress_bz2(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_decompress_bz2_invalid_data() {
        // Invalid bzip2 data
        let invalid_data = b"This is not valid bzip2 data!!!";

        let result = decompress_bz2(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompress_bz2_short_data() {
        // Too short to be valid bzip2
        let short_data = b"ab";

        let result = decompress_bz2(short_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompress_bz2_binary_data() {
        // Test with binary data (simulating compressed NEXRAD data structure)
        let original: Vec<u8> = (0..255).cycle().take(10000).collect();

        let mut encoder = BzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&original).unwrap();
        let compressed = encoder.finish().unwrap();

        let decompressed = decompress_bz2(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    // Gzip decompression tests

    #[test]
    fn test_decompress_gzip_valid_data() {
        // Create some test data
        let original = b"NEXRAD Level II Data - Test message for gzip decompression";

        // Compress it using gzip
        let mut encoder = GzEncoder::new(Vec::new(), Flate2Compression::default());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        // Decompress and verify
        let decompressed = decompress_gzip(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_decompress_gzip_empty_data() {
        // Compress empty data
        let original = b"";
        let mut encoder = GzEncoder::new(Vec::new(), Flate2Compression::default());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        // Decompress and verify
        let decompressed = decompress_gzip(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_decompress_gzip_invalid_data() {
        // Invalid gzip data
        let invalid_data = b"This is not valid gzip data!!!";

        let result = decompress_gzip(invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompress_gzip_short_data() {
        // Too short to be valid gzip
        let short_data = b"ab";

        let result = decompress_gzip(short_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_decompress_gzip_binary_data() {
        // Test with binary data (simulating compressed NEXRAD data structure)
        let original: Vec<u8> = (0..255).cycle().take(10000).collect();

        let mut encoder = GzEncoder::new(Vec::new(), Flate2Compression::default());
        encoder.write_all(&original).unwrap();
        let compressed = encoder.finish().unwrap();

        let decompressed = decompress_gzip(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }
}
