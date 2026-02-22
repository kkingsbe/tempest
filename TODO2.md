# TODO2 - Agent 2

> Sprint: 4
> Focus Area: Test Fixtures & Integration Testing (Phase 1 + Phase 3)
> Last Updated: 2026-02-22

## Tasks

- [ ] Acquire test fixtures (NEXRAD Level II data samples)
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/test-driven-development/SKILL.md
  - Scope: Download or create sample .bin files for testing decoder. Use download_fixtures.sh script.
  - Note: Critical for Phase 1 validation - no fixtures means decoder cannot be tested

- [ ] Write integration tests for S3 fetch → decode pipeline
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-engineer/references/testing.md
  - Scope: Create integration tests that verify fetched S3 data correctly decodes through the pipeline. Test with real NEXRAD data if available.
  - Note: Carried over from Sprint 3 - TODO4

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
