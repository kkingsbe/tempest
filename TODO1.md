# TODO1 - Agent 1

> Sprint: 8
> Focus Area: Rendering Pipeline - Color Tables & View Transform
> Last Updated: 2026-02-23

## Tasks

- [ ] Define color tables for radar moments (dBZ, velocity, ZDR color ramps)
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/rust-engineer/SKILL.md`
  - Scope: Implement color lookup tables in tempest-render-core for REF, VEL, SW, ZDR, CC, KDP moments
  - Phase: Phase 2 (Partially complete from Sprint 7)

- [ ] Implement view transform (pan/zoom/rotation)
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`, `./skills/rust-engineer/SKILL.md`
  - Scope: Implement view transformation matrix in tempest-render for interactive radar display
  - Phase: Phase 4 (From Sprint 7)

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
