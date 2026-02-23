# TODO1 - Agent 1

> Sprint: 13
> Focus Area: Golden-value tests and Coverage
> Last Updated: 2026-02-23

## Tasks

- [x] Implement golden-value tests for Tempest decoder
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - Scope: Verify decoded values at specific positions match expected values. Use synthetic fixtures.
- [x] Set up cargo-tarpaulin for coverage tracking
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`
  - Scope: Configure ≥95% decoder, ≥90% render-core, ≥85% overall targets in .cargo/config
- [x] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
