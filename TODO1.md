# TODO1 - Agent 1

> Sprint: 16
> Focus Area: Application Configuration + Timeline Design Debt
> Last Updated: 2026-02-24

## Tasks

- [ ] Complete application polish - config file handling
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - Scope: Implement persistent config file storage in tempest-app
  - See BACKLOG.md for full context on this Sprint 11 carryover item.
  - Fix estimate: L

- [ ] [DD-014] Replace raw RGB with semantic colors in Timeline component
  - 📚 SKILLS: `./skills/iced-rs/SKILL.md`
  - Component: `tempest-app/src/timeline.rs`
  - Scope: Replace raw RGB color values (lines 215, 216, 217, 433, 435) with semantic color constants using theme extended_palette()
  - Evidence:
    ```rust
    iced::Color::from_rgb(0.2, 0.6, 1.0)   // Line 215 - blue accent
    iced::Color::from_rgb(0.12, 0.12, 0.18)// Line 216 - dark background
    iced::Color::from_rgb(0.7, 0.7, 0.7)   // Line 217 - gray
    iced::Color::from_rgb(0.4, 0.7, 1.0)   // Line 433 - lighter blue
    iced::Color::from_rgb(0.3, 0.3, 0.4)   // Line 435 - dark gray
    ```
  - Fix estimate: M (15–45 min)

- [ ] AGENT QA: Run full build and test suite. Fix ALL errors. If green, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
