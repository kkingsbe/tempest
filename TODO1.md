# TODO1 - Agent 1

> Sprint: 17
> Focus Area: Application Configuration + Release Build
> Last Updated: 2026-02-24

## Tasks

- [x] Complete application polish - config file handling
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - Scope: Implement persistent config file storage in tempest-app
  - See BACKLOG.md for full context on this Sprint 11 carryover item.
  - Fix estimate: L
  - Status: Already fully implemented - config.rs has XDG-compliant storage, migration support, load/save methods

- [x] Implement release build
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`
  - Scope: Cross-platform binaries, size optimization
  - See BACKLOG.md for full context on this Sprint 11 carryover item.
  - Fix estimate: M
  - Status: Already implemented - Cargo.toml has LTO, opt-level=3, cross-platform build.sh exists

- [x] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
