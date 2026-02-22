//! Shared types for NEXRAD data decoding.

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
