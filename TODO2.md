# TODO2 - Agent 2

> Sprint: 2
> Focus Area: Projection & Fixtures
> Last Updated: 2026-02-22

## Tasks

- [ ] Implement coordinate projection (lat/lon to radar space)
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md, ./skills/rust-engineer/SKILL.md
  - Scope: Implement WGS84 to radar space coordinate conversion in tempest-render-core. Handle azimuth/elevation to lat/lon projection.

- [ ] Acquire test fixtures (NEXRAD Level II data samples)
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md
  - Scope: Find and download real NEXRAD Level II data files for testing. Target: KTLX or similar station. PRD specifies 10 specific fixtures.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date.
