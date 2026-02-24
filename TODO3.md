# TODO3 - Agent 3

> Sprint: 17
> Focus Area: Test Coverage & CI Setup
> Last Updated: 2026-02-24

## Tasks

- [ ] Test coverage tracking
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - Scope: Implement coverage enforcement with targets: ≥90% decoder coverage, ≥85% overall coverage. Use cargo-tarpaulin or similar tool.
  - See BACKLOG.md Sprint 13+ items for full context.
  - Fix estimate: M

- [ ] CI pipeline setup
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/coding-guidelines/SKILL.md`
  - Scope: Configure CI with Tier 1 (unit tests), Tier 2 (integration tests), Tier 3 (full test suite) as specified in PRD.
  - Fix estimate: L

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_3' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
