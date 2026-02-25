//! Golden reference image management for visual regression testing.
//!
//! This module provides functionality to manage golden reference images:
//! - `update`: Copy source images to golden directory
//! - `verify`: Compare source images against golden references

use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView, ImageReader};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GoldenError {
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Images have different dimensions: {0} vs {1}")]
    DimensionMismatch(u32, u32),
    #[error("Verification failed for {0}: {1}% difference (threshold: {2}%)")]
    VerificationFailed(String, f64, f64),
    #[error("No images found in source directory")]
    NoImagesFound,
}

/// Default threshold for image comparison (1.5% as per PRD)
pub const DEFAULT_THRESHOLD: f64 = 1.5;

/// Image comparison result
#[derive(Debug)]
pub struct ComparisonResult {
    pub path: String,
    pub difference_percent: f64,
    pub passed: bool,
}

/// Calculate the percentage of pixels that differ between two images
pub fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> Result<f64> {
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();

    if w1 != w2 || h1 != h2 {
        return Err(GoldenError::DimensionMismatch(w1, h2).into());
    }

    let rgb1 = img1.to_rgb8();
    let rgb2 = img2.to_rgb8();

    let total_pixels = (w1 * h1) as f64;
    let mut diff_pixels = 0u64;

    for (p1, p2) in rgb1.pixels().zip(rgb2.pixels()) {
        if p1 != p2 {
            // Calculate per-channel difference
            let diff = ((p1[0] as i32 - p2[0] as i32).abs()
                + (p1[1] as i32 - p2[1] as i32).abs()
                + (p1[2] as i32 - p2[2] as i32).abs()) as u64;
            // Count as different if any channel differs
            if diff > 0 {
                diff_pixels += 1;
            }
        }
    }

    Ok((diff_pixels as f64 / total_pixels) * 100.0)
}

/// Get all image files in a directory
fn get_image_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    if !dir.exists() {
        return Ok(images);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Check if it's an image file by trying to load it
            if let Ok(reader) = ImageReader::open(&path) {
                if reader.with_guessed_format()?.decode().is_ok() {
                    images.push(path);
                }
            }
        }
    }

    Ok(images)
}

/// Copy source images to golden directory
pub fn update_golden_images(source_dir: &Path, golden_dir: &Path) -> Result<()> {
    // Create golden directory if it doesn't exist
    if !golden_dir.exists() {
        fs::create_dir_all(golden_dir).context("Failed to create golden directory")?;
    }

    let source_images = get_image_files(source_dir).context("Failed to read source directory")?;

    if source_images.is_empty() {
        return Err(GoldenError::NoImagesFound.into());
    }

    let mut updated_count = 0;
    let mut same_count = 0;

    for source_path in &source_images {
        let file_name = source_path.file_name().context("Invalid file name")?;
        let golden_path = golden_dir.join(file_name);

        // Check if golden image exists and is identical
        if golden_path.exists() {
            let source_img = ImageReader::open(source_path)?
                .with_guessed_format()?
                .decode()?;
            let golden_img = ImageReader::open(&golden_path)?
                .with_guessed_format()?
                .decode()?;

            let diff = compare_images(&source_img, &golden_img)?;

            if diff < 0.01 {
                // Images are essentially identical, skip
                same_count += 1;
                continue;
            }
        }

        // Copy file to golden directory
        fs::copy(source_path, &golden_path)
            .with_context(|| format!("Failed to copy {:?} to golden directory", file_name))?;

        updated_count += 1;
    }

    println!(
        "Update complete: {} updated, {} unchanged",
        updated_count, same_count
    );
    Ok(())
}

/// Verify source images against golden references
pub fn verify_images(
    source_dir: &Path,
    golden_dir: &Path,
    threshold: f64,
) -> Result<Vec<ComparisonResult>> {
    if !golden_dir.exists() {
        anyhow::bail!("Golden directory does not exist: {:?}", golden_dir);
    }

    let source_images = get_image_files(source_dir).context("Failed to read source directory")?;

    if source_images.is_empty() {
        return Err(GoldenError::NoImagesFound.into());
    }

    let golden_images: HashSet<_> = get_image_files(golden_dir)?
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_owned()))
        .collect();

    let mut results = Vec::new();
    let mut failed = 0;
    let mut passed = 0;

    for source_path in &source_images {
        let file_name = source_path.file_name().context("Invalid file name")?;

        let file_name_str = file_name.to_string_lossy().to_string();

        // Check if golden reference exists
        if !golden_images.contains(file_name) {
            results.push(ComparisonResult {
                path: file_name_str.clone(),
                difference_percent: 100.0,
                passed: false,
            });
            failed += 1;
            continue;
        }

        let golden_path = golden_dir.join(file_name);

        // Load and compare images
        let source_img = ImageReader::open(source_path)?
            .with_guessed_format()?
            .decode()?;
        let golden_img = ImageReader::open(&golden_path)?
            .with_guessed_format()?
            .decode()?;

        let diff = compare_images(&source_img, &golden_img)?;
        let passed_check = diff <= threshold;

        if passed_check {
            println!("✓ {}: {}% different", file_name_str, diff);
            passed += 1;
        } else {
            eprintln!(
                "✗ {}: {}% different (threshold: {}%)",
                file_name_str, diff, threshold
            );
            failed += 1;
        }

        results.push(ComparisonResult {
            path: file_name_str,
            difference_percent: diff,
            passed: passed_check,
        });
    }

    // Summary
    let total = results.len();
    println!("Verification complete: {}/{} passed", passed, total);

    if failed > 0 {
        println!("Failed images:");
        for r in &results {
            if !r.passed {
                println!("  - {}: {}% different", r.path, r.difference_percent);
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Create a simple test image
    fn create_test_image(width: u32, height: u32, color: (u8, u8, u8)) -> DynamicImage {
        let mut buffer = vec![0u8; (width * height * 3) as usize];

        for i in 0..(width * height) {
            let idx = (i * 3) as usize;
            buffer[idx] = color.0;
            buffer[idx + 1] = color.1;
            buffer[idx + 2] = color.2;
        }

        image::RgbImage::from_raw(width, height, buffer)
            .map(DynamicImage::ImageRgb8)
            .unwrap()
    }

    /// Save a test image to a directory
    fn save_test_image(dir: &TempDir, name: &str, img: &DynamicImage) -> PathBuf {
        let path = dir.path().join(name);
        img.save(&path).unwrap();
        path
    }

    #[test]
    fn test_identical_images() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(100, 100, (255, 0, 0));

        let diff = compare_images(&img1, &img2).unwrap();

        assert!(diff < 0.01, "Identical images should have 0% difference");
    }

    #[test]
    fn test_different_images() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(100, 100, (0, 0, 255));

        let diff = compare_images(&img1, &img2).unwrap();

        assert!(diff > 99.0, "Different images should have high difference: {}%", diff);
    }

    #[test]
    fn test_partial_difference() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(100, 100, (255, 0, 0));

        // Modify 10% of pixels in img2
        let mut rgb2 = img2.to_rgb8();
        let total_pixels = 100 * 100;
        let pixels_to_change = (total_pixels as f64 * 0.10) as u32;

        for i in 0..pixels_to_change {
            let x = i % 100;
            let y = i / 100;
            rgb2.put_pixel(x, y, image::Rgb([0, 0, 255]));
        }

        let img2_modified = DynamicImage::ImageRgb8(rgb2);
        let diff = compare_images(&img1, &img2_modified).unwrap();

        assert!(
            (diff - 10.0).abs() < 1.0,
            "Expected ~10% difference, got {}%",
            diff
        );
    }

    #[test]
    fn test_dimension_mismatch() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(200, 200, (255, 0, 0));

        let result = compare_images(&img1, &img2);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<GoldenError>()
            .is_some_and(|e| matches!(e, GoldenError::DimensionMismatch(_, _))));
    }

    #[test]
    fn test_update_golden_images() {
        let source_dir = TempDir::new().unwrap();
        let golden_dir = TempDir::new().unwrap();

        // Create source images
        let img1 = create_test_image(100, 100, (255, 0, 0));
        save_test_image(&source_dir, "test1.png", &img1);

        // Run update
        update_golden_images(source_dir.path(), golden_dir.path()).unwrap();

        // Verify golden images exist
        assert!(golden_dir.path().join("test1.png").exists());
    }

    #[test]
    fn test_verify_images_pass() {
        let source_dir = TempDir::new().unwrap();
        let golden_dir = TempDir::new().unwrap();

        // Create identical images in both directories
        let img = create_test_image(100, 100, (255, 0, 0));
        save_test_image(&source_dir, "test.png", &img);
        save_test_image(&golden_dir, "test.png", &img);

        // Run verify with default threshold (1.5%)
        let results = verify_images(source_dir.path(), golden_dir.path(), 1.5).unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].passed, "Images should pass verification");
    }

    #[test]
    fn test_verify_images_fail() {
        let source_dir = TempDir::new().unwrap();
        let golden_dir = TempDir::new().unwrap();

        // Create different images
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(100, 100, (0, 0, 255));
        save_test_image(&source_dir, "test.png", &img1);
        save_test_image(&golden_dir, "test.png", &img2);

        // Run verify with 1.5% threshold
        let results = verify_images(source_dir.path(), golden_dir.path(), 1.5).unwrap();

        assert_eq!(results.len(), 1);
        assert!(!results[0].passed, "Images should fail verification");
    }

    #[test]
    fn test_verify_missing_golden() {
        let source_dir = TempDir::new().unwrap();
        let golden_dir = TempDir::new().unwrap();

        // Create source image but no golden
        let img = create_test_image(100, 100, (255, 0, 0));
        save_test_image(&source_dir, "test.png", &img);

        // Run verify
        let results = verify_images(source_dir.path(), golden_dir.path(), 1.5).unwrap();

        assert_eq!(results.len(), 1);
        assert!(!results[0].passed, "Missing golden should fail verification");
    }

    #[test]
    fn test_default_threshold() {
        assert_eq!(
            DEFAULT_THRESHOLD,
            1.5,
            "Default threshold should be 1.5% as per PRD"
        );
    }
}
