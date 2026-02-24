# TODO1 - Agent 1

> Sprint: 18
> Focus Area: Config & Release Build
> Last Updated: 2026-02-24T07:51:54Z

## Tasks

- [ ] Complete application polish - config file handling
  - 📚 SKILLS: ./skills/frontend-design/SKILL.md
  - Scope: Implement persistent config file storage (verify if complete, refine acceptance criteria)
  - **SPRINT 18**

- [ ] Implement release build
  - 📚 SKILLS: ./skills/rust-best-practices/SKILL.md
  - Scope: Cross-platform binaries, size optimization (verify if complete, refine if needed)
  - **SPRINT 18**

- [ ] [DD-020] Fix CacheManager — Non-8-point padding (10, 5, 20)
  - 📚 SKILLS: ./skills/iced-rs.md, ./skills/coding-guidelines.md
  - Scope: Replace padding values: 10→8(SM) or 12(MD), 5→4(XS), 20→16(BASE) or 24(LG). Lines 252, 268, 298, 300, 342 in tempest-app/src/cache_manager.rs.
  - Fix estimate: M

## QA

- AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
