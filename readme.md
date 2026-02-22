# Tempest — NEXRAD Weather Radar Application

A real-time weather radar application built in Rust that renders NEXRAD Level II radar data on an interactive map with timeline controls. The application streams data directly from the NOAA public S3 bucket, decodes Archive2 binary format, and renders radar imagery using GPU-accelerated graphics.

## Overview

Tempest is a fully self-contained, cross-platform desktop application that processes weather radar data locally. All processing — fetching, decoding, caching, and rendering — happens on your machine. The only external dependency is the public NOAA S3 bucket for radar data.

### Key Features

- **NEXRAD Level II Data Pipeline**: Fetches and decodes radar data from any WSR-88D station in the CONUS
- **Real-time Polling**: Automatically polls NOAA S3 for new volume scans
- **GPU-Accelerated Rendering**: Uses wgpu for cross-platform GPU rendering
- **Interactive Timeline**: Navigate through historical radar data going back to 1991
- **Offline Support**: Local caching of radar data and map tiles
- **Comprehensive Testing**: Built for automated testability with extensive test coverage

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Desktop Application (Rust)               │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                    UI Layer (iced)                      │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │ │
│  │  │  Base Map    │  │ Radar Render │  │  Timeline    │   │ │
│  │  │  (wgpu)      │  │  (wgpu)      │  │  Controls    │   │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                     Core Engine                         │ │
│  │  ┌──────────────┐  ┌─────────────┐  ┌────────────┐     │ │
│  │  │ S3 Fetch &   │  │  Archive2   │  │  Local     │     │ │
│  │  │ HTTP Client  │  │  Decoder    │  │  Cache     │     │ │
│  │  └──────────────┘  └─────────────┘  └────────────┘     │ │
│  └────────────────────────────────────────────────────────┘ │
└─────────────────────────────────┬───────────────────────────┘
                                  │ HTTPS
                    ┌─────────────┴─────────────┐
                    │      AWS S3 (Public)      │
                    │     noaa-nexrad-level2     │
                    └───────────────────────────┘
```

## Workspace Crates

| Crate | Purpose |
|-------|---------|
| [`tempest-decode`](tempest-decode/) | NEXRAD Archive2 decoder library — parses Level II binary radar data |
| [`tempest-render-core`](tempest-render-core/) | Geospatial projection and color mapping — transforms radar data for rendering |
| [`tempest-fetch`](tempest-fetch/) | S3 data pipeline and local cache — fetches radar data from NOAA |
| [`tempest-render`](tempest-render/) | GPU radar rendering with wgpu (Phase 4) |
| [`tempest-map`](tempest-map/) | Interactive base map with tile caching (Phase 5) |

### tempest-decode

The Archive2 decoder library handles parsing of NEXRAD Level II binary format:

- **Message Types**: Supports both Message Type 31 (modern digital radar data) and Message Type 1 (legacy format)
- **Data Moments**: Decodes Reflectivity (REF), Velocity (VEL), Spectrum Width (SW), Differential Reflectivity (ZDR), Correlation Coefficient (CC), and Differential Phase (KDP)
- **Compression**: Handles both bzip2 and gzip compressed messages
- **Error Handling**: Graceful handling of truncated files, missing moments, and partial volume scans

```rust
use tempest_decode::{decode, VolumeScan};

let data = std::fs::read("radar_data.ar2v")?;
let volume = decode(&data)?;
println!("Station: {}", volume.station_id);
println!("Sweeps: {}", volume.sweeps.len());
```

### tempest-render-core

Provides geospatial projection and color mapping:

- **Polar-to-Geographic Projection**: Converts radar polar coordinates (azimuth, range, elevation) to WGS84 lat/lon
- **Beam Height Calculation**: Uses 4/3 earth radius refraction model
- **Color Tables**: Standard NWS color tables for all data moments (−30 to +75 dBZ for reflectivity)
- **Station Registry**: Contains metadata for all NEXRAD stations (lat/lon/elevation)

```rust
use tempest_render_core::{project_volume_scan, RadarSite, get_station};

let site = get_station("KTLX").unwrap();
let projected = project_volume_scan(&volume, &site, RadarMoment::Reflectivity);
```

### tempest-fetch

Handles data acquisition from NOAA S3:

- **Station Discovery**: Enumerates available radar stations with metadata
- **S3 Integration**: Fetches volume scans from `s3://noaa-nexrad-level2/`
- **Real-time Polling**: Polls for new volume scans at configurable intervals
- **Local Caching**: Disk cache with LRU eviction policy

```rust
use tempest_fetch::{list_scans, fetch_scan, get_station};

let station = get_station("KTLX").unwrap();
let scans = list_scans("KTLX", date).await?;
let data = fetch_scan("KTLX", &scan).await?;
```

## Building

### Prerequisites

- Rust 1.70 or later
- For GPU rendering: Vulkan, Metal, or DX12 compatible GPU

### Build Commands

```bash
# Build the entire workspace
cargo build --workspace

# Build a specific crate
cargo build -p tempest-decode

# Run tests
cargo test --workspace

# Build release version
cargo build --release --workspace
```

### Running

```bash
# Run the decode CLI (example)
cargo run -p tempest-decode --example gen_fixtures

# Run tests for a specific crate
cargo test -p tempest-decode
cargo test -p tempest-fetch
cargo test -p tempest-render-core
```

## Data Sources

### NOAA NEXRAD Level II Archive

- **Bucket**: `noaa-nexrad-level2`
- **Region**: `us-east-1`
- **Access**: Public (no authentication required)
- **Path Format**: `s3://noaa-nexrad-level2/{YYYY}/{MM}/{DD}/{KXXX}/`
- **Archive**: Data available from 1991 to present

### Map Tiles

Default tile source is OpenStreetMap. Configurable for other providers (Stadia Maps, MapTiler).

## Supported Data Moments

| Moment | Code | Unit | Description |
|--------|------|------|-------------|
| Reflectivity | REF | dBZ | Radar echo intensity |
| Velocity | VEL | m/s | Radial velocity toward/away from radar |
| Spectrum Width | SW | m/s | Velocity variance |
| Differential Reflectivity | ZDR | dB | Rain/hail discrimination |
| Correlation Coefficient | CC | - | Hydrometeor identification |
| Differential Phase | KDP | degrees | Rain rate estimation |

## Testing

The project follows a comprehensive testing strategy:

### Test Tiers

1. **Unit Tests** (≥90% target on decoder/projection modules)
   - Individual function validation
   - Golden-value tests against known inputs
   - Boundary condition testing

2. **Integration Tests**
   - Full pipeline from S3 fetch to decoded output
   - Cache behavior verification
   - Mock S3 (MinIO) testing

3. **Visual Regression Tests**
   - Pixel-level render output validation
   - Multi-zoom level verification

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test

# Run with coverage
cargo tarpaulin --workspace
```

## Project Phases

The project was developed in phases:

1. **Phase 1**: Archive2 Decoder Library — [`tempest-decode`](tempest-decode/)
2. **Phase 2**: Geospatial Projection & Color Mapping — [`tempest-render-core`](tempest-render-core/)
3. **Phase 3**: S3 Data Pipeline & Local Cache — [`tempest-fetch`](tempest-fetch/)
4. **Phase 4**: GPU Radar Rendering — [`tempest-render`](tempest-render/) (in progress)
5. **Phase 5**: Interactive Base Map — [`tempest-map`](tempest-map/) (in progress)

## Documentation

- [PRD.md](PRD.md) — Product Requirements Document with detailed specifications
- [LEARNINGS.md](LEARNINGS.md) — Development learnings and notes
- [docs/](docs/) — Additional documentation

## License

This project is for educational and research purposes. NEXRAD data is provided by NOAA and the NWS.

## References

- [NOAA NEXRAD Archive](https://www.ncdc.noaa.gov/nexradinv/)
- [NEXRAD Level II Data Format](https://www.roc.noaa.gov/WSR88D/Level_II/Level_II_Format.asp)
- [wgpu Graphics Library](https://wgpu.rs/)
- [iced GUI Framework](https://iced.rs/)
