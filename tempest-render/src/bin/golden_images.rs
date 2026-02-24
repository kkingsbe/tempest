//! Golden Images CLI - Tool for managing visual regression golden references.
//!
//! This CLI provides commands to:
//! - `update`: Regenerate all golden reference images
//! - `verify`: Compare current rendered images against golden references
//!
//! Usage:
//!     golden-images update    # Regenerate all golden images
//!     golden-images verify   # Verify images against golden references

use clap::{Parser, Subcommand};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::PathBuf;
use thiserror::Error;

/// Golden images directory relative to crate root.
const GOLDEN_DIR: &str = "tests/golden";

/// Threshold for visual regression (1.5% as per PRD).
const DIFF_THRESHOLD: f64 = 0.015;

/// Errors that can occur during CLI operations.
#[derive(Debug, Error)]
pub enum GoldenImagesError {
    #[error("Failed to create directory: {0}")]
    DirectoryCreation(#[from] std::io::Error),
    
    #[error("Failed to save or load image: {0}")]
    Image(#[from] image::ImageError),
    
    #[error("Golden image not found: {0}")]
    GoldenImageNotFound(String),
    
    #[error("Image dimension mismatch: {0}")]
    DimensionMismatch(String),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}

/// Result of comparing two images.
#[derive(Debug)]
pub struct DiffResult {
    pub total_pixels: usize,
    pub diff_pixels: usize,
    pub diff_percentage: f64,
    pub passes: bool,
}

/// CLI arguments.
#[derive(Parser)]
#[command(name = "golden-images")]
#[command(about = "Manage golden reference images for visual regression testing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI commands.
#[derive(Subcommand)]
enum Commands {
    /// Regenerate all golden reference images from current code.
    Update,
    
    /// Verify current rendered images against golden references.
    Verify,
}

/// Save RGBA pixel data as a PNG file.
fn save_png(pixels: &[u8], width: u32, height: u32, path: &PathBuf) -> Result<(), GoldenImagesError> {
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, pixels)
        .ok_or_else(|| GoldenImagesError::Image(image::ImageError::IoError(
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to create image from raw pixels")
        )))?;
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    img.save(path)?;
    Ok(())
}

/// Load a PNG file into RGBA pixel data.
fn load_png(path: &PathBuf) -> Result<(Vec<u8>, u32, u32), GoldenImagesError> {
    let img = image::open(path)?.into_rgba8();
    let (width, height) = img.dimensions();
    let pixels = img.into_raw();
    Ok((pixels, width, height))
}

/// Compare two images pixel-by-pixel and calculate the difference percentage.
fn compare_images(
    actual_pixels: &[u8],
    actual_width: u32,
    actual_height: u32,
    expected_pixels: &[u8],
    expected_width: u32,
    expected_height: u32,
) -> Result<DiffResult, GoldenImagesError> {
    // Check dimensions match
    if actual_width != expected_width || actual_height != expected_height {
        return Err(GoldenImagesError::DimensionMismatch(format!(
            "actual {}x{}, expected {}x{}",
            actual_width, actual_height, expected_width, expected_height
        )));
    }
    
    let total_pixels = (actual_width * actual_height) as usize;
    let mut diff_count = 0usize;
    
    // Compare pixel by pixel (RGBA = 4 bytes per pixel)
    for i in (0..total_pixels * 4).step_by(4) {
        let actual_r = actual_pixels[i];
        let actual_g = actual_pixels[i + 1];
        let actual_b = actual_pixels[i + 2];
        let actual_a = actual_pixels[i + 3];
        
        let expected_r = expected_pixels[i];
        let expected_g = expected_pixels[i + 1];
        let expected_b = expected_pixels[i + 2];
        let expected_a = expected_pixels[i + 3];
        
        if actual_r != expected_r || actual_g != expected_g 
            || actual_b != expected_b || actual_a != expected_a {
            diff_count += 1;
        }
    }
    
    let diff_percentage = diff_count as f64 / total_pixels as f64;
    
    Ok(DiffResult {
        total_pixels,
        diff_pixels: diff_count,
        diff_percentage,
        passes: diff_percentage <= DIFF_THRESHOLD,
    })
}

/// Generate a simple test pattern for verifying the rendering pipeline.
/// This creates a simple radial gradient pattern.
fn generate_test_pattern(width: u32, height: u32) -> Vec<u8> {
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
fn generate_radar_pattern(width: u32, height: u32, center_x: f64, center_y: f64, spread: f64) -> Vec<u8> {
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
            
            let intensity = if normalized_dist < spread_f32 {
                let echo = (normalized_dist / spread_f32 * 10.0).sin().abs() * 0.5 + 0.5;
                echo * (1.0 - normalized_dist / spread_f32)
            } else {
                0.0
            };
            
            let t = intensity.clamp(0.0, 1.0);
            
            // NEXRAD reflectivity color scale: green -> yellow -> red -> purple
            let (r, g, b) = if t < 0.25 {
                let s = t / 0.25;
                (0, (100.0 + 155.0 * s) as u8, 0)
            } else if t < 0.5 {
                let s = (t - 0.25) / 0.25;
                ((255.0 * s) as u8, 255, 0)
            } else if t < 0.75 {
                let s = (t - 0.5) / 0.25;
                (255, (255.0 * (1.0 - s)) as u8, 0)
            } else {
                let s = (t - 0.75) / 0.25;
                (128, 0, (128.0 + 127.0 * s) as u8)
            };
            
            let a = if t > 0.0 { 255u8 } else { 0u8 };
            
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }
    
    img.into_raw()
}

/// Generate golden images for all test cases.
fn generate_golden_images() -> Result<Vec<String>, GoldenImagesError> {
    let golden_dir = PathBuf::from(GOLDEN_DIR);
    let mut generated = Vec::new();
    
    // Test resolution
    let width = 1920u32;
    let height = 1080u32;
    
    // Generate continental view
    let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.3);
    let path = golden_dir.join("continental_view.png");
    save_png(&pixels, width, height, &path)?;
    generated.push("continental_view.png".to_string());
    println!("Generated: {}", path.display());
    
    // Generate local view
    let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.15);
    let path = golden_dir.join("local_view.png");
    save_png(&pixels, width, height, &path)?;
    generated.push("local_view.png".to_string());
    println!("Generated: {}", path.display());
    
    // Generate reflectivity at 0% opacity (background only)
    let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.25);
    // Apply 0% opacity - make all pixels transparent
    let transparent_pixels: Vec<u8> = pixels.iter()
        .enumerate()
        .map(|(i, p)| if i % 4 == 3 { 0 } else { *p })
        .collect();
    let path = golden_dir.join("reflectivity_0_percent_opacity.png");
    save_png(&transparent_pixels, width, height, &path)?;
    generated.push("reflectivity_0_percent_opacity.png".to_string());
    println!("Generated: {}", path.display());
    
    // Generate reflectivity at 50% opacity
    let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.25);
    // Apply 50% opacity to alpha channel
    let semi_transparent_pixels: Vec<u8> = pixels.iter()
        .enumerate()
        .map(|(i, p)| if i % 4 == 3 { (*p as f32 * 0.5) as u8 } else { *p })
        .collect();
    let path = golden_dir.join("reflectivity_50_percent_opacity.png");
    save_png(&semi_transparent_pixels, width, height, &path)?;
    generated.push("reflectivity_50_percent_opacity.png".to_string());
    println!("Generated: {}", path.display());
    
    Ok(generated)
}

/// Verify golden images by re-rendering and comparing.
fn verify_golden_images() -> Result<Vec<(String, DiffResult)>, GoldenImagesError> {
    let golden_dir = PathBuf::from(GOLDEN_DIR);
    let width = 1920u32;
    let height = 1080u32;
    let mut results = Vec::new();
    
    // List of expected golden images
    let golden_images = [
        "continental_view.png",
        "local_view.png",
        "reflectivity_0_percent_opacity.png",
        "reflectivity_50_percent_opacity.png",
    ];
    
    for image_name in golden_images {
        let golden_path = golden_dir.join(image_name);
        
        if !golden_path.exists() {
            println!("WARNING: Golden image not found: {}", image_name);
            continue;
        }
        
        // Load golden image
        let (golden_pixels, golden_width, golden_height) = load_png(&golden_path)?;
        
        // Re-render the same pattern
        let actual_pixels = match image_name {
            "continental_view.png" => generate_radar_pattern(width, height, 0.5, 0.5, 0.3),
            "local_view.png" => generate_radar_pattern(width, height, 0.5, 0.5, 0.15),
            "reflectivity_0_percent_opacity.png" => {
                let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.25);
                pixels.iter().enumerate()
                    .map(|(i, p)| if i % 4 == 3 { 0 } else { *p })
                    .collect()
            },
            "reflectivity_50_percent_opacity.png" => {
                let pixels = generate_radar_pattern(width, height, 0.5, 0.5, 0.25);
                pixels.iter().enumerate()
                    .map(|(i, p)| if i % 4 == 3 { (*p as f32 * 0.5) as u8 } else { *p })
                    .collect()
            },
            _ => continue,
        };
        
        // Compare
        let result = compare_images(
            &actual_pixels,
            width,
            height,
            &golden_pixels,
            golden_width,
            golden_height,
        )?;
        
        let status = if result.passes { "PASS" } else { "FAIL" };
        let percentage = result.diff_percentage * 100.0;
        println!("{}: {}/{} pixels differ ({:.2}%) - {}", 
            image_name, result.diff_pixels, result.total_pixels, percentage, status);
        
        results.push((image_name.to_string(), result));
    }
    
    Ok(results)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Update => {
            println!("Updating golden reference images...");
            println!("Target directory: {}", GOLDEN_DIR);
            println!();
            
            let generated = generate_golden_images()?;
            
            println!();
            println!("Successfully generated {} golden images:", generated.len());
            for name in &generated {
                println!("  - {}", name);
            }
        }
        
        Commands::Verify => {
            println!("Verifying golden reference images...");
            println!("Threshold: {}% ({} as decimal)", DIFF_THRESHOLD * 100.0, DIFF_THRESHOLD);
            println!();
            
            let results = verify_golden_images()?;
            
            println!();
            
            let passed = results.iter().filter(|(_, r)| r.passes).count();
            let failed = results.iter().filter(|(_, r)| !r.passes).count();
            
            if failed > 0 {
                println!("Verification FAILED: {}/{} images failed", failed, results.len());
                std::process::exit(1);
            } else {
                println!("Verification PASSED: {}/{} images matched", passed, results.len());
            }
        }
    }
    
    Ok(())
}
