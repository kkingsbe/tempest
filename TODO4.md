# TODO4 - Agent 4

> Sprint: 8
> Focus Area: UI Integration - Opacity, Compositing & Controls
> Last Updated: 2026-02-23

## Tasks

- [ ] Implement opacity control for radar overlay
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/rust-engineer/SKILL.md`
  - Scope: Add alpha blending control for radar layer in tempest-render
  - Phase: Phase 4 (From Sprint 7)

- [ ] Implement map/radar compositing
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/rust-engineer/SKILL.md`
  - Scope: Composite base map tiles with radar data overlay
  - Phase: Phase 5 (From Sprint 7)
  - Note: Depends on base map tile rendering from previous sprint (now complete)

- [ ] Implement station selector UI component
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/rust-engineer/SKILL.md`, `./skills/frontend-design/SKILL.md`
  - Scope: Build station selector UI in tempest-app for choosing NEXRAD station
  - Phase: Phase 6 (From Sprint 7)

- [ ] Implement data moment switcher (REF, VEL, SW, ZDR, CC, KDP)
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/rust-engineer/SKILL.md`, `./skills/frontend-design/SKILL.md`
  - Scope: Build moment selector UI for switching between radar data moments
  - Phase: Phase 6 (From Sprint 7)

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
