# Golden Reference Images CLI - Research Summary

## Overview
This document summarizes the research findings for the Golden Reference Images CLI task (TODO2.md). **Key Finding: The CLI tool already exists and is fully implemented.**

---

## 1. Recommended Location for New CLI

**Finding: Already implemented as separate workspace crate at [`tempest-golden/`](tempest-golden/)**

The CLI was created as a separate workspace crate rather than a module in `tempest-app`. This is the correct approach because:
- Better separation of concerns (testing infrastructure is separate from the main app)
- Can be run independently without loading the full GUI application
- Follows Rust best practices for workspace organization

---

## 2. Proposed CLI Command Structure

**Finding: Already implemented with two commands**

```
tempest-golden update <SOURCE_DIR> <GOLDEN_DIR>
tempest-golden verify <SOURCE_DIR> <GOLDEN_DIR> [--threshold <VALUE>]
```

### Implemented Commands:

| Command | Description |
|---------|-------------|
| `update` | Copy source images to golden directory |
| `verify` | Compare source images against golden references |

### CLI Structure (from [`tempest-golden/src/main.rs`](tempest-golden/src/main.rs:251)):
```rust
#[derive(Subcommand)]
enum Commands {
    /// Update golden reference images
    Update {
        #[arg(value_name = "SOURCE_DIR")]
        source_dir: PathBuf,
        
        #[arg(value_name = "GOLDEN_DIR")]
        golden_dir: PathBuf,
    },
    
    /// Verify current images against golden references
    Verify {
        #[arg(value_name = "SOURCE_DIR")]
        source_dir: PathBuf,
        
        #[arg(value_name = "GOLDEN_DIR")]
        golden_dir: PathBuf,
        
        #[arg(long, default_value_t = DEFAULT_THRESHOLD)]
        threshold: f64,
    },
}
```

---

## 3. Key Dependencies

**Finding: All required dependencies already configured**

From [`tempest-golden/Cargo.toml`](tempest-golden/Cargo.toml:1):

| Dependency | Version | Purpose |
|------------|---------|---------|
| `clap` | 4.5 | CLI argument parsing with derive macros |
| `image` | 0.25 | Image loading and processing for comparison |
| `similar` | 2.4 | Text/image difference comparison |
| `anyhow` | 1.0 | Error handling |
| `thiserror` | 1.0 | Custom error types |
| `log` / `env_logger` | 0.11 | Logging |
| `chrono` | 0.4 | Timestamps for logging |

The `image` crate is already used in the main `tempest-app` (version 0.25) and is available workspace-wide.

---

## 4. Golden File Storage Location

**Finding: Already exists at [`tempest-render/tests/golden/`](tempest-render/tests/golden/)**

Existing golden reference images:

| File | Description |
|------|-------------|
| `continental_view.png` | Wide-area radar view (zoom 6) |
| `regional_view.png` | Regional radar view (zoom 10) |
| `local_view.png` | Local radar view (zoom 14) |
| `reflectivity_sweep.png` | REF moment sweep |
| `velocity_sweep.png` | VEL moment sweep |
| `spectrum_width_sweep.png` | SW moment sweep |
| `reflectivity_0_percent_opacity.png` | 0% opacity test |
| `reflectivity_50_percent_opacity.png` | 50% opacity test |

---

## 5. Visual Regression Testing Implementation

### PRD Requirements Met:

| Requirement | Status | Implementation |
|-------------|--------|----------------|
| 1.5% threshold | ✅ Implemented | `MAX_DIFF_THRESHOLD: f64 = 0.015` in [`visual_regression.rs`](tempest-render/tests/visual_regression.rs:31) |
| Pixel-diff analysis | ✅ Implemented | `compare_images()` function in [`tempest-golden/src/main.rs`](tempest-golden/src/main.rs:42) |
| Multiple zoom levels | ✅ Available | Golden images at continental, regional, local views |
| Multiple moments | ✅ Available | REF, VEL, SW golden images |

### Test Infrastructure:

- **Visual regression tests**: [`tempest-render/tests/visual_regression.rs`](tempest-render/tests/visual_regression.rs) (18 tests)
- **E2E test harness**: [`tempest-app/tests/e2e/app_harness_test.rs`](tempest-app/tests/e2e/app_harness_test.rs)
- **CLI tests**: 4 tests in `tempest-golden`

---

## 6. Implementation Details

### Image Comparison Algorithm
From [`tempest-golden/src/main.rs:42-71`](tempest-golden/src/main.rs:42):

```rust
fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> Result<f64> {
    // Convert to RGB8 for consistent comparison
    let rgb1 = img1.to_rgb8();
    let rgb2 = img2.to_rgb8();
    
    // Count pixels that differ in any channel
    for (p1, p2) in rgb1.pixels().zip(rgb2.pixels()) {
        if p1 != p2 {
            diff_pixels += 1;
        }
    }
    
    // Return percentage difference
    Ok((diff_pixels as f64 / total_pixels) * 100.0)
}
```

### Error Handling
Custom error type [`GoldenError`](tempest-golden/src/main.rs:17) with:
- `Image(ImageError)` - Image loading/parsing errors
- `Io(std::io::Error)` - File system errors
- `DimensionMismatch` - When comparing images of different sizes
- `VerificationFailed` - When difference exceeds threshold

---

## 7. Running the CLI

```bash
# Update golden references
cargo run -p tempest-golden -- update ./output ./tests/golden

# Verify with default 1.5% threshold
cargo run -p tempest-golden -- verify ./output ./tests/golden

# Verify with custom threshold
cargo run -p tempest-golden -- verify ./output ./tests/golden --threshold 2.0
```

---

## 8. Recommendations

The research shows that the Golden Reference Images CLI is **already fully implemented**. No additional work is needed for the core functionality.

### Optional Enhancements (if needed):
1. **Add screenshot capture for tempest-app**: Currently golden images are in `tempest-render`. Could add capability to capture screenshots from the running GUI.
2. **CI integration**: The PRD mentions CI should fail with diff images saved as artifacts - verify this is configured in `.github/workflows/`.
3. **Add more test scenarios**: Currently covers REF, VEL, SW at various zoom levels. Could add ZDR, CC, KDP moments.

---

## 9. Conclusion

| Item | Status |
|------|--------|
| CLI crate location | ✅ `tempest-golden/` workspace member |
| `update` command | ✅ Implemented |
| `verify` command | ✅ Implemented |
| 1.5% threshold | ✅ Implemented |
| Golden images | ✅ 8 images in `tempest-render/tests/golden/` |
| Visual regression tests | ✅ 18 tests in `tempest-render` |
| Unit tests | ✅ 4 tests in `tempest-golden` |

**The TODO2.md task "Golden Reference Images CLI" is already COMPLETE.**

---

## References

- [`tempest-golden/Cargo.toml`](tempest-golden/Cargo.toml)
- [`tempest-golden/src/main.rs`](tempest-golden/src/main.rs)
- [`tempest-render/tests/visual_regression.rs`](tempest-render/tests/visual_regression.rs)
- [`tempest-render/tests/golden/`](tempest-render/tests/golden/)
- PRD.md lines 259-262 (1.5% threshold requirement)
- BACKLOG.md lines 226-229 (CLI requirements)
