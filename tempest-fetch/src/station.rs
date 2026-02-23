//! NEXRAD Station types and registry for Tempest weather radar application.
//!
//! This module provides the core data structures for representing NEXRAD radar
//! stations and a registry for looking them up by their ICAO identifier.

use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

/// Represents a NEXRAD weather radar station.
///
/// Stations are identified by their 4-letter ICAO code (e.g., "KTLX" for
/// Oklahoma City). The station metadata includes geographic location
/// and elevation information.
#[derive(Debug, Clone, PartialEq)]
pub struct Station {
    /// The ICAO identifier (e.g., "KTLX", "KICT")
    pub id: String,
    /// Human-readable station name (e.g., "Oklahoma City")
    pub name: String,
    /// Latitude in decimal degrees (negative for southern hemisphere)
    pub lat: f64,
    /// Longitude in decimal degrees (negative for western hemisphere)
    pub lon: f64,
    /// Station elevation in meters above sea level
    pub elevation_m: f32,
}

impl Station {
    /// Creates a new Station with the given parameters.
    #[inline]
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        lat: f64,
        lon: f64,
        elevation_m: f32,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            lat,
            lon,
            elevation_m,
        }
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.id, self.name)
    }
}

/// A registry of all known NEXRAD stations.
///
/// Provides fast lookup by station ID and iteration over all stations.
/// The registry is populated with embedded station metadata at compile time.
#[derive(Debug, Clone)]
pub struct StationRegistry {
    /// Map from station ICAO code to station data
    stations: HashMap<String, Station>,
    /// Ordered list of all station IDs
    station_ids: Vec<String>,
}

impl StationRegistry {
    /// Creates a new registry populated with all known NEXRAD stations.
    ///
    /// The registry is initialized with embedded station data at compile time.
    #[must_use]
    pub fn new() -> Self {
        let stations = crate::stations_data::STATIONS
            .iter()
            .map(|s| {
                (
                    s.id.to_string(),
                    Station {
                        id: s.id.to_string(),
                        name: s.name.to_string(),
                        lat: s.lat,
                        lon: s.lon,
                        elevation_m: s.elevation_m,
                    },
                )
            })
            .collect();

        let mut station_ids: Vec<String> = crate::stations_data::STATIONS
            .iter()
            .map(|s| s.id.to_string())
            .collect();
        station_ids.sort();

        Self {
            stations,
            station_ids,
        }
    }

    /// Returns a reference to the station with the given ICAO identifier, if it exists.
    #[inline]
    pub fn get(&self, id: &str) -> Option<&Station> {
        self.stations.get(id)
    }

    /// Returns an iterator over all stations, sorted by ICAO identifier.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Station> {
        self.station_ids
            .iter()
            .filter_map(|id| self.stations.get(id))
    }

    /// Returns the number of stations in the registry.
    #[inline]
    pub fn len(&self) -> usize {
        self.stations.len()
    }

    /// Returns true if the registry contains no stations.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.stations.is_empty()
    }
}

impl Default for StationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns a reference to the global station registry.
///
/// This is a convenience function that provides access to all known NEXRAD stations.
#[must_use]
pub fn registry() -> &'static StationRegistry {
    static REGISTRY: OnceLock<StationRegistry> = OnceLock::new();
    REGISTRY.get_or_init(StationRegistry::new)
}

/// Returns all stations sorted by ICAO identifier.
#[must_use]
pub fn list_all_stations() -> Vec<Station> {
    registry().iter().cloned().collect()
}

/// Looks up a station by its ICAO identifier.
///
/// # Arguments
/// * `id` - The 4-letter ICAO code (e.g., "KTLX", "KICT")
///
/// # Returns
/// * `Some(Station)` if the station exists in the registry
/// * `None` if the station is not found
#[must_use]
pub fn get_station(id: &str) -> Option<Station> {
    registry().get(id).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_stations() {
        let reg = registry();
        assert!(!reg.is_empty());
        assert!(reg.len() > 10);
    }

    #[test]
    fn test_get_station_ktlx() {
        let station = get_station("KTLX");
        assert!(station.is_some());
        let s = station.unwrap();
        assert_eq!(s.id, "KTLX");
        // Oklahoma City coordinates should be approximately:
        // lat: 35.4, lon: -97.5
        assert!((s.lat - 35.4).abs() < 0.5);
        assert!((s.lon - (-97.5)).abs() < 0.5);
    }

    #[test]
    fn test_get_station_invalid() {
        let station = get_station("INVALID");
        assert!(station.is_none());
    }

    #[test]
    fn test_list_all_stations_sorted() {
        let stations = list_all_stations();
        assert!(!stations.is_empty());
        for i in 1..stations.len() {
            assert!(stations[i - 1].id <= stations[i].id);
        }
    }
}
