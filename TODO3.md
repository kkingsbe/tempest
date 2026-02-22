# TODO3 - Agent 3

> Sprint: 2
> Focus Area: Integration
> Last Updated: 2026-02-22

## Tasks

- [ ] Integrate decoder with projection - convert raw radial data to geo-coordinates
  - 📚 SKILLS: ./skills/rust-engineer/SKILL.md, ./skills/rust-best-practices/SKILL.md
  - Scope: Wire up tempest-decode radial output to tempest-render-core projection. Ensure lat/lon points are generated correctly.

- [ ] Write integration tests for decode→project pipeline
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/rust-engineer/references/testing.md
  - Scope: Create integration tests that verify decoded radial data correctly projects to expected coordinates.

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_3' with the current date.
