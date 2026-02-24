# TODO2 - Agent 2

> Sprint: 16
> Focus Area: Release Build + CacheManager Design Debt
> Last Updated: 2026-02-24

## Tasks

- [ ] Implement release build
  - 📚 SKILLS: `./skills/rust-best-practices/SKILL.md`
  - Scope: Cross-platform binaries, size optimization
  - See BACKLOG.md for full context on this Sprint 11 carryover item.
  - Fix estimate: M

- [ ] [DD-012] Fix non-8-point spacing in CacheManager
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`
  - Component: `tempest-app/src/cache_manager.rs`
  - Scope: Replace all non-8-point values (lines 259, 264, 275, 297, 309, 325, 331, 343, 345, 350, 355, 357) with 8-point scale tokens
  - Evidence:
    ```rust
    .padding(10)   // Line 259
    .padding(10)   // Line 264
    .padding(5)    // Line 275
    .spacing(10)   // Line 297
    .padding(10)   // Line 309
    .spacing(10)   // Lines 325, 331, 343
    .size(15)      // Lines 345, 350
    .spacing(5)    // Line 355
    .padding(20)   // Line 357
    ```
  - Fix estimate: M (15–45 min)

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
