//! Vertex conversion utilities for radar data.
//!
//! This module provides functions to convert decoded radar data (from tempest-decode)
//! into vertex buffers suitable for GPU rendering.
//!
//! The conversion handles:
//! - Polar coordinate transformation (range bins → vertices)
//! - Radar value type mapping
//! - Gate filtering (removing invalid/no-data values)

use crate::pipeline::{PolarRadarVertex, RadarVertex};
use crate::view_transform::ViewTransform;
use tempest_render_core::{
    colorize, ProjectedSweep as CoreProjectedSweep, RadarMoment, RadarSentinel,
};

/// Error types for vertex conversion operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VertexConversionError {
    /// No valid gates found in the radial data.
    NoValidGates,
    /// Invalid gate data encountered.
    InvalidGateData,
}

/// Result type for vertex conversion operations.
pub type VertexResult<T> = Result<T, VertexConversionError>;

/// Configuration for radar-to-vertex conversion.
#[derive(Debug, Clone)]
pub struct RadarVertexConfig {
    /// Minimum reflectivity threshold in dBZ.
    /// Gates below this value will be filtered out.
    pub reflectivity_threshold: f32,

    /// Maximum range in meters.
    /// Gates beyond this range will be filtered out.
    pub max_range_meters: f32,

    /// Whether to include velocity data.
    pub include_velocity: bool,

    /// Whether to normalize azimuth angles (0-360 to -180 to 180).
    pub normalize_azimuth: bool,
}

impl Default for RadarVertexConfig {
    /// Creates a default configuration.
    ///
    /// Default settings:
    /// - reflectivity_threshold: 5.0 dBZ (below = no precipitation)
    /// - max_range_meters: 460,000 m (typical NEXRAD range)
    /// - include_velocity: true
    /// - normalize_azimuth: false
    fn default() -> Self {
        Self {
            reflectivity_threshold: 5.0,
            max_range_meters: 460_000.0,
            include_velocity: true,
            normalize_azimuth: false,
        }
    }
}

/// Converts a single radial (one azimuth sweep of gates) to vertices.
///
/// This function iterates over all gates in a radial and creates a vertex
/// for each gate that passes the filtering criteria.
///
/// # Arguments
///
/// * `azimuth` - Azimuth angle in degrees
/// * `gates` - Slice of gate data
/// * `config` - Conversion configuration
///
/// # Returns
///
/// A vector of vertices for all valid gates in the radial.
///
/// # Example
///
/// ```ignore
/// use tempest_render::vertices::{radial_to_vertices, RadarVertexConfig};
/// use tempest_decode::{Radial, Gate};
///
/// let config = RadarVertexConfig::default();
/// let radial = Radial::new(45.0);
/// // ... populate radial with gates ...
/// let vertices = radial_to_vertices(radial.azimuth, &radial.gates, &config);
/// ```
pub fn radial_to_vertices(
    azimuth: f32,
    gates: &[Gate],
    config: &RadarVertexConfig,
) -> VertexResult<Vec<PolarRadarVertex>> {
    let mut vertices = Vec::with_capacity(gates.len());

    for gate in gates {
        // Filter by range
        if gate.range > config.max_range_meters {
            continue;
        }

        // Get reflectivity value if available
        if let Some(reflectivity) = gate.reflectivity {
            // Filter by reflectivity threshold
            if reflectivity >= config.reflectivity_threshold {
                vertices.push(PolarRadarVertex::with_reflectivity(
                    gate.range,
                    azimuth,
                    reflectivity,
                ));
            }
        }

        // Include velocity if configured
        if config.include_velocity {
            if let Some(velocity) = gate.velocity {
                // Velocity can be negative (outbound) or positive (inbound)
                // Include all valid velocity readings
                vertices.push(PolarRadarVertex::with_velocity(
                    gate.range, azimuth, velocity,
                ));
            }
        }
    }

    if vertices.is_empty() {
        return Err(VertexConversionError::NoValidGates);
    }

    Ok(vertices)
}

/// Converts a sweep (collection of radials) to vertices.
///
/// This function iterates over all radials in a sweep and creates vertices
/// for each radial's gates.
///
/// # Arguments
///
/// * `sweep` - The sweep containing radials
/// * `config` - Conversion configuration
///
/// # Returns
///
/// A vector of vertices for all valid gates in the sweep.
pub fn sweep_to_vertices(
    sweep: &Sweep,
    config: &RadarVertexConfig,
) -> VertexResult<Vec<PolarRadarVertex>> {
    let mut vertices = Vec::new();

    for radial in &sweep.radials {
        let radial_vertices = radial_to_vertices(radial.azimuth, &radial.gates, config);
        if let Ok(mut radial_verts) = radial_vertices {
            vertices.append(&mut radial_verts);
        }
    }

    if vertices.is_empty() {
        return Err(VertexConversionError::NoValidGates);
    }

    Ok(vertices)
}

/// Converts a volume scan (collection of sweeps) to vertices.
///
/// This function converts all sweeps in the volume to vertices.
/// If multiple sweeps are present, all are included in the output.
///
/// # Arguments
///
/// * `volume` - The volume scan containing sweeps
/// * `config` - Conversion configuration
///
/// # Returns
///
/// A vector of vertices for all valid gates in all sweeps.
pub fn volume_to_vertices(
    volume: &VolumeScan,
    config: &RadarVertexConfig,
) -> VertexResult<Vec<PolarRadarVertex>> {
    let mut vertices = Vec::new();

    for sweep in &volume.sweeps {
        let sweep_vertices = sweep_to_vertices(sweep, config);
        if let Ok(mut sweep_verts) = sweep_vertices {
            vertices.append(&mut sweep_verts);
        }
    }

    if vertices.is_empty() {
        return Err(VertexConversionError::NoValidGates);
    }

    Ok(vertices)
}

/// Converts a projected sweep (with lat/lng coordinates) to render vertices.
///
/// This function takes a projected sweep from `tempest-render-core` (which contains
/// geographic coordinates) and converts it to render vertices using the ViewTransform
/// to convert geographic coordinates to NDC (Normalized Device Coordinates).
///
/// The resulting vertices use `RadarVertex` format with pre-transformed positions
/// in NDC, suitable for rendering with `BASIC_SHADER_WGSL`.
///
/// # Arguments
///
/// * `projected_sweep` - The projected sweep containing lat/lng coordinates and values
/// * `view` - The view transform for converting lat/lng to NDC
/// * `moment` - The radar moment type for color mapping (Reflectivity, Velocity, etc.)
///
/// # Returns
///
/// A vector of `RadarVertex` with positions in NDC and colors from the color mapping.
///
/// # Example
///
/// ```ignore
/// use tempest_render::vertices::projected_sweep_to_vertices;
/// use tempest_render::view_transform::ViewTransform;
/// use tempest_render_core::{project_sweep, get_station, RadarMoment};
/// use tempest_decode::{Sweep, Radial, Gate};
///
/// // Create a test sweep
/// let mut sweep = Sweep::new(0.5);
/// let mut radial = Radial::new(90.0);
/// let mut gate = Gate::new(10000.0);
/// gate.reflectivity = Some(30.0);
/// radial.gates.push(gate);
/// sweep.radials.push(radial);
///
/// let site = get_station("KTLX").unwrap();
/// let projected = project_sweep(&site, &sweep, RadarMoment::Reflectivity);
///
/// let view = ViewTransform::new(site.lat, site.lon, 1.0, 16.0 / 9.0);
/// let vertices = projected_sweep_to_vertices(&projected, &view, RadarMoment::Reflectivity);
/// assert!(!vertices.is_empty());
/// ```
pub fn projected_sweep_to_vertices(
    projected_sweep: &CoreProjectedSweep,
    view: &ViewTransform,
    moment: RadarMoment,
) -> Vec<RadarVertex> {
    let mut vertices = Vec::with_capacity(projected_sweep.points.len());

    for point in &projected_sweep.points {
        // Convert lat/lng to NDC using the view transform
        let (x, y) = view.to_clip_space(point.lat, point.lng);

        // Get color from the color mapping based on radar moment and value
        let rgba = colorize(moment, point.value, RadarSentinel::Valid);

        // Convert RGBA (0-255) to normalized float (0.0-1.0)
        let r = rgba.r as f32 / 255.0;
        let g = rgba.g as f32 / 255.0;
        let b = rgba.b as f32 / 255.0;
        let a = rgba.a as f32 / 255.0;

        vertices.push(RadarVertex::new(x, y, r, g, b, a));
    }

    vertices
}

/// Creates a simple test sweep with synthetic radar data.
///
/// This function is useful for testing and development purposes
/// when real radar data is not available.
pub fn create_test_sweep(num_radials: usize, gates_per_radial: usize) -> Sweep {
    use tempest_decode::Sweep as RadarSweep;

    let mut sweep = RadarSweep::new(0.5);

    for i in 0..num_radials {
        let azimuth = (i as f32 / num_radials as f32) * 360.0;
        let mut radial = Radial::new(azimuth);

        for j in 0..gates_per_radial {
            let range = 1000.0 + (j as f32 * 1000.0); // Gates every 1km
            let mut gate = Gate::new(range);

            // Create synthetic reflectivity: increases with range
            let reflectivity = 30.0 + (azimuth / 10.0) + (j as f32 / 10.0);
            gate.reflectivity = Some(reflectivity.min(75.0));

            // Add synthetic velocity for half the gates
            if j % 2 == 0 {
                let velocity = ((azimuth / 90.0) - 1.0) * 50.0; // -50 to +50 knots
                gate.velocity = Some(velocity);
            }

            radial.gates.push(gate);
        }

        sweep.radials.push(radial);
    }

    sweep
}

/// Re-exports from tempest-decode for convenience.
// Re-export the types needed for vertex conversion
pub use tempest_decode::{Gate, Radial, Sweep, VolumeScan};

#[cfg(test)]
mod tests {
    use super::*;
    use tempest_render_core::{ProjectedPoint, ProjectedSweep};

    /// Test that projected_sweep_to_vertices produces valid vertices for a single point.
    #[test]
    fn test_projected_sweep_to_vertices_single_point() {
        // Create a projected sweep with a single point at the radar site location
        let site_lat = 35.4183;
        let site_lng = -97.4514;

        let projected_sweep =
            ProjectedSweep::new(0.5, vec![ProjectedPoint::new(site_lat, site_lng, 30.0)]);

        // Create view centered on the radar site
        let view = ViewTransform::new(site_lat, site_lng, 1.0, 1.0);

        let vertices =
            projected_sweep_to_vertices(&projected_sweep, &view, RadarMoment::Reflectivity);

        // Should have one vertex
        assert_eq!(vertices.len(), 1);

        // Center point should be at origin (0, 0) in NDC
        assert!(
            (vertices[0].x).abs() < 0.0001,
            "x should be ~0 for center point"
        );
        assert!(
            (vertices[0].y).abs() < 0.0001,
            "y should be ~0 for center point"
        );
    }

    /// Test that projected_sweep_to_vertices handles empty sweep.
    #[test]
    fn test_projected_sweep_to_vertices_empty() {
        let site_lat = 35.4183;
        let site_lng = -97.4514;

        let projected_sweep = ProjectedSweep::new(0.5, vec![]);

        let view = ViewTransform::new(site_lat, site_lng, 1.0, 1.0);

        let vertices =
            projected_sweep_to_vertices(&projected_sweep, &view, RadarMoment::Reflectivity);

        // Should have no vertices
        assert!(vertices.is_empty());
    }

    /// Test that projected_sweep_to_vertices correctly offsets points from center.
    #[test]
    fn test_projected_sweep_to_vertices_offsets() {
        let site_lat = 35.4183;
        let site_lng = -97.4514;

        // Create a point 1 degree east (higher longitude)
        let point_east = ProjectedPoint::new(site_lat, site_lng + 1.0, 30.0);

        // Create a point 1 degree north (higher latitude)
        let point_north = ProjectedPoint::new(site_lat + 1.0, site_lng, 30.0);

        let projected_sweep = ProjectedSweep::new(0.5, vec![point_east, point_north]);

        let view = ViewTransform::new(site_lat, site_lng, 1.0, 1.0);

        let vertices =
            projected_sweep_to_vertices(&projected_sweep, &view, RadarMoment::Reflectivity);

        assert_eq!(vertices.len(), 2);

        // East point should have positive x
        assert!(vertices[0].x > 0.0, "East point should have positive x");
        // North point should have positive y
        assert!(vertices[1].y > 0.0, "North point should have positive y");
    }

    /// Test that zoom affects the vertex positions correctly.
    #[test]
    fn test_projected_sweep_to_vertices_zoom() {
        let site_lat = 35.4183;
        let site_lng = -97.4514;

        // Point 1 degree east
        let point = ProjectedPoint::new(site_lat, site_lng + 1.0, 30.0);
        let projected_sweep = ProjectedSweep::new(0.5, vec![point]);

        // With zoom 1.0
        let view1 = ViewTransform::new(site_lat, site_lng, 1.0, 1.0);
        let vertices1 =
            projected_sweep_to_vertices(&projected_sweep, &view1, RadarMoment::Reflectivity);

        // With zoom 2.0 (more zoomed in)
        let view2 = ViewTransform::new(site_lat, site_lng, 2.0, 1.0);
        let vertices2 =
            projected_sweep_to_vertices(&projected_sweep, &view2, RadarMoment::Reflectivity);

        // With higher zoom, the NDC distance should be half
        assert!(
            (vertices2[0].x - vertices1[0].x * 0.5).abs() < 0.001,
            "Zoom 2.0 should give half the NDC distance"
        );
    }

    /// Test that velocity moment produces valid vertices.
    #[test]
    fn test_projected_sweep_to_vertices_velocity() {
        let site_lat = 35.4183;
        let site_lng = -97.4514;

        // Create a point with velocity value
        let point = ProjectedPoint::new(site_lat, site_lng, 25.0); // 25 m/s velocity
        let projected_sweep = ProjectedSweep::new(0.5, vec![point]);

        let view = ViewTransform::new(site_lat, site_lng, 1.0, 1.0);

        let vertices = projected_sweep_to_vertices(&projected_sweep, &view, RadarMoment::Velocity);

        assert_eq!(vertices.len(), 1);
        // Velocity should produce a red-ish color (positive velocity = inbound/red)
        assert!(
            vertices[0].r > 0.0,
            "Velocity should produce red color component"
        );
    }
}
