# TODO1 - Agent 1

> Sprint: 3
> Focus Area: Station Discovery & S3 Integration
> Last Updated: 2026-02-22

## Tasks

- [ ] Implement station discovery (enumerate NEXRAD stations with metadata)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Station ID, lat/lon, elevation, name lookup. Should populate a station registry in tempest-fetch.
  - Note: Use NOAA NEXRAD station list API or embedded station metadata

- [ ] Implement S3 integration (fetch from noaa-nexrad-level2 bucket)
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Unsigned S3 GET requests, reqwest HTTP client. Fetch NEXRAD Level II data files.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
