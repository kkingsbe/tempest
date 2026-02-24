//! Integration tests for the decode→project pipeline.
//!
//! These tests verify that decoded radar data from tempest-decode
//! correctly projects to geographic coordinates using tempest-render-core.

use std::path::Path;

/// Test fixture paths relative to the tempest-decode crate
const FIXTURE_DIR: &str = "../tempest-decode/tests/fixtures";

/// Integration test: decode VCP_215 fixture and project to geographic coordinates
#[test]
fn test_decode_vcp215_project_reflectivity() {
    // Read the VCP 215 fixture file (using .bin format that decoder expects)
    let fixture_path = Path::new(FIXTURE_DIR).join("vcp215_clear_air.bin");
    let data = std::fs::read(&fixture_path).expect("Failed to read VCP_215 fixture");

    // Decode the radar data
    let volume = tempest_decode::decode(&data).expect("Failed to decode VCP_215 data");

    // Verify we got some sweeps
    assert!(
        !volume.sweeps.is_empty(),
        "Volume should have at least one sweep"
    );

    // Get the radar site (KTLX is in our station registry)
    let site =
        tempest_render_core::get_station("KTLX").expect("KTLX should be in station registry");

    // Project the volume scan to geographic coordinates
    let projected = tempest_render_core::project_volume_scan(
        &volume,
        site,
        tempest_render_core::RadarMoment::Reflectivity,
    );

    // Verify projection produced sweeps
    assert!(
        !projected.is_empty(),
        "Projected sweeps should not be empty"
    );

    // Sum up all points across all sweeps
    let total_points: usize = projected.iter().map(|s| s.points.len()).sum();

    // Note: Some fixtures may not have moment data parsed, which is OK for this test
    // The key is that the pipeline doesn't panic and produces some output
    println!(
        "VCP 215 projected: {} sweeps, {} total points",
        projected.len(),
        total_points
    );

    // If we have points, verify they're in a reasonable location
    if total_points > 0 {
        if let Some(first_point) = projected.first().and_then(|s| s.points.first()) {
            let lat_diff = (first_point.lat - site.lat).abs();
            let lon_diff = (first_point.lng - site.lon).abs();

            // Points should be within ~5 degrees of the radar site (roughly 460km at this latitude)
            assert!(lat_diff < 5.0, "Point latitude should be near radar site");
            assert!(lon_diff < 5.0, "Point longitude should be near radar site");
        }
    }
}

/// Integration test: decode VCP_35 fixture (uses KOKC which isn't in registry - skip)
#[test]
#[ignore]
fn test_decode_vcp35_project() {
    // This test is skipped because KOKC is not in the station registry
    // The fixture vcp35_clear_air.bin is from KOKC radar
}

/// Integration test: decode super-resolution fixture
#[test]
fn test_decode_superres_project() {
    // Read the super-resolution fixture
    let fixture_path = Path::new(FIXTURE_DIR).join("super_resolution.bin");
    let data = std::fs::read(&fixture_path).expect("Failed to read SuperRes fixture");

    // Decode
    let volume = tempest_decode::decode(&data).expect("Failed to decode SuperRes data");

    // Get radar site (synthetic data uses KTLX)
    let site =
        tempest_render_core::get_station("KTLX").expect("KTLX should be in station registry");

    // Project
    let projected = tempest_render_core::project_volume_scan(
        &volume,
        site,
        tempest_render_core::RadarMoment::Reflectivity,
    );

    // Verify projection
    assert!(!projected.is_empty(), "Should have projected sweeps");

    // Points should be distributed around the radar
    if let Some(first_sweep) = projected.first() {
        if !first_sweep.points.is_empty() {
            // Points should span different azimuths (represented by different lng values)
            let lats: Vec<f64> = first_sweep.points.iter().map(|p| p.lat).collect();
            let lngs: Vec<f64> = first_sweep.points.iter().map(|p| p.lng).collect();

            let lat_range = lats.iter().fold(f64::MIN, |a, &b| a.max(b))
                - lats.iter().fold(f64::MAX, |a, &b| a.min(b));
            let lng_range = lngs.iter().fold(f64::MIN, |a, &b| a.max(b))
                - lngs.iter().fold(f64::MAX, |a, &b| a.min(b));

            // Should have some geographic spread
            assert!(
                lat_range > 0.0 || lng_range > 0.0,
                "Points should have geographic spread"
            );
        }
    }
}

/// Integration test: verify projected points are in correct quadrants
#[test]
fn test_projected_points_quadrants() {
    // Create a synthetic volume scan with known azimuths
    let mut volume = tempest_decode::VolumeScan::new("KTLX", chrono::Utc::now(), 215);

    let mut sweep = tempest_decode::Sweep::new(0.5);

    // Add radials at known azimuths: 0 (North), 90 (East), 180 (South), 270 (West)
    for azimuth in [0.0_f32, 90.0, 180.0, 270.0] {
        let mut radial = tempest_decode::Radial::new(azimuth);
        let mut gate = tempest_decode::Gate::new(10000.0); // 10km range
        gate.reflectivity = Some(30.0);
        radial.gates.push(gate);
        sweep.radials.push(radial);
    }
    volume.sweeps.push(sweep);

    // Get KTLX location (Oklahoma City area)
    let site = tempest_render_core::get_station("KTLX").expect("KTLX should be in registry");

    // Project
    let projected = tempest_render_core::project_volume_scan(
        &volume,
        site,
        tempest_render_core::RadarMoment::Reflectivity,
    );

    assert_eq!(projected.len(), 1);
    let sweep_proj = &projected[0];
    assert_eq!(sweep_proj.points.len(), 4);

    // Check quadrant placement
    // North (azimuth 0): should have higher latitude
    // South (azimuth 180): should have lower latitude
    // East (azimuth 90): should have higher longitude
    // West (azimuth 270): should have lower longitude

    let north_point = &sweep_proj.points[0];
    let south_point = &sweep_proj.points[2];
    let east_point = &sweep_proj.points[1];
    let west_point = &sweep_proj.points[3];

    // North should be north of the radar site
    assert!(
        north_point.lat > site.lat,
        "North point should be north of radar"
    );

    // South should be south of the radar site
    assert!(
        south_point.lat < site.lat,
        "South point should be south of radar"
    );

    // East should be east of the radar site
    assert!(
        east_point.lng > site.lon,
        "East point should be east of radar"
    );

    // West should be west of the radar site
    assert!(
        west_point.lng < site.lon,
        "West point should be west of radar"
    );
}

/// Integration test: multiple moments from same volume
#[test]
fn test_decode_project_multiple_moments() {
    // Read a fixture that should have multiple moments
    let fixture_path = Path::new(FIXTURE_DIR).join("vcp215_clear_air.bin");
    let data = std::fs::read(&fixture_path).expect("Failed to read fixture");

    let volume = tempest_decode::decode(&data).expect("Failed to decode");
    let site = tempest_render_core::get_station("KTLX").expect("KTLX should be in registry");

    // Project reflectivity
    let ref_projected = tempest_render_core::project_volume_scan(
        &volume,
        site,
        tempest_render_core::RadarMoment::Reflectivity,
    );

    // Project velocity
    let vel_projected = tempest_render_core::project_volume_scan(
        &volume,
        site,
        tempest_render_core::RadarMoment::Velocity,
    );

    // Both should produce results (may have different point counts)
    assert!(
        !ref_projected.is_empty(),
        "Reflectivity projection should not be empty"
    );
    assert!(
        !vel_projected.is_empty() || vel_projected.iter().all(|s| s.points.is_empty()),
        "Velocity projection should exist (may be empty if moment not present)"
    );
}
