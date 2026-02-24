# TODO3 - Agent 3

> Sprint: 18
> Focus Area: CI Pipeline & Test Coverage
> Last Updated: 2026-02-24T07:51:54Z

## Tasks

- [x] CI pipeline setup
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/coding-guidelines/SKILL.md
  - Scope: Configure CI with GitHub Actions. Tier 1 (unit tests), Tier 2 (integration tests), Tier 3 (full test suite) as specified in PRD. Use cargo-tarpaulin for coverage.
  - **SPRINT 18**

- [x] Test coverage tracking
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Implement coverage enforcement with cargo-tarpaulin. Targets: ≥90% decoder coverage, ≥85% overall coverage. Set ratchet at 1.5% decrease threshold.
  - **SPRINT 18**

- [x] [DD-021] Fix StationSelector — inline RGB instead of semantic constants
  - 📚 SKILLS: ./skills/iced-rs.md, ./skills/coding-guidelines.md
  - Scope: Replace inline Color::from_rgb calls with semantic constants (colors::ACCENT, colors::TEXT_SECONDARY) at lines 138-141, 200-202 in tempest-app/src/station_selector.rs.
  - Fix estimate: S

## QA

- AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_3' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
