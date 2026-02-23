//! Visual test utilities for radar rendering.
//!
//! This module provides utilities for rendering color gradient strips
//! to verify color table implementations visually.

use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::Path;

/// Render a color gradient strip for a radar moment type.
///
/// This function creates a horizontal gradient image showing the full
/// color range for a given radar moment, which is useful for visual
/// verification of color table implementations.
///
/// # Arguments
/// * `moment_type` - The radar moment code: "REF", "VEL", "SW", "ZDR", "CC", or "KDP"
/// * `width` - Width of the output image in pixels
/// * `height` - Height of the output image in pixels
///
/// # Returns
/// * `Vec<u8>` - Raw RGBA pixel data (width * height * 4 bytes)
///
/// # Panics
/// Panics if `moment_type` is not recognized.
pub fn render_color_table_as_image(moment_type: &str, width: u32, height: u32) -> Vec<u8> {
    let (min_val, max_val) = get_value_range(moment_type);
    let color_table_fn = get_color_table_fn(moment_type);

    let mut img: RgbaImage = ImageBuffer::new(width, height);

    for x in 0..width {
        // Map x position to radar value
        let t = x as f32 / (width - 1) as f32;
        let value = min_val + t * (max_val - min_val);

        // Get the color from the color table
        let color = color_table_fn(value);

        // Fill the column with this color
        for y in 0..height {
            img.put_pixel(x, y, color);
        }
    }

    // Convert to raw RGBA bytes
    let raw_pixels: Vec<u8> = img.into_raw();
    raw_pixels
}

/// Get the value range (min, max) for a radar moment type.
fn get_value_range(moment_type: &str) -> (f32, f32) {
    match moment_type {
        "REF" => (-30.0, 75.0),   // Reflectivity: dBZ
        "VEL" => (-100.0, 100.0), // Velocity: m/s
        "SW" => (0.0, 20.0),      // Spectrum Width: m/s
        "ZDR" => (-7.5, 7.5),     // ZDR: dB
        "CC" => (0.0, 1.0),       // Correlation Coefficient: 0-1
        "KDP" => (0.0, 10.0),     // KDP: degrees/km
        _ => panic!("Unknown radar moment type: {}", moment_type),
    }
}

/// Get the color table function for a radar moment type.
fn get_color_table_fn(moment_type: &str) -> impl Fn(f32) -> Rgba<u8> {
    // Use the color table functions from the render core
    use tempest_render_core::{colorize, RadarMoment, RadarSentinel};

    match moment_type {
        "REF" => {
            |value: f32| -> Rgba<u8> {
                let c = colorize(RadarMoment::Reflectivity, value, RadarSentinel::Valid);
                Rgba([c.r, c.g, c.b, c.a])
            }
        }
        "VEL" => {
            |value: f32| -> Rgba<u8> {
                let c = colorize(RadarMoment::Velocity, value, RadarSentinel::Valid);
                Rgba([c.r, c.g, c.b, c.a])
            }
        }
        "SW" => {
            |value: f32| -> Rgba<u8> {
                let c = colorize(RadarMoment::SpectrumWidth, value, RadarSentinel::Valid);
                Rgba([c.r, c.g, c.b, c.a])
            }
        }
        "ZDR" => {
            |value: f32| -> Rgba<u8> {
                let c = colorize(RadarMoment::Zdr, value, RadarSentinel::Valid);
                Rgba([c.r, c.g, c.b, c.a])
            }
        }
        "CC" => {
            |value: f32| -> Rgba<u8> {
                let c = colorize(RadarMoment::Cc, value, RadarSentinel::Valid);
                Rgba([c.r, c.g, c.b, c.a])
            }
        }
        "KDP" => {
            |value: f32| -> Rgba<u8> {
                let c = colorize(RadarMoment::Kdp, value, RadarSentinel::Valid);
                Rgba([c.r, c.g, c.b, c.a])
            }
        }
        _ => panic!("Unknown radar moment type: {}", moment_type),
    }
}

/// Render and save a color gradient image for testing.
///
/// This is a convenience function that renders the gradient and returns
/// the raw bytes, useful for insta snapshot testing.
///
/// # Arguments
/// * `moment_type` - The radar moment code
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
///
/// # Returns
/// * `Vec<u8>` - Raw RGBA pixel data
pub fn render_gradient_for_test(moment_type: &str, width: u32, height: u32) -> Vec<u8> {
    render_color_table_as_image(moment_type, width, height)
}

/// Save image bytes to a PNG file for debugging.
///
/// This is useful for manually inspecting rendered images during test development.
///
/// # Arguments
/// * `pixels` - Raw RGBA pixel data
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `path` - Output file path
pub fn save_png(pixels: &[u8], width: u32, height: u32, path: &Path) -> Result<(), String> {
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
        .ok_or_else(|| "Failed to create image from raw pixels".to_string())?;

    img.save(path)
        .map_err(|e| format!("Failed to save PNG: {}", e))
}

/// Compare rendered pixels against an insta snapshot.
///
/// This function creates a snapshot from the rendered pixels and
/// compares it against any existing snapshot.
///
/// # Arguments
/// * `pixels` - Raw RGBA pixel data
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `name` - Snapshot name (typically test function name)
///
/// # Returns
/// * `Result<Snapshot, String>` - The snapshot result
pub fn assert_snapshot(pixels: &[u8], width: u32, height: u32, name: &str) -> Result<Snapshot, String> {
    // Create a PNG from the raw pixels
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
        .ok_or_else(|| "Failed to create image from raw pixels".to_string())?;

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut png_bytes);
    img.write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    // Use insta to create and compare snapshot
    let snapshot = Snapshot::from_json(name, &png_bytes);
    Ok(snapshot)
}

/// List of supported radar moment types for testing.
pub const SUPPORTED_MOMENTS: &[&str] = &["REF", "VEL", "SW", "ZDR", "CC", "KDP"];

/// Get the display name for a radar moment type.
pub fn moment_display_name(moment_type: &str) -> &'static str {
    match moment_type {
        "REF" => "Reflectivity (dBZ)",
        "VEL" => "Velocity (m/s)",
        "SW" => "Spectrum Width (m/s)",
        "ZDR" => "Differential Reflectivity (dB)",
        "CC" => "Correlation Coefficient",
        "KDP" => "Differential Phase (deg/km)",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_reflectivity_gradient() {
        let pixels = render_color_table_as_image("REF", 256, 32);
        assert_eq!(pixels.len(), 256 * 32 * 4);
    }

    #[test]
    fn test_render_velocity_gradient() {
        let pixels = render_color_table_as_image("VEL", 256, 32);
        assert_eq!(pixels.len(), 256 * 32 * 4);
    }

    #[test]
    fn test_render_all_moments() {
        for moment in SUPPORTED_MOMENTS {
            let pixels = render_color_table_as_image(moment, 128, 16);
            assert_eq!(pixels.len(), 128 * 16 * 4, "Failed for moment: {}", moment);
        }
    }

    #[test]
    fn test_moment_display_names() {
        assert_eq!(moment_display_name("REF"), "Reflectivity (dBZ)");
        assert_eq!(moment_display_name("VEL"), "Velocity (m/s)");
        assert_eq!(moment_display_name("SW"), "Spectrum Width (m/s)");
        assert_eq!(moment_display_name("ZDR"), "Differential Reflectivity (dB)");
        assert_eq!(moment_display_name("CC"), "Correlation Coefficient");
        assert_eq!(moment_display_name("KDP"), "Differential Phase (deg/km)");
    }

    #[test]
    #[should_panic(expected = "Unknown radar moment type")]
    fn test_unknown_moment_panics() {
        render_color_table_as_image("INVALID", 64, 8);
    }
}
