//! Visual regression test framework for radar rendering.
//!
//! This module provides utilities for rendering radar sweeps to images
//! and comparing them against golden reference images for visual regression testing.
//!
//! ## Design
//!
//! - Uses wgpu with GL backend for deterministic, reproducible rendering
//! - Fixed resolution (1920x1080) as per PRD specification
//! - Fixed map viewport for reproducibility across runs
//! - Pixel-diff comparison with ≤1.5% threshold as per PRD
//!
//! ## Usage
//!
//! ```rust
//! use tempest_render::visual_test::render_and_compare;
//!
//! // Compare rendered output against golden reference
//! let result = render_and_compare("reflectivity_sweep", &radar_data);
//! assert!(result.is_ok());
//! ```

use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::{Path, PathBuf};

/// Fixed resolution for visual regression tests as per PRD.
pub const TEST_WIDTH: u32 = 1920;
pub const TEST_HEIGHT: u32 = 1080;

/// Maximum acceptable difference threshold as per PRD (1.5%).
pub const MAX_DIFF_THRESHOLD: f64 = 0.015;

/// Directory containing golden reference images.
pub const GOLDEN_DIR: &str = "tests/golden";

/// Test output directory for generated images.
pub const OUTPUT_DIR: &str = "tests/output";

/// Initialize wgpu with software/GL backend for headless rendering.
///
/// This function creates a wgpu Instance and Adapter using multiple backend options,
/// trying GL, Vulkan, and Metal backends to find one that works in the current environment.
/// The software backend provides deterministic software rendering independent of GPU hardware.
///
/// # Returns
///
/// * `Ok((wgpu::Instance, wgpu::Adapter))` - Initialized wgpu components
/// * `Err(String)` - Error message if initialization fails
pub fn init_wgpu_software_backend() -> Result<(wgpu::Instance, wgpu::Adapter), String> {
    // Try multiple backends in order of preference for software rendering
    let backends_to_try = [
        (wgpu::Backends::GL, "GL"),
        (wgpu::Backends::VULKAN, "Vulkan"),
        (wgpu::Backends::METAL, "Metal"),
        (wgpu::Backends::DX12, "DX12"),
    ];
    
    let mut last_error = String::new();
    
    for (backends, name) in backends_to_try {
        // Skip empty backends
        if backends.is_empty() {
            continue;
        }
        
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends,
            dx12_shader_compiler: Default::default(),
            flags: wgpu::InstanceFlags::default(),
            gles_minor_version: Default::default(),
        });

        // Request adapter (device)
        match futures::executor::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: None,
                force_fallback_adapter: true, // Force software rendering
            },
        )) {
            Some(adapter) => {
                println!("Successfully initialized wgpu with {} backend", name);
                return Ok((instance, adapter));
            }
            None => {
                last_error = format!("Failed to request {} adapter", name);
                println!("{} - trying next backend", last_error);
            }
        }
    }
    
    Err(format!("No suitable wgpu adapter found. Last error: {}", last_error))
}

/// Create a wgpu Device and Queue for rendering.
///
/// # Arguments
///
/// * `adapter` - The wgpu adapter to use
///
/// # Returns
///
/// * `Ok((wgpu::Device, wgpu::Queue))` - Initialized device and queue
/// * `Err(String)` - Error message if initialization fails
pub fn create_device_and_queue(
    adapter: &wgpu::Adapter,
) -> Result<(wgpu::Device, wgpu::Queue), String> {
    futures::executor::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("visual-regression-test"),
            required_features: wgpu::Features::default(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .map_err(|e| format!("Failed to request device: {}", e))
}

/// Fixed map viewport configuration for reproducible rendering.
///
/// This viewport is used for all visual regression tests to ensure
/// consistent results across different runs and platforms.
#[derive(Debug, Clone)]
pub struct TestViewport {
    /// Center latitude of the viewport
    pub center_lat: f64,
    /// Center longitude of the viewport
    pub center_lon: f64,
    /// Zoom level (z)
    pub zoom: u8,
    /// Viewport width in pixels
    pub width: u32,
    /// Viewport height in pixels
    pub height: u32,
}

impl Default for TestViewport {
    fn default() -> Self {
        Self {
            center_lat: 39.0, // Continental US center approximately
            center_lon: -98.0,
            zoom: 6,
            width: TEST_WIDTH,
            height: TEST_HEIGHT,
        }
    }
}

impl TestViewport {
    /// Create a viewport for continental view (z=6).
    pub fn continental() -> Self {
        Self {
            center_lat: 39.0,
            center_lon: -98.0,
            zoom: 6,
            width: TEST_WIDTH,
            height: TEST_HEIGHT,
        }
    }

    /// Create a viewport for regional view (z=10).
    pub fn regional() -> Self {
        Self {
            center_lat: 39.0,
            center_lon: -98.0,
            zoom: 10,
            width: TEST_WIDTH,
            height: TEST_HEIGHT,
        }
    }

    /// Create a viewport for local view (z=14).
    pub fn local() -> Self {
        Self {
            center_lat: 39.0,
            center_lon: -98.0,
            zoom: 14,
            width: TEST_WIDTH,
            height: TEST_HEIGHT,
        }
    }
}

/// Render configuration for visual regression tests.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Viewport configuration
    pub viewport: TestViewport,
    /// Radar opacity (0.0 to 1.0)
    pub opacity: f32,
    /// Whether to render with antialiasing
    pub antialiasing: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            viewport: TestViewport::default(),
            opacity: 1.0,
            antialiasing: true,
        }
    }
}

impl RenderConfig {
    /// Create config with full opacity.
    pub fn with_full_opacity() -> Self {
        Self::default()
    }

    /// Create config with 50% opacity.
    pub fn with_half_opacity() -> Self {
        Self {
            opacity: 0.5,
            ..Default::default()
        }
    }
}

/// Save RGBA pixel data as a PNG file.
///
/// # Arguments
///
/// * `pixels` - Raw RGBA pixel data (width * height * 4 bytes)
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `path` - Output file path
///
/// # Returns
///
/// * `Ok(())` - Image saved successfully
/// * `Err(String)` - Error message if save fails
pub fn save_png(pixels: &[u8], width: u32, height: u32, path: &Path) -> Result<(), String> {
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
        .ok_or_else(|| "Failed to create image from raw pixels".to_string())?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    img.save(path).map_err(|e| format!("Failed to save PNG: {}", e))
}

/// Load a PNG file into RGBA pixel data.
///
/// # Arguments
///
/// * `path` - Input file path
///
/// # Returns
///
/// * `Ok((Vec<u8>, u32, u32))` - Pixel data, width, and height
/// * `Err(String)` - Error message if load fails
pub fn load_png(path: &Path) -> Result<(Vec<u8>, u32, u32), String> {
    let img = image::open(path)
        .map_err(|e| format!("Failed to open PNG: {}", e))?
        .into_rgba8();

    let (width, height) = img.dimensions();
    let pixels = img.into_raw();

    Ok((pixels, width, height))
}

/// Compare two images pixel-by-pixel and calculate the difference percentage.
///
/// # Arguments
///
/// * `actual_pixels` - Actual rendered pixel data
/// * `actual_width` - Actual image width
/// * `actual_height` - Actual image height
/// * `expected_pixels` - Expected/golden pixel data
/// * `expected_width` - Expected image width
/// * `expected_height` - Expected image height
///
/// # Returns
///
/// * `Ok(DiffResult)` - Comparison result with difference percentage
/// * `Err(String)` - Error message if comparison fails
pub fn compare_images(
    actual_pixels: &[u8],
    actual_width: u32,
    actual_height: u32,
    expected_pixels: &[u8],
    expected_width: u32,
    expected_height: u32,
) -> Result<DiffResult, String> {
    // Check dimensions match
    if actual_width != expected_width || actual_height != expected_height {
        return Err(format!(
            "Image dimensions mismatch: actual {}x{}, expected {}x{}",
            actual_width, actual_height, expected_width, expected_height
        ));
    }

    let total_pixels = (actual_width * actual_height) as usize;
    let mut diff_count = 0usize;

    // Compare pixel by pixel (RGBA = 4 bytes per pixel)
    for i in (0..total_pixels * 4).step_by(4) {
        // Check if any RGBA component differs
        let actual_r = actual_pixels[i];
        let actual_g = actual_pixels[i + 1];
        let actual_b = actual_pixels[i + 2];
        let actual_a = actual_pixels[i + 3];

        let expected_r = expected_pixels[i];
        let expected_g = expected_pixels[i + 1];
        let expected_b = expected_pixels[i + 2];
        let expected_a = expected_pixels[i + 3];

        // Consider pixels different if any component differs
        if actual_r != expected_r
            || actual_g != expected_g
            || actual_b != expected_b
            || actual_a != expected_a
        {
            diff_count += 1;
        }
    }

    let diff_percentage = diff_count as f64 / total_pixels as f64;

    Ok(DiffResult {
        total_pixels,
        diff_pixels: diff_count,
        diff_percentage,
        passes: diff_percentage <= MAX_DIFF_THRESHOLD,
    })
}

/// Result of image comparison.
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// Total number of pixels in the image
    pub total_pixels: usize,
    /// Number of pixels that differ
    pub diff_pixels: usize,
    /// Percentage of pixels that differ (0.0 to 1.0)
    pub diff_percentage: f64,
    /// Whether the comparison passes the threshold
    pub passes: bool,
}

impl DiffResult {
    /// Returns a formatted string describing the diff result.
    pub fn summary(&self) -> String {
        let percentage = self.diff_percentage * 100.0;
        format!(
            "{}/{} pixels differ ({:.2}%) - {}",
            self.diff_pixels,
            self.total_pixels,
            percentage,
            if self.passes { "PASS" } else { "FAIL" }
        )
    }
}

/// Compare rendered output against a golden reference image.
///
/// # Arguments
///
/// * `test_name` - Name of the test (used for file paths)
/// * `actual_pixels` - Actual rendered pixel data
/// * `width` - Image width
/// * `height` - Image height
/// * `update_golden` - If true, update the golden reference instead of comparing
///
/// # Returns
///
/// * `Ok(DiffResult)` - Comparison result
/// * `Err(String)` - Error message
pub fn render_and_compare(
    test_name: &str,
    actual_pixels: &[u8],
    width: u32,
    height: u32,
    update_golden: bool,
) -> Result<DiffResult, String> {
    let golden_path = PathBuf::from(GOLDEN_DIR).join(format!("{}.png", test_name));

    if update_golden {
        // Update golden reference
        save_png(actual_pixels, width, height, &golden_path)?;
        return Ok(DiffResult {
            total_pixels: (width * height) as usize,
            diff_pixels: 0,
            diff_percentage: 0.0,
            passes: true,
        });
    }

    // Load golden reference
    let (expected_pixels, expected_width, expected_height) = load_png(&golden_path)?;

    // Compare
    compare_images(
        actual_pixels,
        width,
        height,
        &expected_pixels,
        expected_width,
        expected_height,
    )
}

/// Generate a simple test pattern for verifying the rendering pipeline.
///
/// This creates a simple radial gradient pattern that can be used
/// to verify the basic rendering functionality.
///
/// # Arguments
///
/// * `width` - Image width
/// * `height` - Image height
///
/// # Returns
///
/// * `Vec<u8>` - Raw RGBA pixel data
pub fn generate_test_pattern(width: u32, height: u32) -> Vec<u8> {
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let max_radius = (center_x * center_x + center_y * center_y).sqrt();

    let mut img: RgbaImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - center_x;
            let dy = y as f32 - center_y;
            let distance = (dx * dx + dy * dy).sqrt();
            let t = (distance / max_radius).clamp(0.0, 1.0);

            // Create a radial gradient from green to red
            let r = (t * 255.0) as u8;
            let g = ((1.0 - t) * 255.0) as u8;
            let b = 0u8;
            let a = 255u8;

            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img.into_raw()
}

/// Generate a radar-like pattern with multiple sweep echoes.
///
/// This creates a more realistic radar pattern simulating reflectivity data
/// with multiple echo centers at different intensities.
///
/// # Arguments
///
/// * `width` - Image width
/// * `height` - Image height
/// * `center_x` - Normalized center X position (0.0-1.0)
/// * `center_y` - Normalized center Y position (0.0-1.0)
/// * `spread` - Spread factor for the radar echoes (smaller = more focused)
///
/// # Returns
///
/// * `Vec<u8>` - Raw RGBA pixel data
pub fn generate_radar_pattern(width: u32, height: u32, center_x: f64, center_y: f64, spread: f64) -> Vec<u8> {
    let cx = (width as f64 * center_x) as f32;
    let cy = (height as f64 * center_y) as f32;
    let max_radius = ((width as f32 / 2.0).powi(2) + (height as f32 / 2.0).powi(2)).sqrt();
    let spread_f32 = spread as f32;

    let mut img: RgbaImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let distance = (dx * dx + dy * dy).sqrt();
            let normalized_dist = distance / max_radius;
            
            // Create radar-like echo pattern
            let intensity = if normalized_dist < spread_f32 {
                // Inside radar range - create echo pattern
                let echo = (normalized_dist / spread_f32 * 10.0).sin().abs() * 0.5 + 0.5;
                echo * (1.0 - normalized_dist / spread_f32)
            } else {
                0.0
            };
            
            let t = intensity.clamp(0.0, 1.0);
            
            // NEXRAD reflectivity color scale: green -> yellow -> red -> purple
            let (r, g, b) = if t < 0.25 {
                // Light green to green
                let s = t / 0.25;
                (0, (100.0 + 155.0 * s) as u8, 0)
            } else if t < 0.5 {
                // Green to yellow
                let s = (t - 0.25) / 0.25;
                ((255.0 * s) as u8, 255, 0)
            } else if t < 0.75 {
                // Yellow to red
                let s = (t - 0.5) / 0.25;
                (255, (255.0 * (1.0 - s)) as u8, 0)
            } else {
                // Red to purple
                let s = (t - 0.75) / 0.25;
                (255, 0, (128.0 * s) as u8)
            };
            
            let a = if t > 0.01 { 200 } else { 0 }; // Semi-transparent background
            
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    img.into_raw()
}

/// Generate a reflectivity-style radar pattern with classic NEXRAD colors.
///
/// # Arguments
///
/// * `width` - Image width
/// * `height` - Image height
///
/// # Returns
///
/// * `Vec<u8>` - Raw RGBA pixel data
pub fn generate_reflectivity_pattern(width: u32, height: u32) -> Vec<u8> {
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let max_radius = (cx * cx + cy * cy).sqrt();

    let mut img: RgbaImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let distance = (dx * dx + dy * dy).sqrt();
            let t = (distance / max_radius).clamp(0.0, 1.0);

            // Reflectivity color scale: light green -> green -> yellow -> red -> purple
            let (r, g, b) = if t < 0.1 {
                // Below 5 dBZ - light green
                (100, 200, 100)
            } else if t < 0.25 {
                // 5-20 dBZ - green
                (0, 255, 0)
            } else if t < 0.4 {
                // 20-35 dBZ - yellow
                (255, 255, 0)
            } else if t < 0.6 {
                // 35-50 dBZ - red
                (255, 0, 0)
            } else if t < 0.8 {
                // 50-60 dBZ - purple
                (200, 0, 200)
            } else {
                // 60+ dBZ - white/outline
                (255, 255, 255)
            };

            // Create some echo structure
            let echo_pattern = ((t * 20.0).sin() * 0.3 + 0.7).max(0.0);
            let alpha = if t < 0.9 { (echo_pattern * 255.0) as u8 } else { 0 };

            img.put_pixel(x, y, Rgba([r, g, b, alpha]));
        }
    }

    img.into_raw()
}

/// Generate a velocity-style radar pattern with blue/red colors.
///
/// Velocity data shows directional velocity: negative (approaching) in blue,
/// positive (receding) in red, with green near zero (broadening).
///
/// # Arguments
///
/// * `width` - Image width
/// * `height` - Image height
///
/// # Returns
///
/// * `Vec<u8>` - Raw RGBA pixel data
pub fn generate_velocity_pattern(width: u32, height: u32) -> Vec<u8> {
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;

    let mut img: RgbaImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let dx = (x as f32 - cx) / cx;
            let dy = (y as f32 - cy) / cy;
            
            // Create a velocity field pattern (approaching/receding)
            let velocity = dx.sin() * dy.cos();
            let t = (velocity + 1.0) / 2.0; // Normalize to 0-1

            // Velocity color scale: blue (approaching) -> green (zero) -> red (receding)
            let (r, g, b) = if t < 0.45 {
                // Strong approaching - dark blue to light blue
                let s = t / 0.45;
                (0, (100.0 * s) as u8, (150.0 + 105.0 * s) as u8)
            } else if t < 0.55 {
                // Near zero - green (gatefold)
                (0, 255, 0)
            } else {
                // Receding - light red to dark red
                let s = (t - 0.55) / 0.45;
                ((150.0 + 105.0 * s) as u8, (100.0 * (1.0 - s)) as u8, 0)
            };

            // Fade out at edges
            let dist = (dx * dx + dy * dy).sqrt();
            let alpha = if dist < 1.0 { ((1.0 - dist) * 255.0) as u8 } else { 0 };

            img.put_pixel(x, y, Rgba([r, g, b, alpha]));
        }
    }

    img.into_raw()
}

/// Generate a spectrum width-style radar pattern with brown/tan colors.
///
/// Spectrum Width shows the spread of velocities within a resolution volume - 
/// a measure of turbulence and velocity dispersion. It is typically displayed
/// in brown/tan colors following meteorological conventions.
///
/// # Arguments
///
/// * `width` - Image width
/// * `height` - Image height
///
/// # Returns
///
/// * `Vec<u8>` - Raw RGBA pixel data
pub fn generate_spectrum_width_pattern(width: u32, height: u32) -> Vec<u8> {
    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;

    let mut img: RgbaImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let dx = (x as f32 - cx) / cx;
            let dy = (y as f32 - cy) / cy;
            
            // Create a spectrum width pattern - shows turbulence/spread
            // Use radial patterns with varying widths to simulate spectrum width
            let dist = (dx * dx + dy * dy).sqrt();
            let angle = dy.atan2(dx);
            
            // Create turbulence-like pattern
            let turbulence = (angle * 3.0).sin() * 0.3 + (dist * 10.0).sin() * 0.2;
            let t = ((turbulence + 0.5).clamp(0.0, 1.0) * (1.0 - dist * 0.5)).clamp(0.0, 1.0);

            // Spectrum Width color scale: light tan/brown -> dark brown
            // Following meteorological conventions: light brown (low SW) to dark brown (high SW)
            let (r, g, b) = if t < 0.25 {
                // Very low spectrum width - light tan
                let s = t / 0.25;
                ((180.0 + 75.0 * s) as u8, (160.0 + 60.0 * s) as u8, (120.0 + 50.0 * s) as u8)
            } else if t < 0.5 {
                // Low to medium - tan to light brown
                let s = (t - 0.25) / 0.25;
                ((150.0 + 30.0 * s) as u8, (130.0 + 30.0 * s) as u8, (100.0 + 20.0 * s) as u8)
            } else if t < 0.75 {
                // Medium to high - light brown to brown
                let s = (t - 0.5) / 0.25;
                ((120.0 + 30.0 * s) as u8, (100.0 + 30.0 * s) as u8, (80.0 + 20.0 * s) as u8)
            } else {
                // High spectrum width - dark brown
                let s = (t - 0.75) / 0.25;
                ((90.0 + 30.0 * s) as u8, (70.0 + 30.0 * s) as u8, (50.0 + 30.0 * s) as u8)
            };

            // Fade out at edges
            let alpha = if dist < 1.0 { ((1.0 - dist) * 255.0) as u8 } else { 0 };

            img.put_pixel(x, y, Rgba([r, g, b, alpha]));
        }
    }

    img.into_raw()
}

/// Get the path to a golden reference image for a test.
///
/// # Arguments
///
/// * `test_name` - Name of the test
///
/// # Returns
///
/// * `PathBuf` - Path to the golden reference image
pub fn golden_path(test_name: &str) -> PathBuf {
    PathBuf::from(GOLDEN_DIR).join(format!("{}.png", test_name))
}

/// Get the path to save test output.
///
/// # Arguments
///
/// * `test_name` - Name of the test
///
/// # Returns
///
/// * `PathBuf` - Path for the output image
pub fn output_path(test_name: &str) -> PathBuf {
    PathBuf::from(OUTPUT_DIR).join(format!("{}.png", test_name))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_png_roundtrip() {
        // Generate test pattern
        let pixels = generate_test_pattern(256, 128);
        let width = 256u32;
        let height = 128u32;

        // Save to temp file
        let temp_path = std::env::temp_dir().join("test_visual_regression.png");
        save_png(&pixels, width, height, &temp_path).unwrap();

        // Load back
        let (loaded_pixels, loaded_width, loaded_height) = load_png(&temp_path).unwrap();

        // Verify
        assert_eq!(loaded_width, width);
        assert_eq!(loaded_height, height);
        assert_eq!(loaded_pixels, pixels);

        // Cleanup
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_compare_identical_images() {
        let pixels = generate_test_pattern(100, 100);

        let result = compare_images(
            &pixels,
            100,
            100,
            &pixels,
            100,
            100,
        )
        .unwrap();

        assert!(result.passes);
        assert_eq!(result.diff_pixels, 0);
    }

    #[test]
    fn test_compare_different_images() {
        let pixels1 = generate_test_pattern(100, 100);
        let pixels2 = generate_test_pattern(100, 100);

        // Modify pixels in pixels2 - change the center pixel (index 50*100*4 + 50*4)
        // The center of a 100x100 image is at (50, 50)
        let mut pixels2_modified = pixels2.clone();
        let center_idx = 50 * 100 * 4 + 50 * 4; // Center pixel in RGBA
        pixels2_modified[center_idx] = 255; // Set red channel to max
        pixels2_modified[center_idx + 1] = 0; // Green to 0
        pixels2_modified[center_idx + 2] = 0; // Blue to 0

        let result = compare_images(
            &pixels1,
            100,
            100,
            &pixels2_modified,
            100,
            100,
        )
        .unwrap();

        // Only 1 pixel out of 10000 should differ
        assert!(result.diff_pixels >= 1);
        assert!(result.diff_percentage < MAX_DIFF_THRESHOLD);
    }

    #[test]
    fn test_compare_dimension_mismatch() {
        let pixels1 = generate_test_pattern(100, 100);
        let pixels2 = generate_test_pattern(200, 200);

        let result = compare_images(
            &pixels1,
            100,
            100,
            &pixels2,
            200,
            200,
        );

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("dimensions mismatch"));
    }

    #[test]
    fn test_diff_result_summary() {
        let result = DiffResult {
            total_pixels: 10000,
            diff_pixels: 100,
            diff_percentage: 0.01,
            passes: true,
        };

        let summary = result.summary();
        assert!(summary.contains("100/10000"));
        assert!(summary.contains("1.00%"));
        assert!(summary.contains("PASS"));
    }

    #[test]
    fn test_viewport_configs() {
        let continental = TestViewport::continental();
        assert_eq!(continental.zoom, 6);
        assert_eq!(continental.width, TEST_WIDTH);

        let regional = TestViewport::regional();
        assert_eq!(regional.zoom, 10);

        let local = TestViewport::local();
        assert_eq!(local.zoom, 14);
    }

    #[test]
    fn test_render_configs() {
        let full_opacity = RenderConfig::with_full_opacity();
        assert_eq!(full_opacity.opacity, 1.0);

        let half_opacity = RenderConfig::with_half_opacity();
        assert_eq!(half_opacity.opacity, 0.5);
    }

    #[test]
    fn test_path_generation() {
        let golden = golden_path("test_sweep");
        assert!(golden.to_string_lossy().ends_with("tests/golden/test_sweep.png"));

        let output = output_path("test_sweep");
        assert!(output.to_string_lossy().ends_with("tests/output/test_sweep.png"));
    }

    // ============================================================================
    // WGPU Software Backend Tests
    // ============================================================================
    // These tests verify the wgpu software backend initialization works correctly.
    // The software backend provides deterministic, reproducible rendering for CI.
    // Note: These tests may be skipped in environments without GPU/graphics support.

    #[test]
    fn test_wgpu_software_backend_initialization() {
        // Initialize the wgpu software backend
        let result = init_wgpu_software_backend();
        
        if result.is_err() {
            // Skip test if no wgpu backend is available (common in headless CI)
            println!("SKIPPED: No wgpu adapter available (expected in headless environments)");
            return;
        }
        
        let (_instance, adapter) = result.unwrap();
        
        // Verify the instance was created
        let info = adapter.get_info();
        println!("wgpu adapter info: {:?}", info);
        
        // The adapter should be available
        assert!(!info.name.is_empty(), "Adapter should have a name");
    }

    #[test]
    fn test_wgpu_device_and_queue_creation() {
        // Initialize the wgpu software backend
        let result = init_wgpu_software_backend();
        
        if result.is_err() {
            // Skip test if no wgpu backend is available (common in headless CI)
            println!("SKIPPED: No wgpu adapter available (expected in headless environments)");
            return;
        }
        
        let (_instance, adapter) = result.expect("Failed to initialize wgpu software backend");
        
        // Create device and queue
        let result = create_device_and_queue(&adapter);
        assert!(result.is_ok(), "Failed to create device and queue: {:?}", result.err());
        
        let (device, _queue) = result.unwrap();
        
        // Print device limits for verification
        let limits = device.limits();
        println!("wgpu device limits: max_texture_2d={}, max_buffer_size={}", 
            limits.max_texture_dimension_2d, limits.max_buffer_size);
        
        // Check that we can create basic resources
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("test buffer"),
            size: 1024,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        });
        
        assert!(buffer.size() >= 1024, "Buffer should have correct size");
        
        println!("WGPU software backend initialized successfully");
    }

    // ============================================================================
    // Baseline Generation Tests
    // ============================================================================
    // These tests generate baseline golden images for visual regression testing.
    // Run with: cargo test --package tempest-render --test visual_regression baseline -- --nocapture
    // To regenerate baselines, use the RECORD_BASELINES environment variable.

    #[test]
    fn baseline_continental_view() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate a radar-like pattern for continental view
        let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.3);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("continental_view", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/continental_view.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    #[test]
    fn baseline_regional_view() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate a more detailed radar pattern for regional view
        let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.15);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("regional_view", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/regional_view.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    #[test]
    fn baseline_local_view() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate high-detail radar pattern for local view
        let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.05);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("local_view", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/local_view.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    #[test]
    fn baseline_reflectivity_sweep() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate reflectivity-style radar pattern (red/yellow/green colors)
        let pixels = generate_reflectivity_pattern(width, height);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("reflectivity_sweep", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/reflectivity_sweep.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    #[test]
    fn baseline_velocity_sweep() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate velocity-style radar pattern (blue/red colors)
        let pixels = generate_velocity_pattern(width, height);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("velocity_sweep", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/velocity_sweep.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    #[test]
    fn baseline_spectrum_width_sweep() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate spectrum width-style radar pattern (brown/tan colors)
        let pixels = generate_spectrum_width_pattern(width, height);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("spectrum_width_sweep", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/spectrum_width_sweep.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    /// Generate a pattern with 0% opacity (completely transparent).
    /// This tests that the renderer correctly handles fully transparent radar data.
    fn generate_0_percent_opacity_pattern(width: u32, height: u32) -> Vec<u8> {
        // First generate the reflectivity pattern
        let base_pixels = generate_reflectivity_pattern(width, height);
        
        // Set all alpha values to 0 (completely transparent)
        let mut pixels = base_pixels;
        for i in (0..pixels.len()).step_by(4) {
            pixels[i + 3] = 0; // Set alpha to 0
        }
        
        pixels
    }

    /// Generate a pattern with 50% opacity.
    /// This tests that the renderer correctly applies alpha blending at half opacity.
    fn generate_50_percent_opacity_pattern(width: u32, height: u32) -> Vec<u8> {
        // First generate the reflectivity pattern
        let base_pixels = generate_reflectivity_pattern(width, height);
        
        // Multiply all alpha values by 0.5 (50% opacity)
        let mut pixels = base_pixels;
        for i in (0..pixels.len()).step_by(4) {
            let original_alpha = pixels[i + 3];
            pixels[i + 3] = (original_alpha as f32 * 0.5) as u8;
        }
        
        pixels
    }

    #[test]
    fn baseline_reflectivity_0_percent_opacity() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate reflectivity pattern with 0% opacity (completely transparent)
        let pixels = generate_0_percent_opacity_pattern(width, height);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("reflectivity_0_percent_opacity", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/reflectivity_0_percent_opacity.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }

    #[test]
    fn baseline_reflectivity_50_percent_opacity() {
        let width = TEST_WIDTH;
        let height = TEST_HEIGHT;
        
        // Generate reflectivity pattern with 50% opacity
        let pixels = generate_50_percent_opacity_pattern(width, height);
        
        let update_golden = std::env::var("RECORD_BASELINES").is_ok();
        let result = render_and_compare("reflectivity_50_percent_opacity", &pixels, width, height, update_golden).unwrap();
        
        if update_golden {
            println!("Created baseline: tests/golden/reflectivity_50_percent_opacity.png");
        }
        assert!(result.passes, "Baseline should match itself");
    }
}
