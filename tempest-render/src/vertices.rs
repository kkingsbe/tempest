//! Vertex conversion utilities for radar data.
//!
//! This module provides functions to convert decoded radar data (from tempest-decode)
//! into vertex buffers suitable for GPU rendering.
//!
//! The conversion handles:
//! - Polar coordinate transformation (range bins → vertices)
//! - Radar value type mapping
//! - Gate filtering (removing invalid/no-data values)

use crate::pipeline::PolarRadarVertex;

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
