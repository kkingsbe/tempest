//! Tests for Golden CLI module.
//!
//! These tests verify the golden reference image management functionality
//! used for visual regression testing.

use tempfile::TempDir;
use tempest_app::golden_cli::{
    compare_images, update_golden_images, verify_images, GoldenError, DEFAULT_THRESHOLD,
};

/// Create a simple test image as PNG bytes
fn create_test_image(width: u32, height: u32, color: (u8, u8, u8)) -> image::DynamicImage {
    let mut buffer = vec![0u8; (width * height * 3) as usize];

    for i in 0..(width * height) {
        let idx = (i * 3) as usize;
        buffer[idx] = color.0;
        buffer[idx + 1] = color.1;
        buffer[idx + 2] = color.2;
    }

    image::RgbImage::from_raw(width, height, buffer)
        .map(image::DynamicImage::ImageRgb8)
        .unwrap()
}

/// Save a test image to a directory
fn save_test_image(dir: &TempDir, name: &str, img: &image::DynamicImage) -> std::path::PathBuf {
    let path = dir.path().join(name);
    img.save(&path).unwrap();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test: identical images should have 0% difference
    #[test]
    fn test_identical_images() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(100, 100, (255, 0, 0));

        let diff = compare_images(&img1, &img2).unwrap();

        assert!(diff < 0.01, "Identical images should have 0% difference");
    }

    /// Test: completely different images should have high difference
    #[test]
    fn test_different_images() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(100, 100, (0, 0, 255));

        let diff = compare_images(&img1, &img2).unwrap();

        // All pixels should be different (100% difference)
        assert!(
            diff > 99.0,
            "Different images should have high difference: {}%",
            diff
        );
    }

    /// Test: partially different images should have proportional difference
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

        let img2_modified = image::DynamicImage::ImageRgb8(rgb2);
        let diff = compare_images(&img1, &img2_modified).unwrap();

        assert!(
            (diff - 10.0).abs() < 1.0,
            "Expected ~10% difference, got {}%",
            diff
        );
    }

    /// Test: dimension mismatch should return error
    #[test]
    fn test_dimension_mismatch() {
        let img1 = create_test_image(100, 100, (255, 0, 0));
        let img2 = create_test_image(200, 200, (255, 0, 0));

        let result = compare_images(&img1, &img2);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .downcast_ref::<GoldenError>()
            .is_some());
    }

    /// Test: update should copy images to golden directory
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

    /// Test: verify should pass when images match
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

    /// Test: verify should fail when images differ beyond threshold
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

    /// Test: verify should fail when golden image is missing
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

    /// Test: default threshold should be 1.5%
    #[test]
    fn test_default_threshold() {
        assert_eq!(
            DEFAULT_THRESHOLD,
            1.5,
            "Default threshold should be 1.5% as per PRD"
        );
    }
}
