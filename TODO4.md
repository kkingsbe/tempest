# TODO4 - Agent 4

> Sprint: 16
> Focus Area: StationSelector Design Debt Fix
> Last Updated: 2026-02-24

## Tasks

- [ ] [DD-017] Replace non-8-point values in StationSelector
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`
  - Component: `tempest-app/src/station_selector.rs`
  - Scope: Replace all non-8-point spacing values (lines 167, 207, 209) with 8-point scale tokens
  - Evidence:
    ```rust
    container(details_column).padding(15).into()  // Line 167
    ...
    .spacing(5)    // Line 207
    .padding(20)   // Line 209
    ```
  - Suggested fix: Replace 15→16(MD), 5→4(XS), 20→16(BASE) or 24(LG)
  - Fix estimate: S (< 15 min)

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
