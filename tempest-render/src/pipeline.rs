//! Render pipeline definitions for Tempest radar visualization.
//!
//! This module provides the vertex format definitions and shader code
//! for rendering radar data on the GPU.

use wgpu::VertexAttribute;
use wgpu::VertexFormat;

/// The stride (in bytes) of a single radar vertex.
pub const RADAR_VERTEX_STRIDE: wgpu::BufferAddress =
    std::mem::size_of::<RadarVertex>() as wgpu::BufferAddress;

/// The stride (in bytes) of a polar radar vertex.
pub const POLAR_RADAR_VERTEX_STRIDE: wgpu::BufferAddress =
    std::mem::size_of::<PolarRadarVertex>() as wgpu::BufferAddress;

/// Vertex format for radar data points.
///
/// This vertex format is designed for polar coordinate radar data,
/// where each point is defined by:
/// - Position (x, y) in normalized device coordinates
/// - Color (RGBA) for the radar value
///
/// The position will be computed from polar coordinates (range, azimuth)
/// in the vertex shader for full polar rendering.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RadarVertex {
    /// X position in normalized device coordinates (-1 to 1).
    pub x: f32,
    /// Y position in normalized device coordinates (-1 to 1).
    pub y: f32,
    /// Red component of the vertex color (0 to 1).
    pub r: f32,
    /// Green component of the vertex color (0 to 1).
    pub g: f32,
    /// Blue component of the vertex color (0 to 1).
    pub b: f32,
    /// Alpha component of the vertex color (0 to 1).
    pub a: f32,
}

impl RadarVertex {
    /// Creates a new radar vertex with the given position and color.
    #[inline]
    pub fn new(x: f32, y: f32, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { x, y, r, g, b, a }
    }

    /// Creates a new radar vertex with a white color.
    #[inline]
    pub fn with_position(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

/// Value type for radar data.
///
/// This enum represents the different types of radar values
/// that can be rendered, determining the color mapping used.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RadarValueType {
    /// Reflectivity value in dBZ (decibels relative to Z)
    #[default]
    Reflectivity,
    /// Velocity in knots (radial velocity toward/away from radar)
    Velocity,
    /// Spectrum width in m/s
    SpectrumWidth,
    /// Differential reflectivity in dB
    Zdr,
    /// Correlation coefficient (0-1)
    Cc,
    /// Differential phase in degrees
    Kdp,
}

impl RadarValueType {
    /// Converts the value type to a float for shader upload.
    ///
    /// - Reflectivity = 0.0
    /// - Velocity = 1.0
    /// - SpectrumWidth = 2.0
    /// - Zdr = 3.0
    /// - Cc = 4.0
    /// - Kdp = 5.0
    #[inline]
    pub fn to_f32(&self) -> f32 {
        match self {
            RadarValueType::Reflectivity => 0.0,
            RadarValueType::Velocity => 1.0,
            RadarValueType::SpectrumWidth => 2.0,
            RadarValueType::Zdr => 3.0,
            RadarValueType::Cc => 4.0,
            RadarValueType::Kdp => 5.0,
        }
    }

    /// Creates a RadarValueType from a float value.
    #[inline]
    pub fn from_f32(value: f32) -> Self {
        match value.round() as u32 {
            0 => RadarValueType::Reflectivity,
            1 => RadarValueType::Velocity,
            2 => RadarValueType::SpectrumWidth,
            3 => RadarValueType::Zdr,
            4 => RadarValueType::Cc,
            5 => RadarValueType::Kdp,
            _ => RadarValueType::Reflectivity,
        }
    }
}

/// Vertex format for polar coordinate radar data.
///
/// This vertex format is designed for polar coordinate radar data where
/// each point is defined by range and azimuth angles. The vertex shader
/// transforms these polar coordinates to Cartesian positions for rendering.
///
/// The expected WGSL shader expects:
/// - range: Distance from radar in meters
/// - azimuth: Angle in degrees (0 = North, clockwise)
/// - value: Radar measurement value (dBZ, velocity, etc.)
/// - value_type: Type of radar value (0=reflectivity, 1=velocity, etc.)
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PolarRadarVertex {
    /// Range distance from radar in meters.
    pub range: f32,
    /// Azimuth angle in degrees (0 = North, clockwise direction).
    pub azimuth: f32,
    /// Radar measurement value (e.g., dBZ for reflectivity, knots for velocity).
    pub value: f32,
    /// Type of radar value (0.0 = reflectivity, 1.0 = velocity, etc.).
    pub value_type: f32,
}

impl PolarRadarVertex {
    /// Creates a new polar radar vertex.
    #[inline]
    pub fn new(range: f32, azimuth: f32, value: f32, value_type: RadarValueType) -> Self {
        Self {
            range,
            azimuth,
            value,
            value_type: value_type.to_f32(),
        }
    }

    /// Creates a new polar radar vertex for reflectivity data.
    #[inline]
    pub fn with_reflectivity(range: f32, azimuth: f32, dbz: f32) -> Self {
        Self {
            range,
            azimuth,
            value: dbz,
            value_type: RadarValueType::Reflectivity.to_f32(),
        }
    }

    /// Creates a new polar radar vertex for velocity data.
    #[inline]
    pub fn with_velocity(range: f32, azimuth: f32, velocity: f32) -> Self {
        Self {
            range,
            azimuth,
            value: velocity,
            value_type: RadarValueType::Velocity.to_f32(),
        }
    }
}

/// Describes how to map vertex attributes to GPU buffer locations.
pub fn vertex_attributes() -> [VertexAttribute; 2] {
    // Position attribute at location 0
    let position = VertexAttribute {
        format: VertexFormat::Float32x2,
        offset: 0,
        shader_location: 0,
    };

    // Color attribute at location 1
    let color = VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: 8, // After the 2 f32s for position (2 * 4 = 8 bytes)
        shader_location: 1,
    };

    [position, color]
}

/// Describes how to map polar radar vertex attributes to GPU buffer locations.
///
/// This matches the WGSL RadarVertexInput struct:
/// - location(0): range (f32)
/// - location(1): azimuth (f32)
/// - location(2): value (f32)
/// - location(3): value_type (f32)
pub fn polar_vertex_attributes() -> [VertexAttribute; 4] {
    // Range attribute at location 0
    let range = VertexAttribute {
        format: VertexFormat::Float32,
        offset: 0,
        shader_location: 0,
    };

    // Azimuth attribute at location 1
    let azimuth = VertexAttribute {
        format: VertexFormat::Float32,
        offset: 4, // After range (1 * 4 bytes)
        shader_location: 1,
    };

    // Value attribute at location 2
    let value = VertexAttribute {
        format: VertexFormat::Float32,
        offset: 8, // After range + azimuth (2 * 4 bytes)
        shader_location: 2,
    };

    // Value type attribute at location 3
    let value_type = VertexAttribute {
        format: VertexFormat::Float32,
        offset: 12, // After range + azimuth + value (3 * 4 bytes)
        shader_location: 3,
    };

    [range, azimuth, value, value_type]
}

/// WGSL shader module for basic radar rendering.
///
/// This is a pass-through shader that takes vertex position and color
/// as input and outputs them directly to the fragment stage.
pub const BASIC_SHADER_WGSL: &str = r#"
/// Vertex shader input from the vertex buffer.
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

/// Output from vertex shader to fragment shader.
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

/// Vertex shader entry point.
///
/// Transforms input vertex position to clip space and passes
/// the color through to the fragment shader.
@vertex
fn vertex_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

/// Fragment shader entry point.
///
/// Outputs the interpolated color directly to the framebuffer.
@fragment
fn fragment_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;

/// Vertex shader module for radar rendering with polar coordinates.
///
/// This shader transforms polar coordinates (range, azimuth) to
/// Cartesian positions for rendering radar sweeps.
/// Includes NWS standard color mapping for reflectivity values.
pub const POLAR_SHADER_WGSL: &str = r#"
/// Vertex input with polar coordinates and color.
struct RadarVertexInput {
    @location(0) range: f32,
    @location(1) azimuth: f32,
    @location(2) value: f32,
    @location(3) value_type: f32,
}

/// Uniforms for radar rendering configuration.
struct RadarUniforms {
    /// Center X position (normalized -1 to 1).
    center_x: f32,
    /// Center Y position (normalized -1 to 1).
    center_y: f32,
    /// Scale factor for range (meters per unit).
    range_scale: f32,
    /// Aspect ratio correction.
    aspect_ratio: f32,
    /// Current sweep rotation angle in degrees.
    sweep_rotation: f32,
    /// Opacity multiplier for radar overlay (0.0-1.0).
    opacity: f32,
    /// Padding for alignment.
    _padding: vec2<f32>,
}

/// Vertex output to fragment shader.
struct RadarVertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) value: f32,
    @location(1) value_type: f32,
}

/// NWS Standard Radar Color Mapping for Reflectivity (dBZ)
/// Returns RGB color for a given dBZ value using NWS standard colors.
/// 
/// Color stops (dBZ -> RGB):
/// - Below 5: Transparent (no precipitation)
/// - 5: Light green
/// - 10: Green
/// - 15: Dark green
/// - 20: Yellow
/// - 25: Dark yellow
/// - 30: Red
/// - 35: Dark red
/// - 40: Magenta
/// - 45: Purple
/// - 50+: White (hail)
fn dbz_to_color(dbz: f32) -> vec4<f32> {
    // No precipitation detected - transparent
    if (dbz < 5.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // Light green (5-10 dBZ - Light precipitation)
    if (dbz < 10.0) {
        let t = (dbz - 5.0) / 5.0;
        return vec4<f32>(0.2, 0.8, 0.2, 0.3 + 0.4 * t);
    }
    
    // Green (10-15 dBZ - Light to moderate)
    if (dbz < 15.0) {
        let t = (dbz - 10.0) / 5.0;
        return vec4<f32>(0.0, 0.7, 0.0, 0.7);
    }
    
    // Dark green (15-20 dBZ - Moderate)
    if (dbz < 20.0) {
        let t = (dbz - 15.0) / 5.0;
        return vec4<f32>(0.0, 0.5, 0.0, 0.8);
    }
    
    // Yellow (20-25 dBZ - Moderate to heavy)
    if (dbz < 25.0) {
        let t = (dbz - 20.0) / 5.0;
        return vec4<f32>(1.0, 1.0, 0.0, 0.85);
    }
    
    // Dark yellow (25-30 dBZ - Heavy)
    if (dbz < 30.0) {
        let t = (dbz - 25.0) / 5.0;
        return vec4<f32>(0.8, 0.8, 0.0, 0.9);
    }
    
    // Red (30-35 dBZ - Very heavy)
    if (dbz < 35.0) {
        let t = (dbz - 30.0) / 5.0;
        return vec4<f32>(1.0, 0.0, 0.0, 0.92);
    }
    
    // Dark red (35-40 dBZ - Extreme)
    if (dbz < 40.0) {
        let t = (dbz - 35.0) / 5.0;
        return vec4<f32>(0.7, 0.0, 0.0, 0.95);
    }
    
    // Magenta (40-45 dBZ - Severe hail)
    if (dbz < 45.0) {
        let t = (dbz - 40.0) / 5.0;
        return vec4<f32>(0.8, 0.0, 0.8, 0.97);
    }
    
    // Purple (45-50 dBZ - Intense hail)
    if (dbz < 50.0) {
        let t = (dbz - 45.0) / 5.0;
        return vec4<f32>(0.5, 0.0, 0.5, 1.0);
    }
    
    // White (50+ dBZ - Large hail / debris)
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}

/// NWS Standard Radar Color Mapping for Velocity
/// Returns RGB color for velocity values (knots).
/// 
/// Color scheme: Blue (outbound) to Green (near 0) to Red (inbound)
fn velocity_to_color(velocity: f32) -> vec4<f32> {
    // Velocity range typically -100 to +100 knots
    let max_velocity = 100.0;
    let normalized = clamp(velocity / max_velocity, -1.0, 1.0);
    
    // Outbound (negative) - blue shades
    if (normalized < 0.0) {
        let intensity = -normalized;
        // Deep blue to light blue
        return vec4<f32>(0.0, 0.3 * intensity, 1.0 * intensity, 0.7 + 0.3 * intensity);
    }
    
    // Near zero - green
    if (normalized < 0.1) {
        return vec4<f32>(0.0, 1.0, 0.0, 0.8);
    }
    
    // Inbound (positive) - red shades
    let intensity = normalized;
    // Light red to deep red
    return vec4<f32>(1.0 * intensity, 0.0, 0.0, 0.7 + 0.3 * intensity);
}

/// NWS Standard Radar Color Mapping for Spectrum Width
/// Returns RGB color for spectrum width values (m/s).
/// 
/// Color scheme: Light blue (low) to Dark blue (high)
fn spectrum_width_to_color(sw: f32) -> vec4<f32> {
    // Spectrum width range: 0-30 m/s
    let max_sw = 30.0;
    let normalized = clamp(sw / max_sw, 0.0, 1.0);
    
    // Below threshold - transparent
    if (sw < 0.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // Light blue to dark blue gradient
    return vec4<f32>(0.2 * normalized, 0.4 * normalized, 1.0 * normalized, 0.5 + 0.5 * normalized);
}

/// NWS Standard Radar Color Mapping for ZDR (Differential Reflectivity)
/// Returns RGB color for ZDR values (dB).
/// 
/// Color scheme: Browns -> Oranges -> Greens -> Blues
fn zdr_to_color(zdr: f32) -> vec4<f32> {
    // ZDR range: -4 to +8 dB
    let min_zdr = -4.0;
    let max_zdr = 8.0;
    let normalized = clamp((zdr - min_zdr) / (max_zdr - min_zdr), 0.0, 1.0);
    
    // Below threshold - transparent
    if (zdr < -3.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // Browns (-4 to -1 dB)
    if (normalized < 0.25) {
        let t = normalized / 0.25;
        return vec4<f32>(0.6 + 0.2 * t, 0.4 * (1.0 - t), 0.2 * (1.0 - t), 0.6 + 0.2 * t);
    }
    
    // Oranges to Greens (-1 to 3 dB)
    if (normalized < 0.5) {
        let t = (normalized - 0.25) / 0.25;
        return vec4<f32>(0.8 * (1.0 - t), 0.4 + 0.4 * t, 0.2 * (1.0 - t), 0.8);
    }
    
    // Greens to Blues (3 to 8 dB)
    let t = (normalized - 0.5) / 0.5;
    return vec4<f32>(0.0, 0.8 * (1.0 - t), 0.8 + 0.2 * t, 0.85 + 0.15 * t);
}

/// NWS Standard Radar Color Mapping for CC (Correlation Coefficient)
/// Returns RGB color for CC values (0.0-1.0).
/// 
/// Color scheme: Red (low) -> Yellow -> Green -> Blue (high)
fn cc_to_color(cc: f32) -> vec4<f32> {
    // CC range: 0.0 to 1.0
    let normalized = clamp(cc, 0.0, 1.0);
    
    // Below threshold - transparent
    if (cc < 0.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // Red to Yellow (0.5-0.7)
    if (normalized < 0.7) {
        let t = (normalized - 0.5) / 0.2;
        return vec4<f32>(1.0, 0.5 + 0.5 * t, 0.0, 0.6 + 0.2 * t);
    }
    
    // Yellow to Green (0.7-0.85)
    if (normalized < 0.85) {
        let t = (normalized - 0.7) / 0.15;
        return vec4<f32>(1.0 - t, 1.0, 0.0, 0.8);
    }
    
    // Green to Blue (0.85-1.0)
    let t = (normalized - 0.85) / 0.15;
    return vec4<f32>(0.0, 1.0 - t, 1.0 * t, 0.85 + 0.15 * t);
}

/// NWS Standard Radar Color Mapping for KDP (Differential Phase)
/// Returns RGB color for KDP values (degrees/km).
/// 
/// Color scheme: Browns -> Oranges -> Greens -> Blues
fn kdp_to_color(kdp: f32) -> vec4<f32> {
    // KDP range: -2 to +10 deg/km
    let min_kdp = -2.0;
    let max_kdp = 10.0;
    let normalized = clamp((kdp - min_kdp) / (max_kdp - min_kdp), 0.0, 1.0);
    
    // Below threshold - transparent
    if (kdp < -1.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // Browns (-2 to 0 deg/km)
    if (normalized < 0.167) {
        let t = normalized / 0.167;
        return vec4<f32>(0.6 + 0.2 * t, 0.3 * (1.0 - t), 0.2 * (1.0 - t), 0.5 + 0.2 * t);
    }
    
    // Browns to Oranges (0 to 2 deg/km)
    if (normalized < 0.333) {
        let t = (normalized - 0.167) / 0.166;
        return vec4<f32>(0.8 + 0.1 * t, 0.1 * t, 0.0, 0.7 + 0.1 * t);
    }
    
    // Oranges to Greens (2 to 5 deg/km)
    if (normalized < 0.583) {
        let t = (normalized - 0.333) / 0.25;
        return vec4<f32>(0.9 * (1.0 - t), 0.1 + 0.6 * t, 0.0, 0.8);
    }
    
    // Greens to Blues (5 to 10 deg/km)
    let t = (normalized - 0.583) / 0.417;
    return vec4<f32>(0.0, 0.7 * (1.0 - t), 0.8 + 0.2 * t, 0.8 + 0.2 * t);
}

/// Maps a radar value to color based on value type.
/// value_type: 0.0 = reflectivity (dBZ), 1.0 = velocity (knots), 
///             2.0 = spectrum_width (m/s), 3.0 = zdr (dB),
///             4.0 = cc (0-1), 5.0 = kdp (deg/km)
fn radar_value_to_color(value: f32, value_type: f32) -> vec4<f32> {
    if (value_type < 0.5) {
        // Reflectivity (0.0)
        return dbz_to_color(value);
    } else if (value_type < 1.5) {
        // Velocity (1.0)
        return velocity_to_color(value);
    } else if (value_type < 2.5) {
        // Spectrum Width (2.0)
        return spectrum_width_to_color(value);
    } else if (value_type < 3.5) {
        // ZDR (3.0)
        return zdr_to_color(value);
    } else if (value_type < 4.5) {
        // CC (4.0)
        return cc_to_color(value);
    } else {
        // KDP (5.0)
        return kdp_to_color(value);
    }
}

/// Vertex shader for polar radar rendering.
///
/// Converts polar coordinates (range, azimuth) to Cartesian positions
/// and passes the radar value to the fragment shader for color mapping.
@vertex
fn vertex_main(
    input: RadarVertexInput,
    @uniforms(0) uniforms: RadarUniforms
) -> RadarVertexOutput {
    var output: RadarVertexOutput;
    
    // Convert azimuth from degrees to radians and add sweep rotation
    let azimuth_rad = radians(input.azimuth + uniforms.sweep_rotation);
    
    // Calculate Cartesian position from polar coordinates
    // Apply aspect ratio correction to maintain circular radar display
    let x = uniforms.center_x + input.range * uniforms.range_scale * cos(azimuth_rad) / uniforms.aspect_ratio;
    let y = uniforms.center_y + input.range * uniforms.range_scale * sin(azimuth_rad);
    
    output.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    output.value = input.value;
    output.value_type = input.value_type;
    
    return output;
}

/// Fragment shader for radar rendering.
///
/// Applies NWS standard color mapping based on radar value type.
/// Multiplies the color alpha by the opacity uniform.
@fragment
fn fragment_main(
    input: RadarVertexOutput,
    @uniforms(0) uniforms: RadarUniforms
) -> @location(0) vec4<f32> {
    let color = radar_value_to_color(input.value, input.value_type);
    // Apply opacity uniform to the color alpha
    return vec4<f32>(color.rgb, color.a * uniforms.opacity);
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        // Verify the vertex structure size matches expected stride
        let size = std::mem::size_of::<RadarVertex>();
        assert_eq!(size, 24, "RadarVertex should be 24 bytes (6 * f32)");
    }

    #[test]
    fn test_vertex_attributes() {
        let attrs = vertex_attributes();

        // Check position attribute
        assert_eq!(attrs[0].shader_location, 0);
        assert_eq!(attrs[0].format, VertexFormat::Float32x2);
        assert_eq!(attrs[0].offset, 0);

        // Check color attribute
        assert_eq!(attrs[1].shader_location, 1);
        assert_eq!(attrs[1].format, VertexFormat::Float32x4);
        assert_eq!(attrs[1].offset, 8);
    }

    #[test]
    fn test_polar_vertex_size() {
        // Verify the polar vertex structure size matches expected stride
        let size = std::mem::size_of::<PolarRadarVertex>();
        assert_eq!(size, 16, "PolarRadarVertex should be 16 bytes (4 * f32)");
    }

    #[test]
    fn test_polar_vertex_attributes() {
        let attrs = polar_vertex_attributes();

        // Check range attribute
        assert_eq!(attrs[0].shader_location, 0);
        assert_eq!(attrs[0].format, VertexFormat::Float32);
        assert_eq!(attrs[0].offset, 0);

        // Check azimuth attribute
        assert_eq!(attrs[1].shader_location, 1);
        assert_eq!(attrs[1].format, VertexFormat::Float32);
        assert_eq!(attrs[1].offset, 4);

        // Check value attribute
        assert_eq!(attrs[2].shader_location, 2);
        assert_eq!(attrs[2].format, VertexFormat::Float32);
        assert_eq!(attrs[2].offset, 8);

        // Check value_type attribute
        assert_eq!(attrs[3].shader_location, 3);
        assert_eq!(attrs[3].format, VertexFormat::Float32);
        assert_eq!(attrs[3].offset, 12);
    }

    #[test]
    fn test_radar_value_type_conversion() {
        assert_eq!(RadarValueType::Reflectivity.to_f32(), 0.0);
        assert_eq!(RadarValueType::Velocity.to_f32(), 1.0);
        assert_eq!(RadarValueType::SpectrumWidth.to_f32(), 2.0);
        assert_eq!(RadarValueType::Zdr.to_f32(), 3.0);
        assert_eq!(RadarValueType::Cc.to_f32(), 4.0);
        assert_eq!(RadarValueType::Kdp.to_f32(), 5.0);
    }

    #[test]
    fn test_radar_value_type_from_f32() {
        assert_eq!(RadarValueType::from_f32(0.0), RadarValueType::Reflectivity);
        assert_eq!(RadarValueType::from_f32(1.0), RadarValueType::Velocity);
        assert_eq!(RadarValueType::from_f32(2.0), RadarValueType::SpectrumWidth);
        assert_eq!(RadarValueType::from_f32(5.0), RadarValueType::Kdp);
        // Invalid values default to Reflectivity
        assert_eq!(RadarValueType::from_f32(99.0), RadarValueType::Reflectivity);
    }

    #[test]
    fn test_polar_vertex_creation() {
        let vertex = PolarRadarVertex::new(1000.0, 45.0, 30.0, RadarValueType::Reflectivity);
        assert_eq!(vertex.range, 1000.0);
        assert_eq!(vertex.azimuth, 45.0);
        assert_eq!(vertex.value, 30.0);
        assert_eq!(vertex.value_type, 0.0); // Reflectivity

        let vertex2 = PolarRadarVertex::with_velocity(2000.0, 90.0, 50.0);
        assert_eq!(vertex2.value_type, 1.0); // Velocity
    }
}
