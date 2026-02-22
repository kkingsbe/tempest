# Product Requirements Document: Tempest — NEXRAD Weather Radar Application

## Overview

A real-time weather radar application built in Rust that renders NEXRAD Level II radar data on an interactive map with timeline controls. The application streams data directly from the NOAA public S3 bucket, decodes Archive2 binary format, and renders radar imagery using GPU-accelerated graphics. The system is designed for full automated testability to support development via the Gastown automated development agent.

---

## Goals

- Render NEXRAD Level II radar data from any WSR-88D station in the CONUS (and available territories) with sub-second decode-to-render latency
- Provide a timeline interface for scrubbing through historical data going back to the full extent of the NOAA archive (1991–present)
- Display radar overlays on a performant, interactive base map with standard pan/zoom/rotation controls
- Achieve comprehensive automated test coverage across all layers of the system — from binary decoding to pixel-level render output — enabling fully autonomous development workflows

---

## Architecture Overview

This is a fully self-contained, cross-platform desktop application with no cloud server component. All processing — fetching, decoding, caching, and rendering — happens locally on the user's machine. The only external dependency is the public NOAA S3 bucket for radar data.

```
┌───────────────────────────────────────────────────────────┐
│              Desktop Application (Rust)                    │
│                                                           │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                  UI Layer (egui/iced)                 │ │
│  │  ┌─────────────┐  ┌──────────────┐  ┌────────────┐  │ │
│  │  │  Base Map    │  │ Radar Render │  │  Timeline   │  │ │
│  │  │  (wgpu)      │  │  (wgpu)      │  │  Controls   │  │ │
│  │  └─────────────┘  └──────────────┘  └────────────┘  │ │
│  └──────────────────────────────────────────────────────┘ │
│                                                           │
│  ┌──────────────────────────────────────────────────────┐ │
│  │                 Core Engine                           │ │
│  │  ┌──────────────┐  ┌─────────────┐  ┌────────────┐  │ │
│  │  │ S3 Fetch &   │  │  Archive2   │  │  Local Disk │  │ │
│  │  │ HTTP Client  │  │  Decoder    │  │  Cache      │  │ │
│  │  └──────────────┘  └─────────────┘  └────────────┘  │ │
│  └──────────────────────────────────────────────────────┘ │
└────────────────────────┬──────────────────────────────────┘
                         │ HTTPS (unsigned S3 GET)
              ┌──────────┴──────────┐
              │  AWS S3 (Public)    │
              │  noaa-nexrad-level2 │
              └─────────────────────┘
```

**Key architectural decisions:**

- **GUI framework:** `iced` (Rust-native, cross-platform, wgpu-backed) for the application shell, layout, and timeline controls. Alternative: `egui` with `eframe` if rapid iteration speed is preferred over native look-and-feel.
- **GPU rendering:** `wgpu` for all radar and map rendering. Provides cross-platform GPU access (Vulkan, Metal, DX12) from a single Rust codebase with no browser dependency.
- **Map tiles:** Downloaded over HTTPS and cached to local disk. Tile source is configurable (OpenStreetMap, Stadia Maps, MapTiler). Tiles are rendered as textured quads in the wgpu pipeline alongside the radar layer.
- **Caching:** All fetched radar data and map tiles are cached to a local directory (`~/.config/tempest/cache/` or platform equivalent). No external database or server required.
- **Networking:** Standard HTTPS via `reqwest`. The only outbound connections are to the NOAA S3 bucket (radar data) and the configured map tile provider.

---

## Feature Requirements

### F1: Base Map

**Description:** An interactive map layer that serves as the canvas for radar data rendering.

**Requirements:**

- Render a tile-based or vector-based map supporting zoom levels 4–15 (continental view down to neighborhood-level)
- Support pan, zoom, and pinch gestures (touch and mouse)
- Display geographic context: state/country boundaries, city labels, roads at appropriate zoom levels
- Dark theme map style optimized for weather radar overlay visibility
- Smooth 60fps map interactions during radar rendering
- Map tiles sourced from OpenStreetMap (default) or configurable provider (Stadia Maps, MapTiler). Tiles are fetched over HTTPS and cached to local disk for offline/low-bandwidth usage.

### F2: NEXRAD Level II Data Pipeline

**Description:** The backend subsystem responsible for discovering, fetching, and decoding radar data from the NOAA S3 archive.

**Requirements:**

- **Station Discovery:** Enumerate available radar stations with metadata (site ID, lat/lon, elevation, name)
- **S3 Integration:** Fetch volume scan files from `s3://noaa-nexrad-level2/{YYYY}/{MM}/{DD}/{KXXX}/` using unsigned (no-auth) requests
- **Real-time Polling:** For the active station, poll the S3 bucket for new volume scans at a configurable interval (default: 60 seconds). Polling is the only viable approach since there is no server component to receive SNS push notifications.
- **Archive2 Decoding:** Fully decode the NEXRAD Archive2 (Level II) binary format, including:
  - Message Type 31 (digital radar data)
  - Message Type 1 (legacy format, for older archive data)
  - All data moments: Reflectivity (REF), Velocity (VEL), Spectrum Width (SW), Differential Reflectivity (ZDR), Correlation Coefficient (CC/RhoHV), Differential Phase (PHI/KDP)
  - Multiple elevation sweeps per volume scan
  - Support for all Volume Coverage Patterns (VCPs): 12, 21, 35, 212, 215, etc.
  - Handling of super-resolution data (0.5° azimuth, 250m gate spacing)
  - Graceful handling of compressed (bzip2/gzip) and uncompressed messages
- **Data Model:** Expose decoded data as structured types representing volume scans, sweeps, radials, and gates with typed moment data
- **Error Resilience:** Handle truncated files, missing moments, corrupt messages, and partial volume scans without panicking

### F3: Radar Rendering

**Description:** GPU-accelerated rendering of decoded radar data as a map overlay.

**Requirements:**

- Render polar coordinate radar data projected onto the map's coordinate system (WGS84 → Web Mercator) using `wgpu` shaders
- Support rendering of all decoded data moments with appropriate color tables:
  - Reflectivity: standard NWS color table (−30 to +75 dBZ)
  - Velocity: divergent color table (−64 to +64 kts, or m/s with configurable units)
  - Spectrum Width, ZDR, CC, KDP: standard meteorological palettes
- Allow user to switch between data moments for the current volume scan
- Render at the sweep/elevation angle selected by the user (default: lowest tilt, ~0.5°)
- Apply beam height and range-folding corrections for accurate geographic placement
- Support configurable opacity for the radar overlay
- Render smoothly during map pan/zoom — the radar layer must track the map projection in real time
- Target < 200ms from raw data availability to rendered frame

### F4: Timeline & Playback

**Description:** A timeline control for navigating through historical and recent radar data.

**Requirements:**

- Display a horizontal timeline bar showing available volume scans for the selected station
- Show timestamps for each available scan, with the current scan highlighted
- Support the following interactions:
  - Click/tap on timeline to jump to a specific scan
  - Drag scrubber to scrub through time
  - Play/pause button for animated playback
  - Step forward/backward by one volume scan
  - Configurable playback speed (1x, 2x, 5x, 10x frame intervals)
- **Looping:** Support loop mode that replays the last N scans (configurable, default: last 2 hours)
- **Time Range:** Allow selection of a time window for playback (e.g., last 1 hour, last 6 hours, last 24 hours, or custom date/time range)
- **Historical Access:** When a custom date/time is selected, fetch the corresponding volume scans from the S3 archive on demand
- **Prefetching:** Intelligently prefetch adjacent scans in the timeline direction of travel to ensure smooth playback without loading gaps
- Display a loading indicator for scans that are being fetched
- Show scan metadata: timestamp (UTC and local), VCP, and elevation angle

---

## Non-Functional Requirements

### Performance

- Decode a single Archive2 volume scan in < 100ms on modern hardware
- Render a single sweep in < 50ms (GPU time)
- Timeline playback at up to 10 fps with no dropped frames when scans are cached
- Memory usage: < 500MB for a typical session with 30 cached volume scans
- Startup to first rendered frame: < 3 seconds on a warm cache, < 8 seconds cold

### Reliability

- No panics or crashes from malformed radar data — all decode errors are captured and surfaced gracefully
- Automatic retry with exponential backoff on S3 fetch failures
- Graceful degradation when network connectivity is intermittent (display last cached data)

### Compatibility

- Target platforms: Linux (primary), macOS, Windows — all via native desktop binary
- GPU backend: Vulkan (Linux/Windows), Metal (macOS), with `wgpu` software fallback for headless/CI environments
- Minimum window size: 1024×768. Responsive layout up to 4K.
- Single-binary distribution with no external runtime dependencies (statically linked where possible)

---

## Testing & Test Coverage Requirements

Comprehensive automated testability is a foundational requirement of this system. Every layer must be independently testable with deterministic inputs and outputs, enabling the Gastown agent to develop, refactor, and validate changes autonomously without human visual inspection.

### Test Fixture Strategy

Maintain a curated set of real NEXRAD Archive2 files as test fixtures, stored via Git LFS. The fixture set must cover:

| Fixture | Purpose |
|---|---|
| Standard VCP 215 volume scan | Baseline decode and render path |
| VCP 35 (clear-air mode) | Lower PRF, fewer tilts, different gate spacing |
| VCP 12 (severe weather) | High-density scan strategy |
| Super-resolution scan (0.5° azimuth) | Tests high-resolution decode path |
| Legacy Message Type 1 file (pre-2008) | Backward compatibility with older archive data |
| Bzip2-compressed volume scan | Decompression pipeline |
| Truncated/corrupt file | Error handling paths |
| Volume scan with missing moments | Partial data handling |
| Scan from a high-altitude station (e.g., KFSX) | Beam height correction edge cases |
| Scan containing strong velocity aliasing | Velocity decode accuracy |

Each fixture must have an accompanying metadata file (JSON) documenting the expected properties: number of sweeps, elevation angles, moment availability, and known gate values at specific azimuth/range positions for golden-value testing.

### Unit Tests

**Target Coverage: ≥ 90% line coverage on decode and projection modules**

**Archive2 Decoder:**

- Decode each fixture and assert: number of sweeps, number of radials per sweep, number of gates per radial, elevation angles, azimuth angles, and moment availability
- Golden-value tests: for each fixture, assert decoded reflectivity/velocity values at 5+ specific (azimuth, range) positions against independently verified reference values (cross-referenced with Py-ART or GR2Analyst)
- Verify correct handling of all VCP types and their expected scan strategies
- Verify bzip2/gzip decompression produces identical output to known uncompressed data
- Test that truncated files produce a descriptive error (not a panic)
- Test that files with missing moments decode successfully for the moments that are present
- Benchmark: decode time for each fixture must remain below threshold (regression guard)

**Polar-to-Geographic Projection:**

- For known radar site coordinates, azimuth, range, and elevation angle, assert output lat/lon to within 0.01° of independently calculated reference values
- Test all four quadrants (N/S/E/W azimuth) and both near-range and far-range gates
- Verify beam height calculations match standard atmospheric refraction model (4/3 earth radius)
- Edge cases: 0° azimuth, 360° azimuth wrapping, maximum unambiguous range

**Color Table Mapping:**

- For each supported data moment, verify that specific input values map to the exact expected RGBA output
- Test boundary values at color table transitions (e.g., the dBZ value where green transitions to yellow)
- Test values below minimum and above maximum (should produce transparent and max-color, respectively)
- Test "no data" / "range folded" sentinel values produce the correct special-case colors

**Station Metadata:**

- Verify station lookup by ID returns correct lat/lon/elevation
- Verify no duplicate station IDs in the dataset
- Verify coordinate bounds are reasonable (all CONUS stations between 24°–50°N, 65°–125°W)

### Integration Tests

**Target Coverage: Full pipeline from raw bytes to renderable data structures**

**S3 Fetch Pipeline (with Mock S3):**

- Run a local S3-compatible server (MinIO in Docker) loaded with fixture files
- Test: list available scans for a station and date, fetch a specific volume scan, and verify the decoded output matches expected fixture metadata
- Test: polling detects new files when they are added to the mock bucket
- Test: fetch failure (server down, 404, timeout) triggers retry logic and surfaces appropriate errors
- Test: concurrent fetches for multiple scans do not deadlock or corrupt data

**Decode-to-Render Data Pipeline:**

- Feed raw fixture bytes through the full pipeline (decompress → decode → project → generate render buffers) and verify the output buffer dimensions and value ranges are correct
- Verify that switching between data moments on the same volume scan reuses the decode and only regenerates the color-mapped output
- Verify that switching elevation angles produces geometrically distinct output (different sweep radii)

**Caching Layer:**

- Verify that fetching the same scan twice returns cached data without a second S3 request (assert request count on mock)
- Verify cache eviction policy: when cache exceeds configured limit, oldest scans are evicted first
- Verify cache keys are correctly scoped by station and timestamp (no cross-contamination)

**Timeline Data Assembly:**

- Load a sequence of fixture scans with known timestamps and verify the timeline produces the correct ordered list with accurate time labels
- Verify that gaps in scan availability (missing files for a time period) are reflected in the timeline without errors
- Verify prefetch logic: when playing forward, the next N scans ahead of the playhead are fetched proactively

### Visual Regression Tests

**Target: Pixel-level validation of rendered output against golden reference images**

**Rendering Environment:**

- Use `wgpu` with a CPU/software backend (e.g., `wgpu` with `Backends::GL` or `llvmpipe` on Linux) to produce deterministic output independent of host GPU hardware
- All reference images are rendered at a fixed resolution (1920×1080) with a fixed map viewport (center lat/lon, zoom level) to ensure reproducibility
- CI environments use the same software renderer to eliminate GPU driver variance

**Test Cases:**

- For each primary data moment (REF, VEL, SW), render the baseline VCP 215 fixture at the lowest tilt and compare against a golden reference image using pixel-diff analysis
- Acceptable diff threshold: ≤ 1.5% of pixels (to account for anti-aliasing and floating-point variance across platforms)
- Test rendering at multiple zoom levels (z=6 continental, z=10 regional, z=14 local) to verify projection accuracy scales correctly
- Test rendering with 0% and 50% opacity to verify alpha blending
- Test that "no data" regions are fully transparent (no rendering artifacts outside the radar sweep)

**Golden Image Management:**

- Reference images are committed to the repository (Git LFS)
- A dedicated CLI command regenerates all golden images from current code (for intentional visual changes)
- CI fails if any visual regression test exceeds the diff threshold, with the diff image saved as a CI artifact for inspection

### End-to-End Tests

**Target: Validate the full user-facing workflow from station selection through animated playback**

**Test Harness:**

- Use a headless test harness that drives the application without a visible window, injecting input events programmatically and capturing rendered frame output via `wgpu` readback
- The harness exposes the application's internal state for assertion (current scan timestamp, loaded station, active moment, playback state) via a test-only API surface
- All E2E tests run against a mock S3 backend (MinIO in Docker) preloaded with a known sequence of volume scans

**Test Scenarios:**

- **Station selection and initial load:** Select station KLWX → verify radar data renders within 8 seconds → verify station name and metadata are displayed correctly
- **Moment switching:** Switch from REF to VEL → verify the rendered output changes (pixel diff exceeds a minimum threshold, confirming a different product is displayed)
- **Elevation switching:** Switch from tilt 1 (0.5°) to tilt 3 (~1.5°) → verify the rendered coverage area changes (reduced radius at higher elevation)
- **Timeline scrubbing:** Load a sequence of 10 scans → scrub to scan 5 → verify the displayed timestamp matches scan 5 → scrub to scan 8 → verify timestamp updates
- **Animated playback:** Start playback on a 10-scan sequence → verify that all 10 frames render in order by capturing timestamps at each frame transition → verify loop mode returns to frame 1 after frame 10
- **Historical date selection:** Select a date 30 days in the past → verify the application fetches and renders data for that date from the mock S3 archive
- **Error resilience:** Kill the mock S3 server mid-playback → verify the application displays an error state rather than crashing → restart mock S3 → verify the application recovers and resumes data access
- **Empty station:** Select a station/date with no data in mock S3 → verify the application displays an appropriate "no data available" message

### Performance / Regression Tests

**Target: Guard against decode and render performance regressions**

- Benchmark Archive2 decode time for each fixture file. CI fails if any benchmark regresses by > 15% compared to the stored baseline.
- Benchmark full pipeline (fetch from mock S3 → decode → render buffer generation) end-to-end latency. Target: < 500ms p95 for a single volume scan.
- Memory profiling: run a simulated 30-minute playback session (cycling through ~60 volume scans) and assert peak memory stays below 500MB.
- Store benchmark baselines in the repository; update them explicitly when performance-affecting changes are intentional.

### CI Pipeline Structure

```
Tier 1 — Fast (every commit / PR, < 2 min)
├── Unit: Archive2 decoder (all fixtures)
├── Unit: Polar-to-geographic projection
├── Unit: Color table mapping
├── Unit: Station metadata
├── Lint: cargo clippy, cargo fmt --check
└── Build: cargo build --release (all targets)

Tier 2 — Medium (every PR, < 10 min)
├── Integration: Mock S3 fetch pipeline (Docker: MinIO)
├── Integration: Decode-to-render pipeline
├── Integration: Caching layer
├── Integration: Timeline assembly
└── Performance: Decode benchmarks (regression check)

Tier 3 — Full (nightly + pre-release, < 30 min)
├── Visual regression: All golden image comparisons
├── E2E: Full workflow tests (Playwright or native harness)
├── E2E: Error resilience scenarios
├── Performance: Full pipeline benchmarks
└── Performance: Memory profiling
```

### Coverage Enforcement

- **Overall line coverage target: ≥ 85%** (enforced in CI via `cargo-tarpaulin` or `llvm-cov`)
- **Decoder module: ≥ 95%** — this is safety-critical parsing of an external binary format
- **Projection module: ≥ 90%** — geometric correctness is essential
- **Rendering module: ≥ 80%** — GPU code paths are partially covered by visual regression tests
- **S3/network module: ≥ 80%** — integration tests with mock S3 cover the critical paths
- Coverage must not decrease on any PR (ratchet policy)

---

## Implementation Phases

Each phase produces a fully tested, runnable artifact. No phase begins until the previous phase's test suite passes at 100%. The Gastown agent should treat each phase as an independent deliverable with its own acceptance criteria.

---

### Phase 1: Archive2 Decoder Library

**Goal:** A standalone Rust library crate (`tempest-decode`) that reads raw NEXRAD Archive2 bytes and produces structured Rust types. No UI, no networking, no rendering.

**Deliverables:**

- `tempest-decode` crate with public API:
  - `decode(bytes: &[u8]) -> Result<VolumeScan, DecodeError>`
  - `VolumeScan` containing `Vec<Sweep>`, each with `Vec<Radial>`, each with gate data per moment
  - Support for Message Type 31 (modern) and Message Type 1 (legacy/pre-2008)
  - Bzip2 and gzip decompression of compressed messages
  - All data moments: REF, VEL, SW, ZDR, CC, KDP
  - VCP metadata extraction
- Test fixture set (10 Archive2 files via Git LFS) with JSON metadata sidecars
- `DecodeError` enum with descriptive variants for all failure modes

**Tests — fully passing before Phase 2 begins:**

- Unit: Decode each of the 10 fixtures, assert sweep count, radial count, gate count, elevation angles, moment availability
- Unit: Golden-value tests — assert decoded values at 5+ specific (azimuth, range) positions per fixture against Py-ART-verified reference values
- Unit: All VCP types decode with correct scan strategy metadata
- Unit: Bzip2-compressed fixture decodes identically to its uncompressed equivalent
- Unit: Truncated fixture produces `DecodeError::Truncated`, not a panic
- Unit: Missing-moment fixture decodes successfully for moments that are present
- Benchmark: Decode time per fixture baselined and stored; CI fails on >15% regression
- Coverage: ≥ 95% line coverage on the `tempest-decode` crate

**Runnable artifact:** `cargo test` passes. A CLI binary (`tempest-decode-cli`) reads an Archive2 file from disk and prints a summary (sweep count, moments, timestamps, value at a given azimuth/range).

---

### Phase 2: Geospatial Projection & Color Mapping

**Goal:** A library crate (`tempest-render-core`) that transforms decoded radar data into render-ready buffers — polar-to-geographic projection and color table application. Still no UI or GPU code.

**Deliverables:**

- `tempest-render-core` crate with public API:
  - `polar_to_latlng(site: &RadarSite, azimuth: f64, range_m: f64, elevation_deg: f64) -> LatLng`
  - `project_sweep(site: &RadarSite, sweep: &Sweep, moment: Moment) -> ProjectedSweep` — produces a buffer of (lat, lng, value) triples
  - `colorize(projected: &ProjectedSweep, table: &ColorTable) -> Vec<[u8; 4]>` — maps values to RGBA
  - Beam height calculation using 4/3 earth radius refraction model
- `RadarSite` registry: all NEXRAD station IDs with lat/lon/elevation, loadable from embedded data
- Color tables for all moments (NWS standard palettes), defined as data (not code) for easy modification
- `ProjectedSweep` struct containing vertices and color data ready for GPU upload (but no GPU dependency)

**Tests — fully passing before Phase 3 begins:**

- Unit: Polar-to-lat/lon for known inputs across all four quadrants, near and far range, assert to within 0.01°
- Unit: Beam height at known ranges matches standard atmospheric model reference values
- Unit: 0° and 360° azimuth wrapping produces identical output
- Unit: Maximum unambiguous range gates project correctly
- Unit: Color table mapping for each moment — specific input values produce exact expected RGBA
- Unit: Color table boundary transitions (e.g., green→yellow dBZ threshold) produce correct colors on both sides
- Unit: Below-minimum, above-maximum, no-data, and range-folded sentinel values produce correct special-case outputs
- Unit: Station lookup by ID returns correct coordinates; no duplicate IDs; all CONUS stations within expected bounds
- Integration: Full pipeline — feed fixture bytes through decode → project → colorize, verify output buffer dimensions and value ranges
- Coverage: ≥ 90% on `tempest-render-core`

**Runnable artifact:** CLI tool reads an Archive2 file from disk and outputs a georeferenced PNG image of the radar sweep (using `image` crate for PNG encoding, no GPU). This serves as a visual sanity check and a reference image generator for later visual regression tests.

---

### Phase 3: S3 Data Pipeline & Local Cache

**Goal:** A library crate (`tempest-fetch`) that discovers, fetches, and caches NEXRAD data from the NOAA S3 bucket with full offline support.

**Deliverables:**

- `tempest-fetch` crate with public API:
  - `list_scans(station: &str, date: NaiveDate) -> Result<Vec<ScanMeta>, FetchError>` — lists available volume scans
  - `fetch_scan(station: &str, scan: &ScanMeta) -> Result<Bytes, FetchError>` — downloads a single scan
  - `poll_latest(station: &str) -> impl Stream<Item = ScanMeta>` — yields new scans as they appear
  - All functions are async (`tokio`)
- Local disk cache (`~/.config/tempest/cache/`):
  - Radar data cached by `{station}/{date}/{filename}`
  - Cache lookup before S3 fetch; cache write after successful fetch
  - Configurable max cache size with LRU eviction
  - Cache stats API: total size, entry count, oldest/newest entry
- HTTP client via `reqwest` with unsigned S3 GET requests
- Retry logic: exponential backoff (3 retries, 1s/2s/4s) on transient failures
- `FetchError` enum covering network, S3 404, timeout, and cache I/O errors

**Tests — fully passing before Phase 4 begins:**

- Integration (Docker/MinIO): List scans for a station/date returns expected fixture filenames
- Integration (Docker/MinIO): Fetch a scan returns bytes that decode successfully via `tempest-decode`
- Integration (Docker/MinIO): Fetch same scan twice — second request is served from cache (assert MinIO request count = 1)
- Integration (Docker/MinIO): Cache eviction — fill cache to limit, add one more, verify oldest entry is evicted
- Integration (Docker/MinIO): Polling detects new files added to mock bucket within one poll interval
- Integration (Docker/MinIO): Fetch with MinIO stopped returns `FetchError::Network`, not a panic; retry logic executes 3 attempts
- Integration (Docker/MinIO): Concurrent fetches for 10 different scans complete without deadlock or data corruption
- Unit: Cache key generation is correctly scoped by station and timestamp (no cross-contamination)
- Unit: LRU eviction order is correct (oldest access time evicted first)
- Unit: Cache size accounting is accurate after insertions and evictions
- Coverage: ≥ 80% on `tempest-fetch`

**Runnable artifact:** CLI tool (`tempest-fetch-cli`) takes a station ID and date, lists available scans, downloads one, decodes it, and prints a summary. Cached scans are served instantly on repeat runs. `--poll` flag streams new scans to stdout as they appear.

---

### Phase 4: GPU Radar Rendering with wgpu

**Goal:** A `tempest-renderer` crate that takes projected/colorized sweep data and renders it to a wgpu surface or texture. No application chrome — just a window showing a rendered radar sweep on a blank background.

**Deliverables:**

- `tempest-renderer` crate with public API:
  - `RadarRenderer::new(device: &wgpu::Device, config: RenderConfig) -> Self`
  - `RadarRenderer::update_sweep(&mut self, sweep: &ColorizedSweep)` — uploads vertex/color data to GPU
  - `RadarRenderer::render(&self, pass: &mut wgpu::RenderPass, view_transform: &ViewTransform)`
  - `RadarRenderer::set_opacity(&mut self, opacity: f32)`
  - `ViewTransform` encapsulating map projection (center, zoom, rotation) → clip space matrix
- wgpu shader (WGSL) that renders radar gates as triangulated fan geometry with per-vertex coloring
- Vertex buffer layout: position (lat/lon → projected), color (RGBA from color table)
- Support for both hardware GPU and `wgpu` software backend (for headless CI rendering)
- Opacity uniform for overlay blending

**Tests — fully passing before Phase 5 begins:**

- Unit: `ViewTransform` correctly converts lat/lon to clip space for known inputs
- Unit: Vertex buffer generation produces expected triangle count for a known sweep (radials × gates × 2 triangles per gate)
- Visual regression: Render VCP 215 REF fixture at lowest tilt on a 1920×1080 black background at 3 fixed zoom levels — pixel-diff against golden reference images, ≤ 1.5% threshold
- Visual regression: Render VEL and SW for the same fixture — verify visually distinct output (pixel diff between REF and VEL exceeds 30%)
- Visual regression: Render at 50% opacity — verify alpha blending against reference
- Visual regression: Verify no-data regions are fully transparent (sample pixels outside sweep arc, assert alpha = 0)
- Integration: Full pipeline from fixture bytes → decode → project → colorize → GPU render → readback → PNG matches reference
- Benchmark: Single sweep render time < 50ms (GPU time, measured via wgpu timestamps)
- Coverage: ≥ 80% on `tempest-renderer`

**Runnable artifact:** A minimal `iced` window that opens, renders a hardcoded fixture's radar sweep on a black background, and allows switching between moments via keyboard shortcuts. No map, no timeline — just proof that the GPU pipeline works end to end.

---

### Phase 5: Interactive Base Map

**Goal:** An interactive, tiled base map rendered in the same wgpu pipeline as the radar layer. Pan, zoom, and tile fetching with local disk caching.

**Deliverables:**

- `tempest-map` crate with public API:
  - `TileManager::new(cache_dir: PathBuf, tile_source: TileSource) -> Self`
  - `TileManager::get_tiles(viewport: &Viewport) -> Vec<Tile>` — returns tiles needed for current view, fetching/caching as needed
  - `MapRenderer::new(device: &wgpu::Device) -> Self`
  - `MapRenderer::render(&self, pass: &mut wgpu::RenderPass, tiles: &[Tile], view: &ViewTransform)`
- Tile fetching from OpenStreetMap (`https://tile.openstreetmap.org/{z}/{x}/{y}.png`) via async HTTP
- Local disk tile cache (`~/.config/tempest/cache/tiles/{source}/{z}/{x}/{y}.png`)
- Tile LOD: load appropriate zoom level tiles for current viewport, with lower-res fallback while higher-res loads
- Input handling: mouse drag to pan, scroll to zoom, pinch gesture support
- Dark theme: apply a CSS-style color inversion/remap filter to OSM tiles in the shader for better radar contrast
- Viewport math: screen coordinates ↔ lat/lon ↔ tile coordinates conversions
- Compositing: map renders first, radar layer composites on top with alpha blending

**Tests — fully passing before Phase 6 begins:**

- Unit: Viewport-to-tile-coordinate calculation — for a known center/zoom, assert correct tile x/y/z set
- Unit: Screen pixel → lat/lon → screen pixel round-trip accuracy within 1 pixel
- Unit: Tile LOD selection returns correct zoom level for given viewport dimensions
- Integration (mock HTTP server): Tile fetch caches to disk; second request serves from cache
- Integration (mock HTTP server): Failed tile fetch shows fallback lower-res tile
- Integration: Tile cache respects configured size limit with LRU eviction
- Visual regression: Render map at a fixed viewport (KLWX area, zoom 8) — pixel-diff against golden reference
- Visual regression: Render map + radar composite — verify radar overlays correctly on geographic features
- E2E: Pan from KLWX to KTLX via input injection — verify tile loading completes and final viewport is correct
- E2E: Zoom from z=6 to z=12 — verify progressive tile loading without visual gaps
- Coverage: ≥ 80% on `tempest-map`

**Runnable artifact:** Full `iced` application showing the interactive base map with the radar layer composited on top. User can pan/zoom the map and see radar data anchored to the correct geographic location. Station is hardcoded; no timeline yet.

---

### Phase 6: Station Selection & Data Moment Controls

**Goal:** UI for selecting a radar station and switching between data moments/elevation tilts. The app is now functionally useful for viewing live radar.

**Deliverables:**

- Station selector panel:
  - Searchable dropdown listing all NEXRAD stations (ID + city/state name)
  - Click-on-map station selection (station markers rendered at each site's lat/lon)
  - Selecting a station fetches the latest volume scan and renders it
  - Map auto-centers on selected station
- Data moment switcher:
  - Toolbar or dropdown to switch between available moments (REF, VEL, SW, ZDR, CC, KDP)
  - Only moments present in the current scan are selectable (others grayed out)
  - Switching moments re-colorizes without re-fetching or re-decoding
- Elevation tilt selector:
  - Dropdown or slider showing available elevation angles for the current scan
  - Switching tilts re-renders from the existing decoded volume scan
- Color table legend:
  - Rendered alongside the radar display showing the active moment's value→color mapping with labeled tick marks
- Real-time polling:
  - After station selection, begin polling for new scans at the configured interval
  - When a new scan arrives, decode and render it automatically (replacing the previous)
  - Visual indicator showing "live" status and time since last update

**Tests — fully passing before Phase 7 begins:**

- E2E: Select station KLWX from dropdown → verify radar renders within 8 seconds → verify station label displayed
- E2E: Select station by clicking map marker → verify same behavior as dropdown selection
- E2E: Switch from REF to VEL → verify rendered output changes (pixel diff > 30% threshold)
- E2E: Switch from tilt 1 to tilt 3 → verify rendered coverage area changes (smaller radius at higher elevation)
- E2E: Select station with missing ZDR moment → verify ZDR option is disabled in UI
- E2E: Verify color legend updates when switching moments (REF legend shows dBZ, VEL shows kts)
- Integration (mock S3): Polling detects new scan and auto-renders within one poll interval + decode time
- Unit: Station search filters correctly (typing "ster" matches "Sterling" / KLWX)
- Coverage: ≥ 80% on new UI code

**Runnable artifact:** Fully interactive app — select any NEXRAD station, view live radar with auto-refresh, switch moments and tilts. No timeline/history yet.

---

### Phase 7: Timeline & Historical Playback

**Goal:** Timeline control for scrubbing through recent and historical radar data with animated playback. This completes the core feature set.

**Deliverables:**

- Timeline bar UI:
  - Horizontal bar at bottom of window showing available scans as tick marks
  - Current scan highlighted with timestamp label (UTC and local time)
  - Click to jump, drag scrubber to scrub
  - Step forward/backward buttons (single scan increment)
- Playback controls:
  - Play/pause toggle
  - Speed selector: 1x, 2x, 5x, 10x (frame interval multiplier)
  - Loop mode toggle: when enabled, playback wraps from last scan to first
- Time range selection:
  - Quick presets: last 1h, 2h, 6h, 12h, 24h
  - Custom date/time picker for historical access
  - Selecting a range triggers batch fetch of scan metadata for the period
- Intelligent prefetching:
  - During forward playback, prefetch the next 3–5 scans ahead of the playhead
  - During scrubbing, cancel pending prefetches and fetch the target scan
  - Prefetch budget is configurable (max concurrent fetches)
- Loading states:
  - Scans that are fetching show a loading indicator on the timeline
  - Playback pauses on unfetched frames (buffering indicator) and resumes when ready
- Scan metadata display:
  - Timestamp (UTC + local), VCP, elevation angle shown for current scan

**Tests — fully passing before Phase 8 begins:**

- E2E: Load 10 mock scans → scrub to scan 5 → verify displayed timestamp matches scan 5 → scrub to scan 8 → verify timestamp updates
- E2E: Play 10-scan sequence → capture frame timestamps → verify all 10 frames rendered in order → verify loop wraps to frame 1
- E2E: Set speed to 5x → verify frame interval is 1/5th of 1x interval
- E2E: Step forward 3 times from scan 1 → verify scan 4 is displayed
- E2E: Select "last 2 hours" preset → verify timeline range matches (assert first scan timestamp ≥ now - 2h)
- E2E: Select a historical date 30 days ago → verify scans are fetched from mock S3 and timeline populates
- E2E: During playback, verify next 3 scans ahead of playhead are in cache (assert no fetch delay on advance)
- E2E: Start playback, kill mock S3 → verify buffering indicator appears → restart mock S3 → verify playback resumes
- E2E: Select station/date with no data → verify "no data available" message displayed
- Integration (mock S3): Batch metadata fetch for a 6-hour window returns correct scan count
- Integration: Prefetch cancellation — start prefetching, then scrub to distant position, verify cancelled fetches don't consume resources
- Unit: Timeline layout — N scans evenly distributed across bar width; current scan highlight position is correct
- Unit: Playback timer fires at correct intervals for each speed setting
- Coverage: ≥ 85% on timeline/playback code

**Runnable artifact:** Complete application with full feature set — station selection, live radar, moment/tilt switching, and historical timeline with playback. This is the v1 product.

---

### Phase 8: Offline Mode & Cache Management

**Goal:** Explicit offline support with a cache management UI, polish, and release packaging.

**Deliverables:**

- Cache management UI:
  - Settings panel showing total cache size (radar + map tiles), entry count
  - Breakdown by station and date range
  - Manual cache clear (all, by station, by date range)
  - Configurable max cache size with slider (default: 2 GB)
  - Visual indicator when data is served from cache vs. live fetch
- Offline mode:
  - Detect network unavailability and switch to offline mode automatically
  - In offline mode, only cached stations/dates are browsable (UI reflects available cache)
  - "Offline" indicator in status bar
  - Graceful return to live mode when connectivity restores
- Application polish:
  - Keyboard shortcuts (space = play/pause, left/right = step, +/- = speed, 1-6 = moments)
  - Window title shows current station and timestamp
  - Configurable preferences file (`~/.config/tempest/config.toml`): default station, cache size, poll interval, preferred units (metric/imperial)
  - Startup: restore last-used station and resume from live data
- Release build:
  - `cargo build --release` producing single binary per platform
  - CI builds for Linux (x86_64), macOS (x86_64 + aarch64), Windows (x86_64)
  - Binary size optimization (`lto = true`, `strip = true`, `opt-level = "z"` evaluation)

**Tests — fully passing for v1 release:**

- E2E: Disconnect network → verify offline indicator appears → browse cached data → verify renders succeed
- E2E: Disconnect network → attempt to load uncached station → verify "not available offline" message
- E2E: Reconnect network → verify live polling resumes within one poll interval
- E2E: Set cache limit to 50 MB → fill beyond limit → verify oldest entries evicted → verify cache size stays within limit
- E2E: Clear cache for a specific station → verify that station's data requires re-fetch
- E2E: Verify all keyboard shortcuts trigger correct actions
- E2E: Kill and restart app → verify it opens to last-used station
- Integration: Config file round-trip — write preferences, restart, verify preferences loaded
- Performance: 30-minute simulated session (60 volume scans cycling) → peak memory < 500 MB
- Performance: Full pipeline benchmark (mock S3 fetch → decode → project → colorize → render) < 500ms p95
- Build: `cargo build --release` succeeds on all 3 platforms in CI
- Build: Binary size < 50 MB (stretch goal: < 30 MB)
- Coverage: Overall project ≥ 85%

**Runnable artifact:** Production-ready v1 binary. Single file, launches instantly, fully functional online and offline.

---

### Phase Dependency Summary

```
Phase 1: Decoder ──────────────────────────────┐
                                                │
Phase 2: Projection & Color ───────────────────┤
                                                │
Phase 3: S3 Pipeline & Cache ──────────────────┤
                                                │
Phase 4: GPU Rendering ────────────────────────┤
         (depends on Phase 1, 2)               │
                                                │
Phase 5: Base Map ─────────────────────────────┤
         (depends on Phase 4)                  │
                                                │
Phase 6: Station & Moment UI ──────────────────┤
         (depends on Phase 3, 4, 5)            │
                                                │
Phase 7: Timeline & Playback ──────────────────┤
         (depends on Phase 3, 6)               │
                                                │
Phase 8: Offline, Polish & Release ────────────┘
         (depends on all prior phases)
```

**Critical path:** Phase 1 → Phase 2 → Phase 4 → Phase 5 → Phase 6 → Phase 7 → Phase 8

**Parallel work possible:** Phase 3 (S3 pipeline) can be developed in parallel with Phases 2 and 4, as it has no dependency on rendering. It only needs to be complete before Phase 6 integrates it.

---


- Multi-radar mosaic / composite view (national map)
- Derived products (storm-relative velocity, echo tops, VIL)
- NWS alerts / SPC watches / warning polygons
- Dual-pol analysis tools (hydrometeor classification)
- User accounts, settings persistence to cloud, or cloud sync
- Mobile builds (iOS/Android)
- Web/WASM target — this is a native desktop application only

---

## Resolved Decisions

| Decision | Choice | Rationale |
|---|---|---|
| GUI framework | `iced` | Rust-native, strong layout system, built-in wgpu integration, native look-and-feel |
| Map tile provider | OpenStreetMap | Free, no API key, sufficient for v1 |
| Offline mode | Explicit feature | Local disk cache with cache management UI for both radar and map tiles |
| Distribution | Single binary | `cargo build --release` producing a single statically-linked binary per platform |