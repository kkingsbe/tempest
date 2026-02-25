# Tempest Golden Reference Image Manager

CLI tool for managing golden reference images for visual regression testing in the Tempest Weather App.

## Installation

```bash
cargo build -p tempest-golden
```

## Usage

### Update Command

Copy source images to the golden directory to establish or update reference images:

```bash
cargo run -p tempest-golden -- update <source_dir> <golden_dir>
```

Example:
```bash
cargo run -p tempest-golden -- update ./screenshots ./golden
```

### Verify Command

Compare source images against golden references and verify they don't differ by more than the threshold:

```bash
cargo run -p tempest-golden -- verify <source_dir> <golden_dir>
```

Example:
```bash
cargo run -p tempest-golden -- verify ./screenshots ./golden
```

#### Options

- `--threshold <VALUE>`: Maximum allowed difference percentage (default: 1.5)

Example with custom threshold:
```bash
cargo run -p tempest-golden -- verify ./screenshots ./golden --threshold 2.0
```

## How It Works

### Image Comparison

The tool uses pixel-by-pixel comparison to calculate the percentage of pixels that differ between two images. A pixel is considered different if any of its RGB channels differ from the corresponding pixel in the other image.

### Threshold

The default threshold is 1.5% as specified in the PRD for visual regression testing. This means:
- If ≤1.5% of pixels differ → PASS
- If >1.5% of pixels differ → FAIL

### Exit Codes

- `0`: All images pass verification (within threshold)
- `1`: One or more images fail verification (exceed threshold)

## Features

- **Update**: Copies images from source to golden directory, skipping files that are already identical
- **Verify**: Compares images and reports difference percentage
- **Automatic Image Detection**: Automatically detects common image formats (PNG, JPEG, GIF, BMP, TIFF, WebP)
- **Logging**: Detailed progress output with timestamps
- **Error Handling**: Proper error handling with descriptive messages

## Development

### Running Tests

```bash
cargo test -p tempest-golden
```

### Running Clippy

```bash
cargo clippy -p tempest-golden
```

## Architecture

The tool is organized into the following modules:

- **CLI**: Command-line interface using `clap` for argument parsing
- **Image Comparison**: Pixel-by-pixel comparison algorithm
- **Error Handling**: Custom error types using `thiserror` and `anyhow`

## Dependencies

- `clap` - CLI argument parsing
- `image` - Image loading and manipulation
- `anyhow` - Error handling
- `thiserror` - Custom error types
- `log` / `env_logger` - Logging
- `chrono` - Timestamp formatting
