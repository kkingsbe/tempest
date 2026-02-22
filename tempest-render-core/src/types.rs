//! Core types for radar data projection.

/// Geographic coordinates in degrees.
///
/// Represents a point on Earth's surface using latitude and longitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLng {
    /// Latitude in degrees (-90 to 90)
    pub lat: f64,
    /// Longitude in degrees (-180 to 180)
    pub lng: f64,
}

impl LatLng {
    /// Create a new LatLng coordinate.
    #[inline]
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }
}

impl Default for LatLng {
    fn default() -> Self {
        Self { lat: 0.0, lng: 0.0 }
    }
}

/// Radar station location and metadata.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadarSite {
    /// ICAO station identifier (e.g., "KTLX")
    pub id: &'static str,
    /// Latitude in degrees (positive = north)
    pub lat: f64,
    /// Longitude in degrees (negative = west)
    pub lon: f64,
    /// Antenna elevation above sea level in meters
    pub elevation_m: f64,
}

impl RadarSite {
    /// Create a new RadarSite with the given parameters.
    ///
    /// # Arguments
    /// * `id` - ICAO station identifier (4 characters)
    /// * `lat` - Latitude in degrees
    /// * `lon` - Longitude in degrees
    /// * `elevation_m` - Antenna elevation in meters
    #[inline]
    pub const fn new(id: &'static str, lat: f64, lon: f64, elevation_m: f64) -> Self {
        Self {
            id,
            lat,
            lon,
            elevation_m,
        }
    }
}

/// Registry of major NEXRAD weather radar stations.
///
/// Contains accurate geographic coordinates and antenna elevations
/// for key NEXRAD sites across the United States.
pub const STATIONS: &[RadarSite] = &[
    // KTLX - Oklahoma City (Twin Lakes)
    RadarSite::new("KTLX", 35.4183, -97.4514, 374.0),
    // KINX - Tulsa, OK
    RadarSite::new("KINX", 36.1764, -95.5603, 268.0),
    // KFWS - Dallas/Fort Worth, TX
    RadarSite::new("KFWS", 32.5739, -97.3028, 175.0),
    // KHGX - Houston, TX
    RadarSite::new("KHGX", 29.4719, -95.0794, 14.0),
    // KMXX - Maxwell, CA
    RadarSite::new("KMXX", 39.6194, -121.6469, 596.0),
    // KFSD - Sioux Falls, SD
    RadarSite::new("KFSD", 43.5878, -96.8294, 450.0),
    // KBUF - Buffalo, NY
    RadarSite::new("KBUF", 42.9489, -78.7369, 207.0),
    // KMPX - Minneapolis/Twin Cities, MN
    RadarSite::new("KMPX", 44.8528, -93.5650, 287.0),
    // KSRX - Little Rock, AR
    RadarSite::new("KSRX", 35.2403, -92.5267, 140.0),
    // KVWX - Vicksburg, MS (Jackson)
    RadarSite::new("KVWX", 32.1658, -90.8656, 91.0),
];

/// Looks up a radar station by its ICAO identifier.
///
/// The lookup is case-insensitive, so "ktlx", "KTLX", and "kTlX"
/// will all return the same result.
///
/// # Arguments
///
/// * `id` - The station ICAO identifier (e.g., "KTLX")
///
/// # Returns
///
/// * `Some(&RadarSite)` if the station is found in the registry
/// * `None` if the station is not found
#[inline]
pub fn get_station(id: &str) -> Option<&RadarSite> {
    let id_uppercase = id.to_uppercase();
    STATIONS.iter().find(|station| station.id == id_uppercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lat_lng_creation() {
        let coord = LatLng::new(35.0, -97.0);
        assert!((coord.lat - 35.0).abs() < f64::EPSILON);
        assert!((coord.lng - (-97.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_lat_lng_default() {
        let coord = LatLng::default();
        assert!((coord.lat - 0.0).abs() < f64::EPSILON);
        assert!((coord.lng - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_radar_site_creation() {
        let site = RadarSite::new("KTLX", 35.0, -97.0, 300.0);
        assert_eq!(site.id, "KTLX");
        assert!((site.lat - 35.0).abs() < f64::EPSILON);
        assert!((site.lon - (-97.0)).abs() < f64::EPSILON);
        assert!((site.elevation_m - 300.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_radar_site_equality() {
        let site1 = RadarSite::new("KTLX", 35.0, -97.0, 300.0);
        let site2 = RadarSite::new("KTLX", 35.0, -97.0, 300.0);
        let site3 = RadarSite::new("KOKC", 35.0, -97.0, 300.0);
        assert_eq!(site1, site2);
        assert_ne!(site1, site3);
    }

    #[test]
    fn test_ktlx_station() {
        let station = get_station("KTLX");
        assert!(station.is_some());

        let site = station.unwrap();
        assert_eq!(site.id, "KTLX");
        // Oklahoma City area
        assert!((site.lat - 35.4183).abs() < 0.1);
        assert!((site.lon - (-97.4514)).abs() < 0.1);
        assert!((site.elevation_m - 374.0).abs() < 1.0);
    }

    #[test]
    fn test_station_case_insensitive() {
        assert!(get_station("ktlx").is_some());
        assert!(get_station("Ktlx").is_some());
        assert!(get_station("kTlX").is_some());
    }

    #[test]
    fn test_invalid_station() {
        assert!(get_station("INVALID").is_none());
        assert!(get_station("").is_none());
        assert!(get_station("XYZ").is_none());
    }

    #[test]
    fn test_all_stations_accessible() {
        let station_ids = [
            "KINX", "KFWS", "KHGX", "KMXX", "KFSD", "KBUF", "KMPX", "KSRX", "KVWX",
        ];

        for id in station_ids {
            let station = get_station(id);
            assert!(station.is_some(), "Station {} should be found", id);
        }
    }

    #[test]
    fn test_stations_count() {
        // Verify we have at least 10 stations as required
        assert!(
            STATIONS.len() >= 10,
            "Expected at least 10 stations, got {}",
            STATIONS.len()
        );
    }
}
