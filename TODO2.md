# TODO2 - Agent 2

> Sprint: 18
> Focus Area: Cache UI + Visual Testing
> Last Updated: 2026-02-24T07:51:54Z

## Tasks

- [ ] Verify cache management UI
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Verify cache_manager.rs UI renders correctly - verify CacheManager functionality, size display, manual clear, configurable limit work as expected
  - **SPRINT 18**

- [ ] Visual regression test setup
  - 📚 SKILLS: ./skills/test-driven-development/SKILL.md, ./skills/frontend-design/SKILL.md
  - Scope: Set up visual regression testing with golden images. PRD specifies **1.5% threshold** (not 3%) for visual differences. Fix threshold to 1.5%.
  - **SPRINT 18**

- [ ] [DD-019] Fix ColorLegend — spacing(0) violation
  - 📚 SKILLS: ./skills/iced-rs.md, ./skills/coding-guidelines.md
  - Scope: Replace spacing(0) with spacing(XXS=2) at line 129 in tempest-app/src/color_legend.rs.
  - Fix estimate: S

## QA

- AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
