# BACKLOG.md

## Phase 1: Archive2 Decoder (tempest-decode)

- [x] Set up Cargo workspace with root Cargo.toml
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Create workspace members: tempest-decode, tempest-render-core, tempest-fetch, tempest-render, tempest-map

- [x] Create tempest-decode crate skeleton
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Basic crate structure with lib.rs and main.rs, add dependencies (bytes, anyhow)

- [x] Implement Message Type 31 header parsing
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/test-driven-development/SKILL.md
  - Scope: Parse WMO header, station ID, timestamp from Msg Type 31

- [x] Implement Message Type 1 (Message Header Segment) parsing
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Parse ICS, East-West, South-North, Elev, etc.

- [x] Implement radial data block parsing (REF, VEL, SW moments)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Parse radial header + data for Reflectivity, Velocity, Spectrum Width
  - **IN SPRINT 4**

- [x] Acquire test fixtures (NEXRAD Level II data samples)
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Download or create sample .bin files for testing decoder
  - **IN SPRINT 4**

- [x] Write unit tests for decoder
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md
  - Scope: Test header parsing, radial data extraction

## Phase 2: Projection & Color Mapping (tempest-render-core)

- [x] Define color tables for radar moments
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Create dBZ, velocity, ZDR color ramps
  - **IN SPRINT 8** (TODO1)

- [x] Implement coordinate projection (lat/lon to radar space)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Convert geographic coordinates to radar PPI coordinates

## Future Phases

### Phase 2: Projection & Color Mapping (tempest-render-core) (IN PROGRESS)

- [x] Define color tables for radar moments
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Create dBZ, velocity, ZDR color ramps
  - **IN SPRINT 8** (TODO1)

- [x] Implement coordinate projection (lat/lon to radar space)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Convert geographic coordinates to radar PPI coordinates
  - **COMPLETED IN PREVIOUS SPRINTS**

### Phase 3: S3 Pipeline & Cache (tempest-fetch)

- [x] Implement station discovery (enumerate NEXRAD stations with metadata)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Station ID, lat/lon, elevation, name lookup
  - **IN SPRINT 8** (TODO2)

- [x] Implement S3 integration (fetch from noaa-nexrad-level2 bucket)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Unsigned S3 GET requests, reqwest HTTP client
  - **IN SPRINT 8** (TODO2)

- [x] Implement real-time polling for new volume scans
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Configurable poll interval, stream new scans
  - **IN SPRINT 8** (TODO3)

- [x] Implement local disk cache with LRU eviction
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Cache to ~/.config/tempest/cache/, configurable size limit
  - **IN SPRINT 8** (TODO3)

- [x] Implement retry logic with exponential backoff
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Handle transient failures gracefully
  - **IN SPRINT 8** (TODO3)

### Phase 4: GPU Rendering (tempest-render)

- [x] Implement wgpu radar renderer
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Render polar radar data to map coordinate system
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement color table application in shaders
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: WGSL shaders for dBZ, velocity, SW, ZDR, CC, KDP
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement opacity control for radar overlay
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Alpha blending uniform
  - **IN SPRINT 8** (TODO4)

- [x] Implement view transform (pan/zoom/rotation)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Clip space matrix for map projection
  - **IN SPRINT 8** (TODO1)

### Phase 5: Base Map (tempest-map)

- [x] Implement tile-based map rendering
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: OpenStreetMap tiles, zoom levels 4-15
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement tile fetching and caching
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Async HTTP fetch, local disk cache
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement pan/zoom input handling
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Mouse drag, scroll, pinch gestures
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement map/radar compositing
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Alpha blending radar on top of map
  - **IN SPRINT 8** (TODO4)

### Phase 6: Station & Moment UI

- [x] Implement station selector UI
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Searchable dropdown, click-on-map selection
  - **IN SPRINT 8** (TODO4)

- [x] Implement data moment switcher
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Toolbar for REF/VEL/SW/ZDR/CC/KDP
  - **IN SPRINT 8** (TODO4)

- [x] Implement elevation tilt selector
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Dropdown/slider for sweep selection
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement color legend display
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Value-to-color mapping with labels
  - **COMPLETED IN PREVIOUS SPRINTS**

### Phase 7: Timeline & Playback

- [x] Implement timeline bar UI
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Scan tick marks, click to jump, drag scrub
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement playback controls
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Play/pause, speed (1x/2x/5x/10x), loop mode
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement time range selection
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Presets (1h/6h/24h), custom date picker
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement intelligent prefetching
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Preload next 3-5 scans during playback
  - **COMPLETED IN SPRINT 10**

### Phase 8: Offline Mode & Release (IN PROGRESS)

- [x] Implement cache management UI
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Size display, manual clear, configurable limit
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Implement offline mode detection
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Auto-detect network, browse cached data
  - **COMPLETED IN PREVIOUS SPRINTS**

- [x] Complete application polish - config file handling
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Implement persistent config file storage
  - **COMPLETED IN SPRINT 18** (TODO1)

- [x] Verify cache management UI
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Verify CacheManager functionality
  - **COMPLETED IN SPRINT 18** (TODO2)

- [x] Implement release build
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Cross-platform binaries, size optimization
  - **COMPLETED IN SPRINT 18** (TODO1)

### Sprint 13+ Items (COMPLETED IN SPRINT 18)

- [x] Test coverage tracking
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Implement coverage enforcement with cargo-tarpaulin. Targets: ≥90% decoder coverage, ≥85% overall coverage.
  - **COMPLETED IN SPRINT 18** (TODO3)

- [x] Visual regression test setup
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/frontend-design/SKILL.md
  - Scope: Set up visual regression testing with golden images. PRD specifies 1.5% threshold for visual differences.
  - **COMPLETED IN SPRINT 18** (TODO2)

- [x] CI pipeline setup
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/coding-guidelines/SKILL.md
  - Scope: Configure CI with GitHub Actions. Tier 1 (unit tests), Tier 2 (integration tests), Tier 3 (full test suite).
  - **COMPLETED IN SPRINT 18** (TODO3)

### Sprint 18 - Missing PRD Items Added

- [ ] E2E Test Harness
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Create end-to-end test harness for Tempest application. Provides infrastructure for testing full user workflows.
  - **IN SPRINT 18** (TODO4)

- [ ] Golden Reference Images CLI
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Create CLI tool to manage golden reference images for visual regression testing. Supports update/verify commands.
  - **IN SPRINT 18** (TODO4)

- [ ] Coverage Ratchet Policy
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Define and implement coverage ratchet policy - automated enforcement that prevents coverage from decreasing. Set threshold at 1.5% decrease allowed.
  - **IN SPRINT 18** (TODO4)
