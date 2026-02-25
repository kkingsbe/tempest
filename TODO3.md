# Sprint 25 - Agent 3 Tasks

**Focus:** elevation_tilt_selector.rs

## Design Debt Items

- [ ] [DD-062] Fix ElevationTiltSelector - Multiple Primary Buttons Violation (High)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Use secondary style for selected buttons instead of primary to ensure maximum 1 primary button per view (with 10-20 elevation tilts, only one should be visually "primary")
  - 📂 Files: `tempest-app/src/elevation_tilt_selector.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-062 for details
  - ✅ Acceptance: Only one primary button exists, build passes, tests pass

- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run full test suite. If ALL errors fixed and tests pass, create '.agent_done_3' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
