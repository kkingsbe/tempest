//! Mock S3 server for testing NEXRAD data fetching.
//!
//! This module provides a mock S3 server implementation using the `mockito` crate
//! to simulate AWS S3 responses for testing the fetch pipeline without making
//! actual network requests.
//!
//! # Example
//!
//! ```ignore
//! use mockito::Server;
//! use tempest_fetch::mock_s3::MockS3Server;
//!
//! async {
//!     let mock_server = MockS3Server::new().await;
//!     
//!     // Register station list response
//!     mock_server.register_station_list();
//!     
//!     // Register scan data response
//!     mock_server.register_scan_data("KTLX", b"mock scan data");
//!     
//!     // Use with S3Client
//!     let client = S3Client::with_base_url(mock_server.url()).unwrap();
//! }
//! ```

use bytes::Bytes;
use chrono::{TimeZone, Utc};
use mockito::{Matcher, Server as MockServer, ServerGuard};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::types::ScanMeta;

/// A mock S3 server for testing NEXRAD data fetching.
///
/// This struct wraps a `mockito::Server` and provides convenience methods
/// to register mock responses for various NEXRAD bucket paths.
pub struct MockS3Server {
    /// The underlying mockito server.
    server: ServerGuard,
    /// Storage for registered scan data (key: filename, value: data).
    scan_data: Mutex<HashMap<String, Vec<u8>>>,
}

impl MockS3Server {
    /// Create a new mock S3 server.
    ///
    /// # Returns
    ///
    /// Returns a new `MockS3Server` instance bound to a random available port.
    pub async fn new() -> Result<Self, crate::FetchError> {
        let server = MockServer::new_async().await;
        
        Ok(Self {
            server,
            scan_data: Mutex::new(HashMap::new()),
        })
    }

    /// Get the base URL of the mock server.
    ///
    /// # Returns
    ///
    /// Returns the URL string that can be used to connect to the mock server.
    #[must_use]
    pub fn url(&self) -> String {
        self.server.url()
    }

    /// Register a mock response for listing scans for a station on a date.
    ///
    /// This registers a mock S3 ListObjectsV2 response containing the given
    /// scan filenames.
    ///
    /// # Arguments
    ///
    /// * `station` - The station ID (e.g., "KTLX")
    /// * `date` - The date in YYYY/MM/DD format as separate components
    /// * `scans` - Vector of scan filenames to include in the response
    pub fn register_list_scans_response(
        &self,
        station: &str,
        year: i32,
        month: u32,
        day: u32,
        scans: &[&str],
    ) {
        // Build XML response simulating S3 ListObjectsV2
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
"#,
        );

        for scan in scans {
            let prefix = format!("/{}/{}/{:02}/{}/{}", year, month, day, station, scan);
            xml.push_str(&format!(
                "  <CommonPrefixes>\n    <Prefix>{}</Prefix>\n  </CommonPrefixes>\n",
                prefix
            ));
        }

        xml.push_str("</ListBucketResult>");

        let path = format!("/{}/{}/{:02}/{}?list-type=2&delimiter=/", year, month, day, station);

        self.server
            .get(&path)
            .match_header("Accept", "application/xml")
            .with_status(200)
            .with_header("Content-Type", "application/xml")
            .with_body(xml)
            .create();
    }

    /// Register mock scan data for download.
    ///
    /// This stores the data internally and configures the mock server to
    /// return it when the corresponding path is requested.
    ///
    /// # Arguments
    ///
    /// * `station` - The station ID (e.g., "KTLX")
    /// * `date` - The date as separate components
    /// * `filename` - The scan filename
    /// * `data` - The raw scan data to return
    pub fn register_scan_data(
        &self,
        station: &str,
        year: i32,
        month: u32,
        day: u32,
        filename: &str,
        data: Vec<u8>,
    ) {
        let path = format!("/{}/{}/{:02}/{}/{}", year, month, day, station, filename);
        
        // Store for later reference
        {
            let mut scans = self.scan_data.lock().unwrap();
            scans.insert(filename.to_string(), data.clone());
        }

        self.server
            .get(&path)
            .with_status(200)
            .with_header("Content-Type", "application/octet-stream")
            .with_header("Content-Length", &data.len().to_string())
            .with_body(data)
            .create();
    }

    /// Register mock scan data with pre-compressed (bzip2) content.
    ///
    /// # Arguments
    ///
    /// * `station` - The station ID (e.g., "KTLX")
    /// * `date` - The date as separate components
    /// * `filename` - The scan filename (should end with .bz2)
    /// * `compressed_data` - The bzip2 compressed scan data
    pub fn register_compressed_scan_data(
        &self,
        station: &str,
        year: i32,
        month: u32,
        day: u32,
        filename: &str,
        compressed_data: Vec<u8>,
    ) {
        let path = format!("/{}/{}/{:02}/{}/{}", year, month, day, station, filename);

        self.server
            .get(&path)
            .with_status(200)
            .with_header("Content-Type", "application/x-bzip2")
            .with_header("Content-Length", &compressed_data.len().to_string())
            .with_body(compressed_data)
            .create();
    }

    /// Register a standard station list response for testing.
    ///
    /// This sets up common NEXRAD stations with sample data that can be
    /// used in E2E tests.
    pub fn register_standard_stations(&self) {
        // Register KTLX (Oklahoma City) scans
        self.register_list_scans_response(
            "KTLX",
            2024,
            3,
            15,
            &["KTLX20240315_120021", "KTLX20240315_120521"],
        );

        // Register KICT (Wichita) scans
        self.register_list_scans_response(
            "KICT",
            2024,
            3,
            15,
            &["KICT20240315_110000"],
        );
    }

    /// Create a ScanMeta object for testing.
    ///
    /// # Arguments
    ///
    /// * `station` - Station ID
    /// * `filename` - Scan filename
    ///
    /// # Returns
    ///
    /// Returns a ScanMeta with the current timestamp.
    pub fn create_scan_meta(&self, station: &str, filename: &str) -> ScanMeta {
        let now = Utc::now();
        ScanMeta::new(station, now, filename, 1024, now)
    }

    /// Get the stored scan data by filename.
    ///
    /// # Arguments
    ///
    /// * `filename` - The scan filename to look up
    ///
    /// # Returns
    ///
    /// Returns the stored data if found.
    pub fn get_scan_data(&self, filename: &str) -> Option<Bytes> {
        let scans = self.scan_data.lock().unwrap();
        scans.get(filename).map(|data| Bytes::from(data.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_server_new() {
        let server = MockS3Server::new().await;
        assert!(server.is_ok());
        let server = server.unwrap();
        // Should have a valid URL
        assert!(!server.url().is_empty());
        assert!(server.url().starts_with("http://"));
    }

    #[tokio::test]
    async fn test_register_list_scans() {
        let server = MockS3Server::new().await.unwrap();
        
        server.register_list_scans_response(
            "KTLX",
            2024,
            3,
            15,
            &["KTLX20240315_120021", "KTLX20240315_120521"],
        );

        // Verify URL is accessible
        let url = server.url();
        assert!(url.contains("mockito"));
    }

    #[tokio::test]
    async fn test_register_scan_data() {
        let server = MockS3Server::new().await.unwrap();
        
        let test_data = b"NEXRAD Level II Test Data";
        
        server.register_scan_data(
            "KTLX",
            2024,
            3,
            15,
            "KTLX20240315_120021",
            test_data.to_vec(),
        );

        // Verify data is stored
        let stored = server.get_scan_data("KTLX20240315_120021");
        assert!(stored.is_some());
        assert_eq!(stored.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_scan_meta_creation() {
        let server = MockS3Server::new().await.unwrap();
        
        let meta = server.create_scan_meta("KTLX", "KTLX20240315_120021");
        
        assert_eq!(meta.station, "KTLX");
        assert_eq!(meta.filename, "KTLX20240315_120021");
    }
}
