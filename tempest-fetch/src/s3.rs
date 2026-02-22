//! S3 client for fetching NEXRAD Level II data from NOAA's public bucket.
//!
//! This module provides functionality to list and fetch radar scans from the
//! public NOAA NEXRAD Level 2 data bucket on AWS S3.
//!
//! # S3 Bucket Structure
//!
//! The NOAA NEXRAD Level 2 bucket uses the following path structure:
//! ```text
//! /{YEAR}/{MONTH}/{DAY}/{STATION_ID}/{FILENAME}
//! ```
//!
//! Example: `/2024/03/15/KTLX/KTLX20240315_120021`
//!
//! Files follow the naming convention: `{STATION_ID}{YYYYMMDD}_{HHMMSS}`

use std::str::FromStr;

use bytes::Bytes;
use chrono::{Datelike, TimeZone, Utc};
use reqwest::Client;
use tracing::{debug, warn};

use crate::cache::Cache;
use crate::error::FetchError;
use crate::retry::{with_retry, RetryConfig};
use crate::types::ScanMeta;

/// Base URL for the NOAA NEXRAD Level 2 S3 bucket.
const NOAA_NEXRAD_BUCKET: &str = "https://noaa-nexrad-level2.s3.amazonaws.com";

/// S3 client for interacting with the NOAA NEXRAD Level 2 bucket.
///
/// This client uses unsigned requests since the bucket is publicly accessible.
#[derive(Debug, Clone)]
pub struct S3Client {
    /// HTTP client for making requests.
    client: Client,
}

impl S3Client {
    /// Create a new S3Client with default configuration.
    pub fn new() -> Result<Self, FetchError> {
        let client = Client::builder()
            .build()
            .map_err(|e| FetchError::internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client })
    }

    /// List all available volume scans for a given station and date.
    ///
    /// # Arguments
    ///
    /// * `station` - Radar station identifier (e.g., "KTLX")
    /// * `date` - Date of the scans to list
    ///
    /// # Returns
    ///
    /// Returns a vector of `ScanMeta` containing metadata for each available scan,
    /// or an error if the request fails.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use chrono::NaiveDate;
    /// use tempest_fetch::s3::S3Client;
    ///
    /// let client = S3Client::new().unwrap();
    /// let scans = client.list_scans("KTLX", NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()).await;
    /// ```
    pub async fn list_scans(
        &self,
        station: &str,
        date: chrono::NaiveDate,
    ) -> Result<Vec<ScanMeta>, FetchError> {
        let path = format!(
            "/{}/{}/{:02}/{}",
            date.year(),
            date.month(),
            date.day(),
            station.to_uppercase()
        );

        let url = format!("{}{}?list-type=2&delimiter=/", NOAA_NEXRAD_BUCKET, path);

        debug!("Listing scans from S3: {}", url);

        let retry_config = RetryConfig::default();

        let response = with_retry(retry_config, || {
            let url = url.clone();
            let client = self.client.clone();
            async move {
                let resp = client.get(&url).send().await?;

                let status = resp.status();
                if status.as_u16() == 404 {
                    return Err(FetchError::s3_not_found(format!(
                        "No scans found for station {} on {}",
                        station, date
                    )));
                }

                if !status.is_success() {
                    return Err(FetchError::http(format!(
                        "S3 request failed with status {}: {}",
                        status, url
                    )));
                }

                let body = resp.text().await?;
                Ok(body)
            }
        })
        .await?;

        let scans = parse_list_objects_response(&response, station, date)?;
        debug!(
            "Found {} scans for station {} on {}",
            scans.len(),
            station,
            date
        );

        Ok(scans)
    }

    /// Fetch a single scan from S3.
    ///
    /// # Arguments
    ///
    /// * `station` - Radar station identifier (e.g., "KTLX")
    /// * `scan` - Scan metadata containing date and filename
    ///
    /// # Returns
    ///
    /// Returns the raw scan data as `Bytes`, or an error if the fetch fails.
    pub async fn fetch_scan(&self, station: &str, scan: &ScanMeta) -> Result<Bytes, FetchError> {
        let path = format!(
            "/{}/{}/{:02}/{}/{}",
            scan.date.year(),
            scan.date.month(),
            scan.date.day(),
            station.to_uppercase(),
            scan.filename
        );

        let url = format!("{}{}/{}", NOAA_NEXRAD_BUCKET, path, scan.filename);

        debug!("Fetching scan from S3: {}", url);

        let retry_config = RetryConfig::default();

        let response = with_retry(retry_config, || {
            let url = url.clone();
            let client = self.client.clone();
            async move {
                let resp = client.get(&url).send().await?;

                let status = resp.status();
                if status.as_u16() == 404 {
                    return Err(FetchError::s3_not_found(format!(
                        "Scan not found: {}",
                        scan.filename
                    )));
                }

                if !status.is_success() {
                    return Err(FetchError::http(format!(
                        "S3 request failed with status {}: {}",
                        status, url
                    )));
                }

                let bytes = resp.bytes().await?;
                Ok(bytes)
            }
        })
        .await?;

        debug!("Fetched scan: {} ({} bytes)", scan.filename, response.len());

        Ok(response)
    }

    /// Fetch a scan with optional caching.
    ///
    /// If a cache is provided, this function will first check if the scan
    /// is already in the cache. If not, it will fetch from S3 and store
    /// the result in the cache.
    ///
    /// # Arguments
    ///
    /// * `station` - Radar station identifier (e.g., "KTLX")
    /// * `scan` - Scan metadata containing date and filename
    /// * `cache` - Optional reference to a Cache instance
    ///
    /// # Returns
    ///
    /// Returns the raw scan data as `Bytes`, or an error if the fetch fails.
    pub async fn fetch_scan_cached(
        &self,
        station: &str,
        scan: &ScanMeta,
        mut cache: Option<&mut Cache>,
    ) -> Result<Bytes, FetchError> {
        let cache_key = scan.cache_key();

        // Try to get from cache first
        if let Some(ref mut cache) = cache {
            if let Ok(Some(data)) = cache.get(&cache_key).await {
                debug!("Cache hit for scan: {}", scan.filename);
                return Ok(Bytes::from(data));
            }
            debug!("Cache miss for scan: {}", scan.filename);
        }

        // Fetch from S3
        let data = self.fetch_scan(station, scan).await?;

        // Store in cache if available
        if let Some(ref mut cache) = cache {
            if let Err(e) = cache.put(&cache_key, data.to_vec()).await {
                warn!("Failed to cache scan {}: {}", scan.filename, e);
            } else {
                debug!("Cached scan: {}", scan.filename);
            }
        }

        Ok(data)
    }
}

impl Default for S3Client {
    fn default() -> Self {
        Self::new().expect("Failed to create default S3Client")
    }
}

/// Parse the XML response from S3 list_objects_v2.
///
/// The response contains CommonPrefixes elements for each "directory" (scan file).
/// We extract the filename from each prefix.
fn parse_list_objects_response(
    xml: &str,
    station: &str,
    date: chrono::NaiveDate,
) -> Result<Vec<ScanMeta>, FetchError> {
    let mut scans = Vec::new();

    // Find all <CommonPrefixes> elements
    let prefix_start = "<CommonPrefixes>";
    let prefix_end = "</CommonPrefixes>";
    let prefix_content_start = "<Prefix>";

    let mut remaining = xml;

    while let Some(prefix_section_start) = remaining.find(prefix_start) {
        // Skip to the start of CommonPrefixes
        let section_start = prefix_section_start + prefix_start.len();
        remaining = &remaining[section_start..];

        // Find the end of CommonPrefixes
        if let Some(prefix_section_end) = remaining.find(prefix_end) {
            let prefix_section = &remaining[..prefix_section_end];

            // Extract the Prefix content
            if let Some(content_start) = prefix_section.find(prefix_content_start) {
                let content = &prefix_section[content_start + prefix_content_start.len()..];

                // Find the closing </Prefix> tag
                if let Some(content_end) = content.find("</Prefix>") {
                    let prefix = &content[..content_end];

                    // Extract filename from the prefix
                    // Format: /2024/03/15/KTLX/KTLX20240315_120021
                    if let Some(filename) = prefix.rsplit('/').next() {
                        if !filename.is_empty() {
                            // Parse the timestamp from filename
                            // Format: {STATION_ID}{YYYYMMDD}_{HHMMSS}
                            if let Some(scan_meta) = parse_scan_filename(station, date, filename) {
                                scans.push(scan_meta);
                            }
                        }
                    }
                }
            }

            remaining = &remaining[prefix_section_end + prefix_end.len()..];
        } else {
            break;
        }
    }

    if scans.is_empty() {
        // Try an alternative parsing approach - look for Key elements
        return parse_list_objects_response_via_keys(xml, station, date);
    }

    Ok(scans)
}

/// Alternative parsing using Key elements in case CommonPrefixes approach doesn't work.
fn parse_list_objects_response_via_keys(
    xml: &str,
    station: &str,
    date: chrono::NaiveDate,
) -> Result<Vec<ScanMeta>, FetchError> {
    let mut scans = Vec::new();

    // Find all <Key> elements
    let key_start = "<Key>";
    let key_end = "</Key>";

    let mut remaining = xml;

    while let Some(key_section_start) = remaining.find(key_start) {
        let content_start = key_section_start + key_start.len();
        remaining = &remaining[content_start..];

        if let Some(key_section_end) = remaining.find(key_end) {
            let key = &remaining[..key_section_end];

            // Extract filename from key
            if let Some(filename) = key.rsplit('/').next() {
                if !filename.is_empty() && filename.contains(station.to_uppercase().as_str()) {
                    if let Some(scan_meta) = parse_scan_filename(station, date, filename) {
                        scans.push(scan_meta);
                    }
                }
            }

            remaining = &remaining[key_section_end + key_end.len()..];
        } else {
            break;
        }
    }

    Ok(scans)
}

/// Parse a scan filename to extract metadata.
///
/// Filename format: `{STATION_ID}{YYYYMMDD}_{HHMMSS}`
/// Example: `KTLX20240315_120021`
fn parse_scan_filename(
    station: &str,
    _date: chrono::NaiveDate,
    filename: &str,
) -> Option<ScanMeta> {
    // Filename format: {STATION_ID}{YYYYMMDD}_{HHMMSS}
    // Example: KTLX20240315_120021

    // Station ID is 4 characters starting with K
    let station_upper = station.to_uppercase();
    let expected_prefix = station_upper.as_str();

    if !filename.starts_with(expected_prefix) {
        return None;
    }

    // Extract the timestamp portion after station ID
    let timestamp_part = &filename[expected_prefix.len()..];

    // Format is YYYYMMDD_HHMMSS (15 characters)
    if timestamp_part.len() < 15 {
        warn!("Filename too short to parse timestamp: {}", filename);
        return None;
    }

    let date_part = &timestamp_part[..8];
    let time_part = &timestamp_part[9..15]; // Skip the underscore

    // Parse date
    let year = i32::from_str(&date_part[0..4]).ok()?;
    let month = u32::from_str(&date_part[4..6]).ok()?;
    let day = u32::from_str(&date_part[6..8]).ok()?;

    // Parse time
    let hour = u32::from_str(&time_part[0..2]).ok()?;
    let minute = u32::from_str(&time_part[2..4]).ok()?;
    let second = u32::from_str(&time_part[4..6]).ok()?;

    // Create DateTime in UTC
    let datetime = Utc
        .with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()?;

    Some(ScanMeta {
        station: station_upper.clone(),
        date: datetime,
        filename: filename.to_string(),
        size: 0, // Size not available from listing
        timestamp: datetime,
    })
}

/// List all available scans for a station on a given date.
///
/// This is a convenience function that creates a temporary S3Client
/// and lists the scans.
///
/// # Arguments
///
/// * `station` - Radar station identifier (e.g., "KTLX")
/// * `date` - Date of the scans to list
///
/// # Returns
///
/// Returns a vector of `ScanMeta` containing metadata for each available scan.
pub async fn list_scans(
    station: &str,
    date: chrono::NaiveDate,
) -> Result<Vec<ScanMeta>, FetchError> {
    let client = S3Client::new()?;
    client.list_scans(station, date).await
}

/// Fetch a single scan from S3.
///
/// This is a convenience function that creates a temporary S3Client
/// and fetches the scan data.
///
/// # Arguments
///
/// * `station` - Radar station identifier (e.g., "KTLX")
/// * `scan` - Scan metadata containing date and filename
///
/// # Returns
///
/// Returns the raw scan data as `Bytes`, or an error if the fetch fails.
///
/// # Example
///
/// ```ignore
/// use chrono::NaiveDate;
/// use tempest_fetch::{list_scans, fetch_scan};
///
/// let scans = list_scans("KTLX", NaiveDate::from_ymd_opt(2024, 3, 15).unwrap()).await?;
/// if let Some(scan) = scans.first() {
///     let data = fetch_scan("KTLX", &scan).await?;
/// }
/// ```
pub async fn fetch_scan(station: &str, scan: &ScanMeta) -> Result<Bytes, FetchError> {
    let client = S3Client::new()?;
    client.fetch_scan(station, scan).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn test_parse_scan_filename_valid() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let result = parse_scan_filename("KTLX", date, "KTLX20240315_120021");

        assert!(result.is_some());
        let scan = result.unwrap();
        assert_eq!(scan.station, "KTLX");
        assert_eq!(scan.filename, "KTLX20240315_120021");
        assert_eq!(scan.date.hour(), 12);
        assert_eq!(scan.date.minute(), 0);
        assert_eq!(scan.date.second(), 21);
    }

    #[test]
    fn test_parse_scan_filename_lowercase() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let result = parse_scan_filename("ktlx", date, "KTLX20240315_120021");

        assert!(result.is_some());
        let scan = result.unwrap();
        assert_eq!(scan.station, "KTLX");
    }

    #[test]
    fn test_parse_scan_filename_wrong_station() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let result = parse_scan_filename("KTLX", date, "KATX20240315_120021");

        assert!(result.is_none());
    }

    #[test]
    fn test_parse_scan_filename_invalid() {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let result = parse_scan_filename("KTLX", date, "invalid");

        assert!(result.is_none());
    }

    #[test]
    fn test_s3_client_default() {
        let _client = S3Client::default();
    }

    #[tokio::test]
    async fn test_list_scans_creates_client() {
        // This test just verifies the function can be called
        // The actual network request would fail in tests without proper setup
        let result = list_scans(
            "KTLX",
            chrono::NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
        )
        .await;
        // We expect either success or an error, but not a panic
        // Note: This will make a real network call
        if let Err(e) = &result {
            println!("Expected error for network call: {}", e);
        }
    }
}
