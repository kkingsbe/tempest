# TODO4 - Agent 4

> Sprint: 4
> Focus Area: Coordinate Projection & Station Discovery (Phase 2 + Phase 3)
> Last Updated: 2026-02-22

## Tasks

- [ ] Implement coordinate projection (lat/lon to radar space)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md
  - Scope: Convert geographic coordinates (lat/lon) to radar PPI (polar) coordinates. Implement in tempest-render-core.
  - Note: Critical for Phase 2 - needed to overlay radar data on geographic maps

- [ ] Implement station discovery (enumerate NEXRAD stations with metadata)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Station ID, lat/lon, elevation, name lookup. Populate station registry in tempest-fetch.
  - Note: Carried over from Sprint 3 - TODO1. Use NOAA NEXRAD station list API or embedded station metadata.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
