//! Volume scan projection module.
//!
//! This module provides functions to project complete volume scans
//! from decoded radar data to geographic coordinates.

use tempest_decode::{Gate, Sweep, VolumeScan};

use crate::{polar_to_latlng, types::RadarSite, ProjectedPoint, ProjectedSweep, RadarMoment};

/// Project a complete volume scan to geographic coordinates.
///
/// Takes a decoded VolumeScan and radar site location, produces
/// projected sweeps ready for rendering.
///
/// # Arguments
/// * `volume` - Decoded volume scan from tempest-decode
/// * `site` - Radar station location
/// * `moment` - Which radar moment to project (Reflectivity, Velocity, etc.)
///
/// # Returns
/// Vector of projected sweeps, one per elevation angle
///
/// # Examples
///
/// ```
/// use tempest_render_core::{get_station, project_volume_scan, RadarMoment};
/// use tempest_decode::decode;
///
/// // Decode radar data (typically from file)
/// // let data = std::fs::read("radar_data.ar2v").unwrap();
/// // let volume = decode(&data).unwrap();
///
/// // For testing, create a mock volume scan
/// use tempest_decode::{VolumeScan, Sweep, Radial, Gate};
/// use chrono::Utc;
///
/// let volume = VolumeScan::new("KTLX", Utc::now(), 215);
/// let site = get_station("KTLX").unwrap();
///
/// let projected = project_volume_scan(&volume, site, RadarMoment::Reflectivity);
/// assert!(projected.is_empty()); // Empty volume has no sweeps
/// ```
pub fn project_volume_scan(
    volume: &VolumeScan,
    site: &RadarSite,
    moment: RadarMoment,
) -> Vec<ProjectedSweep> {
    volume
        .sweeps
        .iter()
        .map(|sweep| project_sweep_internal(site, sweep, moment))
        .collect()
}

/// Internal function to project a single sweep using RadarMoment.
fn project_sweep_internal(site: &RadarSite, sweep: &Sweep, moment: RadarMoment) -> ProjectedSweep {
    let mut points = Vec::new();

    // Iterate over each radial in the sweep
    for radial in &sweep.radials {
        // Iterate over each gate in the radial
        for gate in &radial.gates {
            // Extract the appropriate moment value using RadarMoment
            let value = get_moment_value(gate, moment);

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

/// Extract a moment value from a gate based on the RadarMoment enum.
///
/// Returns the value for the specified moment, or None if not available.
fn get_moment_value(gate: &Gate, moment: RadarMoment) -> Option<f32> {
    match moment {
        RadarMoment::Reflectivity => gate.reflectivity,
        RadarMoment::Velocity => gate.velocity,
        RadarMoment::SpectrumWidth => gate.spectrum_width,
        RadarMoment::Zdr => gate.zdr,
        RadarMoment::Cc => gate.cc,
        RadarMoment::Kdp => gate.kdp,
    }
}

/// Project a sweep to geographic coordinates using RadarMoment.
///
/// This is a convenience function for projecting a single sweep
/// when you don't have a full volume scan.
///
/// # Arguments
/// * `site` - Radar station location
/// * `sweep` - Decoded sweep from tempest-decode
/// * `moment` - Which moment to project
///
/// # Returns
/// * `ProjectedSweep` - Geographic coordinates with moment values
pub fn project_sweep(site: &RadarSite, sweep: &Sweep, moment: RadarMoment) -> ProjectedSweep {
    project_sweep_internal(site, sweep, moment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RadarSite;
    use chrono::Utc;
    use tempest_decode::{Gate, Radial, Sweep, VolumeScan};

    /// Test projecting an empty volume scan
    #[test]
    fn test_empty_volume() {
        let volume = VolumeScan::new("KTLX", Utc::now(), 215);
        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        let projected = project_volume_scan(&volume, &site, RadarMoment::Reflectivity);

        assert!(projected.is_empty());
    }

    /// Test projecting a volume with one sweep
    #[test]
    fn test_single_sweep_volume() {
        let mut volume = VolumeScan::new("KTLX", Utc::now(), 215);

        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(0.0);
        let mut gate = Gate::new(1000.0);
        gate.reflectivity = Some(30.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);
        volume.sweeps.push(sweep);

        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        let projected = project_volume_scan(&volume, &site, RadarMoment::Reflectivity);

        assert_eq!(projected.len(), 1);
        assert_eq!(projected[0].elevation, 0.5);
        assert_eq!(projected[0].points.len(), 1);

        let point = &projected[0].points[0];
        assert!((point.value - 30.0).abs() < 0.001);
    }

    /// Test projecting multiple sweeps
    #[test]
    fn test_multiple_sweeps() {
        let mut volume = VolumeScan::new("KTLX", Utc::now(), 215);

        for elevation in [0.5, 1.5, 2.5] {
            let mut sweep = Sweep::new(elevation);
            let mut radial = Radial::new(0.0);
            let mut gate = Gate::new(1000.0);
            gate.reflectivity = Some(20.0);
            radial.gates.push(gate);
            sweep.radials.push(radial);
            volume.sweeps.push(sweep);
        }

        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        let projected = project_volume_scan(&volume, &site, RadarMoment::Reflectivity);

        assert_eq!(projected.len(), 3);
        assert_eq!(projected[0].elevation, 0.5);
        assert_eq!(projected[1].elevation, 1.5);
        assert_eq!(projected[2].elevation, 2.5);
    }

    /// Test projecting velocity moment
    #[test]
    fn test_velocity_moment() {
        let mut volume = VolumeScan::new("KTLX", Utc::now(), 215);

        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(90.0);
        let mut gate = Gate::new(5000.0);
        gate.velocity = Some(-25.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);
        volume.sweeps.push(sweep);

        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        let projected = project_volume_scan(&volume, &site, RadarMoment::Velocity);

        assert_eq!(projected[0].points.len(), 1);
        let point = &projected[0].points[0];
        assert!((point.value - (-25.0)).abs() < 0.001);
    }

    /// Test that gates without the requested moment are skipped
    #[test]
    fn test_missing_moment_skipped() {
        let mut volume = VolumeScan::new("KTLX", Utc::now(), 215);

        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(0.0);

        // Gate with reflectivity only
        let mut gate1 = Gate::new(1000.0);
        gate1.reflectivity = Some(30.0);

        // Gate with velocity only
        let mut gate2 = Gate::new(2000.0);
        gate2.velocity = Some(10.0);

        // Gate with both
        let mut gate3 = Gate::new(3000.0);
        gate3.reflectivity = Some(25.0);
        gate3.velocity = Some(15.0);

        radial.gates.push(gate1);
        radial.gates.push(gate2);
        radial.gates.push(gate3);
        sweep.radials.push(radial);
        volume.sweeps.push(sweep);

        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        // Project reflectivity - should get 2 points (gate1 and gate3)
        let reflected = project_volume_scan(&volume, &site, RadarMoment::Reflectivity);
        assert_eq!(reflected[0].points.len(), 2);

        // Project velocity - should get 2 points (gate2 and gate3)
        let velocity = project_volume_scan(&volume, &site, RadarMoment::Velocity);
        assert_eq!(velocity[0].points.len(), 2);
    }

    /// Test that projected points are geographically displaced from radar site
    #[test]
    fn test_projected_points_displaced() {
        let mut volume = VolumeScan::new("KTLX", Utc::now(), 215);

        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(90.0); // East
        let mut gate = Gate::new(10000.0); // 10km
        gate.reflectivity = Some(30.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);
        volume.sweeps.push(sweep);

        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        let projected = project_volume_scan(&volume, &site, RadarMoment::Reflectivity);

        let point = &projected[0].points[0];
        // Point should be east of the radar (higher longitude)
        assert!(point.lng > site.lon);
    }

    /// Test the convenience project_sweep function
    #[test]
    fn test_project_sweep_convenience() {
        let mut sweep = Sweep::new(0.5);
        let mut radial = Radial::new(0.0);
        let mut gate = Gate::new(1000.0);
        gate.reflectivity = Some(30.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);

        let site = RadarSite::new("KTLX", 35.4183, -97.4514, 374.0);

        let projected = project_sweep(&site, &sweep, RadarMoment::Reflectivity);

        assert_eq!(projected.elevation, 0.5);
        assert_eq!(projected.points.len(), 1);
    }
}
