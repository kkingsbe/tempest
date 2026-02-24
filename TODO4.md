# TODO4 - Agent 4

> Sprint: 22
> Focus Area: Types Module Design Debt & PRD Verification
> Last Updated: 2026-02-24T15:08Z

## Tasks

- [ ] [DD-033] Fix String vs &str Parameter
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Fix in tempest-decode/src/types.rs line 389. Change `station_id: String` parameter to `&str` and convert internally if ownership needed.

- [ ] [DD-032] Fix as_str_lossy Naming Convention
  - 📚 SKILLS: ./skills/coding-guidelines/SKILL.md
  - Scope: Rename method from `as_str_lossy()` to `to_string_lossy()` in tempest-decode/src/types.rs lines 58-60. Follow Rust naming conventions (as_ for cheap refs, to_ for expensive/owned).

- [ ] PRD Verification - Phase 6-8 E2E Test Coverage
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/iced-rs/SKILL.md
  - Scope: Verify E2E tests exist for: station selection, moment switching, elevation switching, timeline scrubbing, animated playback, historical date selection, error resilience, empty station handling

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
