# BACKLOG.md

## Phase 1: Archive2 Decoder (tempest-decode)

- [ ] Set up Cargo workspace with root Cargo.toml
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Create workspace members: tempest-decode, tempest-render-core, tempest-fetch, tempest-render, tempest-map

- [ ] Create tempest-decode crate skeleton
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Basic crate structure with lib.rs and main.rs, add dependencies (bytes, anyhow)

- [ ] Implement Message Type 31 header parsing
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/test-driven-development/SKILL.md
  - Scope: Parse WMO header, station ID, timestamp from Msg Type 31

- [ ] Implement Message Type 1 (Message Header Segment) parsing
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Parse ICS, East-West, South-North, Elev, etc.

- [ ] Implement radial data block parsing (REF, VEL, SW moments)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Parse radial header + data for Reflectivity, Velocity, Spectrum Width

- [ ] Acquire test fixtures (NEXRAD Level II data samples)
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Download or create sample .bin files for testing decoder

- [ ] Write unit tests for decoder
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md
  - Scope: Test header parsing, radial data extraction

## Phase 2: Projection & Color Mapping (tempest-render-core)

- [ ] Define color tables for radar moments
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Create dBZ, velocity, ZDR color ramps

- [ ] Implement coordinate projection (lat/lon to radar space)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Convert geographic coordinates to radar PPI coordinates

## Future Phases (placeholder)

- Phase 3: S3 Pipeline & Cache (tempest-fetch)
- Phase 4: GPU Rendering (tempest-render)
- Phase 5: Base Map (tempest-map)
- Phase 6: Station & Moment UI
- Phase 7: Timeline & Playback
- Phase 8: Offline Mode & Release
