# Sprint 25 - Agent 2 Tasks

**Focus:** main.rs

## Design Debt Items

- [ ] [DD-059] Fix main.rs - Root Layout Issues (High)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Add `.spacing(spacing::LG)` or `.spacing(spacing::BASE)` to the main view column to provide proper section spacing between components
  - 📂 Files: `tempest-app/src/main.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-059 for details
  - ✅ Acceptance: Main view has proper spacing between components, build passes, tests pass

- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run full test suite. If ALL errors fixed and tests pass, create '.agent_done_2' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
