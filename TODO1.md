# Sprint 25 - Agent 1 Tasks

> ⚠️ Rebalanced from TODO4.md by Architect on 2026-02-25

**Focus:** moment_switcher.rs, color_legend.rs

## Design Debt Items

- [x] [DD-056] Fix MomentSwitcher - Missing Button Padding (High)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Add `.padding(12)` or `.padding([12, 24])` to button definitions to ensure proper button padding (minimum 12px vertical, 24px horizontal)
  - 📂 Files: `tempest-app/src/moment_switcher.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-056 for details
  - ✅ Acceptance: All buttons have proper padding, build passes, tests pass

- [x] [DD-061] Fix MomentSwitcher - Multiple Primary Buttons Violation (High)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change selected buttons to use secondary_button_style instead of primary_button_style to ensure maximum 1 primary button per view
  - 📂 Files: `tempest-app/src/moment_switcher.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-061 for details
  - ✅ Acceptance: Only one primary button exists, build passes, tests pass

- [x] [DD-042] Fix MomentSwitcher - Non-8-point Button Dimensions (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change button dimensions from 110x50 to 8-point compliant values (e.g., 112x48 or 48x48)
  - 📂 Files: `tempest-app/src/moment_switcher.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-042 for details
  - ✅ Acceptance: Button dimensions use 8-point values, build passes, tests pass

- [x] [DD-051] Fix ColorLegend - Section Spacing Below Minimum (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change `.spacing(spacing::SM)` to `.spacing(spacing::LG)` between title, color bar row, and min label sections
  - 📂 Files: `tempest-app/src/color_legend.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-051 for details
  - ✅ Acceptance: Section spacing uses LG (24px), build passes, tests pass

- [x] [DD-052] Fix ColorLegend - Container Padding Below Minimum (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Ensure container padding is at least BASE (16px) for all containers in color_legend.rs
  - 📂 Files: `tempest-app/src/color_legend.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-052 for details
  - ✅ Acceptance: Container padding is at least 16px, build passes, tests pass

- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run full test suite. If ALL errors fixed and tests pass, create '.agent_done_1' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
