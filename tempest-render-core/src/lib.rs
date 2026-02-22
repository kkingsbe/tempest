//! Tempest Render Core - Phase 2: Geospatial projection and color mapping
//!
//! This crate provides:
//! - Color lookup tables for NEXRAD radar moments (REF, VEL, SW, ZDR, CC, KDP)
//! - Polar-to-geographic coordinate projection
//! - Color mapping from decoded radar values to RGBA

use std::fmt;

pub mod color;
pub mod projection;
pub mod types;

// Re-export types for convenient access from crate root
pub use types::{get_station, LatLng, RadarSite, STATIONS};

// Re-export color module types
pub use color::{ColorRamp, ColorStop, Rgb};

// Re-export reflectivity color ramp functions
pub use color::{reflectivity_color, reflectivity_ramp};

// Re-export velocity color ramp functions
pub use color::{velocity_color, velocity_ramp};

// Re-export ZDR color ramp functions
pub use color::{zdr_color, zdr_ramp};

// Re-export projection functions
pub use projection::project_volume_scan;

/// RGBA color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Rgba {
    /// Create a new RGBA color
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create from RGBA bytes
    pub const fn from_bytes(bytes: [u8; 4]) -> Self {
        Self::new(bytes[0], bytes[1], bytes[2], bytes[3])
    }

    /// Transparent color
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    /// Convert to array [r, g, b, a]
    pub const fn to_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// Radar moment types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadarMoment {
    /// Reflectivity (dBZ)
    Reflectivity,
    /// Velocity (m/s)
    Velocity,
    /// Spectrum Width (m/s)
    SpectrumWidth,
    /// Differential Reflectivity (dB)
    Zdr,
    /// Correlation Coefficient (0-1)
    Cc,
    /// Differential Phase (degrees)
    Kdp,
}

impl RadarMoment {
    /// Get the two-character NEXRAD moment code
    pub fn code(&self) -> &'static str {
        match self {
            RadarMoment::Reflectivity => "REF",
            RadarMoment::Velocity => "VEL",
            RadarMoment::SpectrumWidth => "SW",
            RadarMoment::Zdr => "ZDR",
            RadarMoment::Cc => "CC",
            RadarMoment::Kdp => "KDP",
        }
    }
}

/// Sentinel values for radar data
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RadarSentinel {
    /// No data available (below threshold)
    NoData,
    /// Range folded (ambiguous data)
    RangeFolded,
    /// Data is valid
    Valid,
}

/// Color table for radar moments
///
/// Implements NWS standard color tables for meteorological radar data.
#[derive(Debug, Clone)]
pub struct ColorTable {
    /// The moment type this table is for
    pub moment: RadarMoment,
    /// Minimum value in the color table
    pub min_value: f32,
    /// Maximum value in the color table
    pub max_value: f32,
    /// Color entries (value, color) sorted by value
    entries: Vec<(f32, Rgba)>,
    /// Color for no-data sentinel
    pub no_data_color: Rgba,
    /// Color for range-folded sentinel
    pub range_folded_color: Rgba,
}

impl ColorTable {
    /// Create a new color table
    pub fn new(
        moment: RadarMoment,
        min_value: f32,
        max_value: f32,
        entries: Vec<(f32, Rgba)>,
    ) -> Self {
        Self {
            moment,
            min_value,
            max_value,
            entries,
            no_data_color: Rgba::TRANSPARENT,
            range_folded_color: Rgba::new(128, 128, 128, 128), // Gray semi-transparent
        }
    }

    /// Map a radar value to an RGBA color
    ///
    /// # Arguments
    /// * `value` - The radar value to colorize
    /// * `sentinel` - Whether this is a sentinel value
    ///
    /// # Returns
    /// The RGBA color for this value
    pub fn colorize(&self, value: f32, sentinel: RadarSentinel) -> Rgba {
        match sentinel {
            RadarSentinel::NoData => self.no_data_color,
            RadarSentinel::RangeFolded => self.range_folded_color,
            RadarSentinel::Valid => self.colorize_value(value),
        }
    }

    /// Map a valid radar value to an RGBA color (no sentinel handling)
    fn colorize_value(&self, value: f32) -> Rgba {
        // Clamp to valid range
        let value = value.clamp(self.min_value, self.max_value);

        // Find the two entries to interpolate between
        if self.entries.is_empty() {
            return Rgba::TRANSPARENT;
        }

        if self.entries.len() == 1 {
            return self.entries[0].1;
        }

        // Binary search for the right position
        let mut low = 0;
        let mut high = self.entries.len() - 1;

        while low < high {
            let mid = (low + high + 1) / 2;
            if self.entries[mid].0 <= value {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        let (lower_value, lower_color) = self.entries[low];

        // If we're at the last entry, return it
        if low == self.entries.len() - 1 {
            return lower_color;
        }

        let (upper_value, upper_color) = self.entries[low + 1];

        // Interpolate between the two colors
        let t = (value - lower_value) / (upper_value - lower_value);
        let t = t.clamp(0.0, 1.0);

        Rgba::new(
            (lower_color.r as f32 + (upper_color.r as f32 - lower_color.r as f32) * t) as u8,
            (lower_color.g as f32 + (upper_color.g as f32 - lower_color.g as f32) * t) as u8,
            (lower_color.b as f32 + (upper_color.b as f32 - lower_color.b as f32) * t) as u8,
            (lower_color.a as f32 + (upper_color.a as f32 - lower_color.a as f32) * t) as u8,
        )
    }
}

/// NWS Standard Reflectivity Color Table
///
/// Standard NWS color scheme for reflectivity (dBZ):
/// -30 and below: Transparent
/// -30 to -20: Dark Purple
/// -20 to -10: Light Purple
/// -10 to 0: Blue
/// 0 to 10: Cyan
/// 10 to 20: Green
/// 20 to 30: Yellow
/// 30 to 40: Orange
/// 40 to 50: Red
/// 50 to 60: Dark Red
/// 60 to 75: Pink
/// Above 75: White
pub fn reflectivity_color_table() -> ColorTable {
    ColorTable::new(
        RadarMoment::Reflectivity,
        -30.0,
        75.0,
        vec![
            (-30.0, Rgba::new(77, 0, 77, 0)),      // Transparent
            (-20.0, Rgba::new(77, 0, 77, 128)),    // Dark Purple
            (-10.0, Rgba::new(142, 0, 214, 128)),  // Light Purple
            (0.0, Rgba::new(0, 0, 255, 192)),      // Blue
            (10.0, Rgba::new(0, 255, 255, 192)),   // Cyan
            (20.0, Rgba::new(0, 255, 0, 192)),     // Green
            (30.0, Rgba::new(255, 255, 0, 192)),   // Yellow
            (40.0, Rgba::new(255, 170, 0, 192)),   // Orange
            (50.0, Rgba::new(255, 0, 0, 192)),     // Red
            (60.0, Rgba::new(139, 0, 0, 192)),     // Dark Red
            (75.0, Rgba::new(255, 105, 180, 192)), // Pink
        ],
    )
}

/// NWS Standard Velocity Color Table
///
/// Divergent color scheme for velocity (m/s):
/// Negative (toward radar): Blue shades
/// Zero: White
/// Positive (away from radar): Red shades
///
/// # Color Stops
/// - -100 m/s: Dark Blue (toward)
/// - -75 m/s: Medium Blue
/// - -50 m/s: Blue
/// - -25 m/s: Light Blue
/// - 0 m/s: White (zero velocity)
/// - +25 m/s: Light Red
/// - +50 m/s: Red
/// - +75 m/s: Medium Red
/// - +100 m/s: Dark Red (away)
pub fn velocity_color_table() -> ColorTable {
    ColorTable::new(
        RadarMoment::Velocity,
        -100.0,
        100.0,
        vec![
            (-100.0, Rgba::new(0, 0, 100, 192)),    // Dark Blue (toward)
            (-75.0, Rgba::new(0, 0, 180, 192)),     // Medium Blue
            (-50.0, Rgba::new(0, 0, 255, 192)),     // Blue
            (-25.0, Rgba::new(100, 150, 255, 192)), // Light Blue
            (0.0, Rgba::new(255, 255, 255, 192)),   // White
            (25.0, Rgba::new(255, 150, 150, 192)),  // Light Red
            (50.0, Rgba::new(255, 50, 50, 192)),    // Red
            (75.0, Rgba::new(200, 0, 0, 192)),      // Medium Red
            (100.0, Rgba::new(139, 0, 0, 192)),     // Dark Red (away)
        ],
    )
}

/// NWS Standard Spectrum Width Color Table
///
/// Spectrum width (m/s) - indicates turbulence/storm complexity
pub fn spectrum_width_color_table() -> ColorTable {
    ColorTable::new(
        RadarMoment::SpectrumWidth,
        0.0,
        20.0,
        vec![
            (0.0, Rgba::new(0, 0, 0, 0)),        // Transparent
            (2.0, Rgba::new(0, 0, 255, 128)),    // Blue
            (4.0, Rgba::new(0, 255, 0, 128)),    // Green
            (8.0, Rgba::new(255, 255, 0, 128)),  // Yellow
            (12.0, Rgba::new(255, 128, 0, 128)), // Orange
            (20.0, Rgba::new(255, 0, 0, 128)),   // Red
        ],
    )
}

/// Standard ZDR (Differential Reflectivity) Color Table
///
/// ZDR in dB - indicates hail potential, rain vs snow
/// Typical values range from -7.5 to +7.5 dB:
///   - Negative ZDR: Light rain/snow (blues)
///   - Zero ZDR: Moderate rain (greens)
///   - Positive ZDR: Heavy rain/hail (yellows to reds to magenta)
pub fn zdr_color_table() -> ColorTable {
    ColorTable::new(
        RadarMoment::Zdr,
        -7.5,
        7.5,
        vec![
            (-7.5, Rgba::new(25, 25, 112, 128)), // Midnight Blue - light snow/light rain
            (-4.0, Rgba::new(0, 0, 255, 128)),   // Blue
            (-2.0, Rgba::new(0, 255, 255, 128)), // Cyan - light blue
            (0.0, Rgba::new(0, 255, 0, 128)),    // Green - moderate rain
            (1.5, Rgba::new(154, 205, 50, 128)), // YellowGreen
            (3.0, Rgba::new(255, 255, 0, 128)),  // Yellow - heavy rain
            (4.5, Rgba::new(255, 165, 0, 128)),  // Orange - very heavy rain/hail
            (6.0, Rgba::new(255, 0, 0, 128)),    // Red - hail
            (7.5, Rgba::new(255, 0, 255, 128)),  // Magenta - large hail
        ],
    )
}

/// Standard CC (Correlation Coefficient) Color Table
///
/// CC (0-1) - indicates weather type (rain vs hail vs biological)
pub fn cc_color_table() -> ColorTable {
    ColorTable::new(
        RadarMoment::Cc,
        0.0,
        1.0,
        vec![
            (0.0, Rgba::new(128, 0, 128, 192)), // Purple
            (0.5, Rgba::new(255, 0, 0, 192)),   // Red
            (0.8, Rgba::new(255, 255, 0, 192)), // Yellow
            (0.95, Rgba::new(0, 255, 0, 192)),  // Green
            (1.0, Rgba::new(0, 0, 255, 192)),   // Blue
        ],
    )
}

/// Standard KDP (Differential Phase) Color Table
///
/// KDP in degrees - indicates rain rate
pub fn kdp_color_table() -> ColorTable {
    ColorTable::new(
        RadarMoment::Kdp,
        0.0,
        10.0,
        vec![
            (0.0, Rgba::new(0, 0, 0, 0)),       // Transparent
            (0.5, Rgba::new(0, 0, 255, 128)),   // Blue
            (2.0, Rgba::new(0, 255, 0, 128)),   // Green
            (4.0, Rgba::new(255, 255, 0, 128)), // Yellow
            (6.0, Rgba::new(255, 128, 0, 128)), // Orange
            (10.0, Rgba::new(255, 0, 0, 128)),  // Red
        ],
    )
}

/// Get the color table for a specific moment
pub fn color_table_for_moment(moment: RadarMoment) -> ColorTable {
    match moment {
        RadarMoment::Reflectivity => reflectivity_color_table(),
        RadarMoment::Velocity => velocity_color_table(),
        RadarMoment::SpectrumWidth => spectrum_width_color_table(),
        RadarMoment::Zdr => zdr_color_table(),
        RadarMoment::Cc => cc_color_table(),
        RadarMoment::Kdp => kdp_color_table(),
    }
}

/// Colorize a radar value for the given moment
///
/// # Arguments
/// * `moment` - The radar moment type
/// * `value` - The radar value
/// * `sentinel` - Whether this is a sentinel value
///
/// # Returns
/// The RGBA color for this value
pub fn colorize(moment: RadarMoment, value: f32, sentinel: RadarSentinel) -> Rgba {
    color_table_for_moment(moment).colorize(value, sentinel)
}

// ============================================================================
// Geospatial Projection Module
// ============================================================================

/// Standard Earth radius in meters (WGS84 mean radius)
const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Effective Earth radius for standard atmospheric refraction (4/3 model)
/// This accounts for the bending of radar beams through the atmosphere.
const EFFECTIVE_EARTH_RADIUS_M: f64 = (4.0 / 3.0) * EARTH_RADIUS_M;

/// Convert polar radar coordinates to geographic coordinates.
///
/// Uses the standard atmospheric refraction model (4/3 earth radius)
/// for beam height calculation.
///
/// # Arguments
/// * `site` - Radar station location
/// * `azimuth_deg` - Azimuth angle in degrees (0 = North, clockwise)
/// * `range_m` - Range from radar in meters
/// * `elevation_deg` - Elevation angle in degrees (antenna elevation + beam tilt)
///
/// # Returns
/// * `LatLng` - Geographic coordinates of the point
///
/// # Formula
///
/// Beam height uses the 4/3 Earth radius model, accounting for
/// standard atmospheric refraction:
/// h = sqrt(r^2 + R_eff^2 + 2*r*R_eff*sin(elev)) - R_eff + elevation_m
///
/// Geographic projection uses spherical Earth approximation.
///
/// # Examples
///
/// ```
/// use tempest_render_core::{get_station, polar_to_latlng};
///
/// let site = get_station("KTLX").unwrap();
/// // 0° azimuth (North), 10km range, 0.5° elevation
/// let result = polar_to_latlng(site, 0.0, 10_000.0, 0.5);
/// assert!(result.lat > site.lat); // Should be north of site
/// ```
pub fn polar_to_latlng(
    site: &types::RadarSite,
    azimuth_deg: f64,
    range_m: f64,
    elevation_deg: f64,
) -> types::LatLng {
    // Convert inputs to radians
    let _azimuth_rad = azimuth_deg.to_radians();
    let elevation_rad = elevation_deg.to_radians();

    // Calculate beam height using 4/3 Earth radius model
    // h = sqrt(r² + R_eff² + 2*r*R_eff*sin(elev)) - R_eff + elevation_m
    let r = range_m;
    let r_eff = EFFECTIVE_EARTH_RADIUS_M;
    let term = r * r + r_eff * r_eff + 2.0 * r * r_eff * elevation_rad.sin();
    let _beam_height_m = term.sqrt() - r_eff + site.elevation_m;

    // Convert radar coordinates to geographic coordinates
    // Radar azimuth: 0° = North, clockwise
    // Spherical formula bearing: 0° = North, positive = clockwise
    // So bearing = azimuth (in radians)
    let bearing_rad = azimuth_deg.to_radians();

    // Angular distance from radar to point on Earth's surface
    // Using effective Earth radius for the great-circle distance
    let angular_distance = range_m / r_eff;

    // Site coordinates in radians
    let lat1 = site.lat.to_radians();
    let lon1 = site.lon.to_radians();

    // Spherical law of cosines for intermediate point
    let sin_lat2 = lat1.sin() * angular_distance.cos()
        + lat1.cos() * angular_distance.sin() * bearing_rad.cos();
    let lat2 = sin_lat2.asin();

    // Calculate longitude difference
    let y = bearing_rad.sin() * angular_distance.sin() * lat1.cos();
    let x = angular_distance.cos() - lat1.sin() * lat2.sin();
    let lon2 = lon1 + y.atan2(x);

    // Convert back to degrees and normalize longitude
    let lat = lat2.to_degrees();
    let mut lng = lon2.to_degrees();

    // Normalize longitude to [-180, 180]
    while lng > 180.0 {
        lng -= 360.0;
    }
    while lng < -180.0 {
        lng += 360.0;
    }

    types::LatLng::new(lat, lng)
}

// ============================================================================
// Sweep Projection Types
// ============================================================================

/// A projected point with geographic coordinates and a moment value.
///
/// Represents a single radar gate projected to geographic coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProjectedPoint {
    /// Latitude in degrees
    pub lat: f64,
    /// Longitude in degrees
    pub lng: f64,
    /// Moment value (dBZ, m/s, etc.)
    pub value: f32,
}

impl ProjectedPoint {
    /// Create a new projected point.
    #[inline]
    pub fn new(lat: f64, lng: f64, value: f32) -> Self {
        Self { lat, lng, value }
    }
}

/// A projected sweep with elevation and all projected points.
///
/// Contains all the projected gate data for a single elevation sweep.
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectedSweep {
    /// Elevation angle in degrees
    pub elevation: f32,
    /// Projected points for all gates in this sweep
    pub points: Vec<ProjectedPoint>,
}

impl ProjectedSweep {
    /// Create a new projected sweep.
    #[inline]
    pub fn new(elevation: f32, points: Vec<ProjectedPoint>) -> Self {
        Self { elevation, points }
    }

    /// Returns the number of projected points.
    #[inline]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns true if the sweep has no points.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

/// Project a radar sweep to geographic coordinates.
///
/// Converts polar coordinate radial data to geographic lat/lng coordinates
/// using the radar site location and beam height model.
///
/// # Arguments
/// * `site` - Radar station location
/// * `sweep` - Decoded sweep from tempest-decode
/// * `moment` - Which moment to project (Reflectivity, Velocity, etc.)
///
/// # Returns
/// * `ProjectedSweep` - Geographic coordinates with moment values
///
/// # Examples
///
/// ```
/// use tempest_render_core::{get_station, project_sweep};
/// use tempest_decode::{Moment, Sweep, Radial, Gate};
///
/// // Create a simple test sweep
/// let mut sweep = Sweep::new(0.5);
/// let mut radial = Radial::new(0.0);
/// let mut gate = Gate::new(1000.0);
/// gate.reflectivity = Some(30.0);
/// radial.gates.push(gate);
/// sweep.radials.push(radial);
///
/// let site = get_station("KTLX").unwrap();
/// let projected = project_sweep(site, &sweep, Moment::Reflectivity);
///
/// assert!(!projected.points.is_empty());
/// let point = &projected.points[0];
/// assert!(point.lat != site.lat || point.lng != site.lon); // Point moved from radar
/// ```
pub fn project_sweep(
    site: &types::RadarSite,
    sweep: &tempest_decode::Sweep,
    moment: tempest_decode::Moment,
) -> ProjectedSweep {
    let mut points = Vec::new();

    // Iterate over each radial in the sweep
    for radial in &sweep.radials {
        // Iterate over each gate in the radial
        for gate in &radial.gates {
            // Extract the appropriate moment value
            let value = match moment {
                tempest_decode::Moment::Reflectivity => gate.reflectivity,
                tempest_decode::Moment::Velocity => gate.velocity,
                tempest_decode::Moment::SpectrumWidth => gate.spectrum_width,
                tempest_decode::Moment::Zdr => gate.zdr,
                tempest_decode::Moment::Cc => gate.cc,
                tempest_decode::Moment::Kdp => gate.kdp,
            };

            // Only include gates with valid moment data
            if let Some(value) = value {
                // Calculate geographic coordinates
                let latlng = polar_to_latlng(
                    site,
                    radial.azimuth as f64,
                    gate.range as f64,
                    sweep.elevation as f64,
                );

                points.push(ProjectedPoint::new(latlng.lat, latlng.lng, value));
            }
        }
    }

    ProjectedSweep::new(sweep.elevation, points)
}

#[cfg(test)]
mod projection_tests {
    use super::*;
    use crate::types::RadarSite;

    /// Test cardinal direction: 0° (North) should produce increasing latitude
    #[test]
    fn test_azimuth_north_increases_lat() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let result = polar_to_latlng(&site, 0.0, 10_000.0, 0.5);
        assert!(
            result.lat > site.lat,
            "North should increase latitude: site={}, result={}",
            site.lat,
            result.lat
        );
    }

    /// Test cardinal direction: 90° (East) should produce increasing longitude
    #[test]
    fn test_azimuth_east_increases_lng() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let result = polar_to_latlng(&site, 90.0, 10_000.0, 0.5);
        assert!(
            result.lng > site.lon,
            "East should increase longitude: site={}, result={}",
            site.lon,
            result.lng
        );
    }

    /// Test cardinal direction: 180° (South) should produce decreasing latitude
    #[test]
    fn test_azimuth_south_decreases_lat() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let result = polar_to_latlng(&site, 180.0, 10_000.0, 0.5);
        assert!(
            result.lat < site.lat,
            "South should decrease latitude: site={}, result={}",
            site.lat,
            result.lat
        );
    }

    /// Test cardinal direction: 270° (West) should produce decreasing longitude
    #[test]
    fn test_azimuth_west_decreases_lng() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let result = polar_to_latlng(&site, 270.0, 10_000.0, 0.5);
        assert!(
            result.lng < site.lon,
            "West should decrease longitude: site={}, result={}",
            site.lon,
            result.lng
        );
    }

    /// Test that zero range returns the radar site location
    #[test]
    fn test_zero_range_returns_site() {
        let site = RadarSite::new("TEST", 35.4183, -97.4514, 374.0);
        let result = polar_to_latlng(&site, 45.0, 0.0, 0.5);
        assert!((result.lat - site.lat).abs() < 1e-10);
        assert!((result.lng - site.lon).abs() < 1e-10);
    }

    /// Test that 360° azimuth equals 0° azimuth
    #[test]
    fn test_azimuth_360_equals_0() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let result_0 = polar_to_latlng(&site, 0.0, 10_000.0, 0.5);
        let result_360 = polar_to_latlng(&site, 360.0, 10_000.0, 0.5);
        assert!((result_0.lat - result_360.lat).abs() < 1e-10);
        assert!((result_0.lng - result_360.lng).abs() < 1e-10);
    }

    /// Test that larger range produces larger displacement
    #[test]
    fn test_larger_range_produces_larger_displacement() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let result_10km = polar_to_latlng(&site, 90.0, 10_000.0, 0.5);
        let result_20km = polar_to_latlng(&site, 90.0, 20_000.0, 0.5);

        let dist_10km =
            ((result_10km.lat - site.lat).powi(2) + (result_10km.lng - site.lon).powi(2)).sqrt();
        let dist_20km =
            ((result_20km.lat - site.lat).powi(2) + (result_20km.lng - site.lon).powi(2)).sqrt();

        assert!(
            dist_20km > dist_10km,
            "20km should be further than 10km: {} vs {}",
            dist_20km,
            dist_10km
        );
    }

    /// Test with real NEXRAD station (KTLX)
    #[test]
    fn test_with_ktlx_station() {
        let site = get_station("KTLX").expect("KTLX should be found");
        let result = polar_to_latlng(site, 90.0, 50_000.0, 0.5);

        // 50km east should still be in Oklahoma
        assert!(result.lat > 34.0 && result.lat < 36.0);
        assert!(result.lng > -98.0 && result.lng < -96.0);
    }

    /// Test beam height calculation at known point
    /// At 10km range and 0.5° elevation, beam height should be approximately 87m + site elevation
    #[test]
    fn test_beam_height_calculation() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let range_m = 10_000.0;
        let elevation_deg: f64 = 0.5;

        // Calculate expected beam height
        let r = range_m;
        let r_eff = EFFECTIVE_EARTH_RADIUS_M;
        let elev_rad = elevation_deg.to_radians();
        let term = r * r + r_eff * r_eff + 2.0 * r * r_eff * elev_rad.sin();
        let expected_height = term.sqrt() - r_eff + site.elevation_m;

        // The height should be positive and approximately 87m above site
        assert!(expected_height > 0.0, "Beam height should be positive");
        assert!(
            (expected_height - 300.0 - 87.0).abs() < 50.0,
            "Beam height should be ~87m above site elevation: {}",
            expected_height - 300.0
        );
    }
}

#[cfg(test)]
mod sweep_projection_tests {
    use super::*;
    use tempest_decode::{Gate, Moment, Radial, Sweep};

    /// Test project_sweep with a simple single-gate radial
    #[test]
    fn test_project_sweep_single_gate() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(0.0);
        let mut gate = Gate::new(10_000.0); // 10km range
        gate.reflectivity = Some(30.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);

        let projected = project_sweep(&site, &sweep, Moment::Reflectivity);

        assert_eq!(projected.elevation, 0.5);
        assert_eq!(projected.points.len(), 1);

        let point = &projected.points[0];
        assert!((point.lat - site.lat).abs() > 0.01); // Should be north of site
        assert_eq!(point.value, 30.0);
    }

    /// Test project_sweep produces correct number of points
    #[test]
    fn test_project_sweep_point_count() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let mut sweep = Sweep::new(0.5);

        // Create 2 radials with 3 gates each = 6 points
        for az in [0.0, 90.0] {
            let mut radial = Radial::new(az);
            for range in [5000.0, 10000.0, 15000.0] {
                let mut gate = Gate::new(range);
                gate.velocity = Some(10.0);
                radial.gates.push(gate);
            }
            sweep.radials.push(radial);
        }

        let projected = project_sweep(&site, &sweep, Moment::Velocity);
        assert_eq!(projected.points.len(), 6);
    }

    /// Test project_sweep filters out gates without the requested moment
    #[test]
    fn test_project_sweep_filters_missing_moments() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(0.0);

        // Gate with only reflectivity
        let mut gate1 = Gate::new(10_000.0);
        gate1.reflectivity = Some(30.0);
        // Gate with only velocity
        let mut gate2 = Gate::new(20_000.0);
        gate2.velocity = Some(15.0);

        radial.gates.push(gate1);
        radial.gates.push(gate2);
        sweep.radials.push(radial);

        // Project reflectivity - should only get 1 point
        let projected_refl = project_sweep(&site, &sweep, Moment::Reflectivity);
        assert_eq!(projected_refl.points.len(), 1);
        assert_eq!(projected_refl.points[0].value, 30.0);

        // Project velocity - should only get 1 point
        let projected_vel = project_sweep(&site, &sweep, Moment::Velocity);
        assert_eq!(projected_vel.points.len(), 1);
        assert_eq!(projected_vel.points[0].value, 15.0);
    }

    /// Test that projected points are within reasonable distance of radar site
    #[test]
    fn test_project_sweep_points_within_range() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(90.0); // East

        // 50km range
        let mut gate = Gate::new(50_000.0);
        gate.reflectivity = Some(40.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);

        let projected = project_sweep(&site, &sweep, Moment::Reflectivity);

        let point = &projected.points[0];
        // 50km east should increase longitude
        assert!(point.lng > site.lon);
        // Latitude should be roughly similar (within 1 degree)
        assert!((point.lat - site.lat).abs() < 1.0);
    }

    /// Test with KTLX real station coordinates
    #[test]
    #[allow(clippy::needless_borrow)]
    fn test_project_sweep_with_ktlx() {
        let site = get_station("KTLX").expect("KTLX should be found");
        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(45.0);

        let mut gate = Gate::new(20_000.0); // 20km
        gate.zdr = Some(2.5);
        radial.gates.push(gate);
        sweep.radials.push(radial);

        let projected = project_sweep(&site, &sweep, Moment::Zdr);

        assert_eq!(projected.points.len(), 1);
        let point = &projected.points[0];

        // Should be somewhere in Oklahoma
        assert!(point.lat > 34.0 && point.lat < 37.0);
        assert!(point.lng > -99.0 && point.lng < -96.0);
        assert_eq!(point.value, 2.5);
    }

    /// Test ProjectedSweep helper methods
    #[test]
    fn test_projected_sweep_len_and_is_empty() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let sweep = Sweep::new(0.5);

        let projected = project_sweep(&site, &sweep, Moment::Reflectivity);
        assert!(projected.is_empty());
        assert_eq!(projected.len(), 0);
    }

    /// Test all moment types can be projected
    #[test]
    fn test_project_sweep_all_moments() {
        let site = RadarSite::new("TEST", 35.0, -97.0, 300.0);
        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(0.0);
        let mut gate = Gate::new(10_000.0);
        gate.reflectivity = Some(30.0);
        gate.velocity = Some(10.0);
        gate.spectrum_width = Some(5.0);
        gate.zdr = Some(1.0);
        gate.cc = Some(0.95);
        gate.kdp = Some(2.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);

        let moments = [
            Moment::Reflectivity,
            Moment::Velocity,
            Moment::SpectrumWidth,
            Moment::Zdr,
            Moment::Cc,
            Moment::Kdp,
        ];

        for moment in moments {
            let projected = project_sweep(&site, &sweep, moment);
            assert_eq!(
                projected.points.len(),
                1,
                "Moment {:?} should have 1 point",
                moment
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflectivity_color_table_size() {
        let table = reflectivity_color_table();
        assert!(!table.entries.is_empty());
    }

    #[test]
    fn test_velocity_color_table_size() {
        let table = velocity_color_table();
        assert!(!table.entries.is_empty());
    }

    #[test]
    fn test_colorize_reflectivity_green() {
        let table = reflectivity_color_table();
        // 25 dBZ should be green-yellow (between 20 green and 30 yellow)
        let color = table.colorize(25.0, RadarSentinel::Valid);
        // Should be somewhere between green (0,255,0) and yellow (255,255,0)
        assert!(color.g > 200);
    }

    #[test]
    fn test_colorize_reflectivity_clamped() {
        let table = reflectivity_color_table();
        // Values above max should clamp to max color
        let color = table.colorize(100.0, RadarSentinel::Valid);
        assert_eq!(color.r, 255); // Pink max
        assert_eq!(color.g, 105);
        assert_eq!(color.b, 180);
    }

    #[test]
    fn test_colorize_velocity_zero() {
        let table = velocity_color_table();
        // 0 m/s should be white
        let color = table.colorize(0.0, RadarSentinel::Valid);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_colorize_no_data_is_transparent() {
        let table = reflectivity_color_table();
        let color = table.colorize(0.0, RadarSentinel::NoData);
        assert_eq!(color.a, 0);
    }

    #[test]
    fn test_colorize_range_folded_is_gray() {
        let table = reflectivity_color_table();
        let color = table.colorize(0.0, RadarSentinel::RangeFolded);
        assert_eq!(color.r, 128);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 128);
    }

    #[test]
    fn test_colorize_all_moments() {
        let moments = vec![
            RadarMoment::Reflectivity,
            RadarMoment::Velocity,
            RadarMoment::SpectrumWidth,
            RadarMoment::Zdr,
            RadarMoment::Cc,
            RadarMoment::Kdp,
        ];
        for moment in moments {
            let table = color_table_for_moment(moment);
            assert!(!table.entries.is_empty());
        }
    }

    #[test]
    fn test_rgba_display() {
        let color = Rgba::new(255, 128, 64, 32);
        assert_eq!(format!("{}", color), "rgba(255, 128, 64, 32)");
    }

    #[test]
    fn test_rgba_to_array() {
        let color = Rgba::new(255, 128, 64, 32);
        assert_eq!(color.to_array(), [255, 128, 64, 32]);
    }

    #[test]
    fn test_moment_code() {
        assert_eq!(RadarMoment::Reflectivity.code(), "REF");
        assert_eq!(RadarMoment::Velocity.code(), "VEL");
        assert_eq!(RadarMoment::SpectrumWidth.code(), "SW");
        assert_eq!(RadarMoment::Zdr.code(), "ZDR");
        assert_eq!(RadarMoment::Cc.code(), "CC");
        assert_eq!(RadarMoment::Kdp.code(), "KDP");
    }

    #[test]
    fn test_colorize_function() {
        let color = colorize(RadarMoment::Reflectivity, 30.0, RadarSentinel::Valid);
        // 30 dBZ should be yellow
        assert!(color.r > 200);
        assert!(color.g > 200);
    }
}
