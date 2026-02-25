# Sprint 24 - Agent 1

## Tasks

- [ ] E2E Test Harness
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - 🎯 Goal: Create end-to-end test harness for Tempest application providing infrastructure for testing full user workflows
  - 📂 Files: `tempest-app/tests/e2e/`, `tempest-app/src/`
  - 🧭 Context: This is a key testing infrastructure item from BACKLOG.md needed to enable comprehensive E2E testing
  - ✅ Acceptance: Test harness can launch app, simulate user interactions, and verify state changes

- [ ] [DD-049] Fix ElevationTiltSelector - Button Padding Missing
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Add proper button padding (≥12px vertical, ≥24px horizontal) to elevation tilt selector buttons
  - 📂 Files: `tempest-app/src/elevation_tilt_selector.rs`
  - 🧭 Context: Lines 136-148 - buttons lack padding, violating iced-rs button padding requirement
  - ✅ Acceptance: All buttons in ElevationTiltSelector have .padding([12, 16]) or higher

- [ ] [DD-051] Fix ColorLegend - Section Spacing Below Minimum
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Increase section spacing from SM (8px) to LG (24px) between distinct sections
  - 📂 Files: `tempest-app/src/color_legend.rs`
  - 🧭 Context: Line 170 - spacing between title, color bar row, and min label violates mandatory section spacing rule
  - ✅ Acceptance: Section spacing is at least LG (24px)

- [ ] AGENT QA: Verify all tasks in TODO1.md are complete and correct
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`
  - 🎯 Goal: Review and verify all completed work meets acceptance criteria
  - 📂 Files: All modified files
  - 🧭 Context: Final quality check before marking sprint complete
  - ✅ Acceptance: All tasks have passing tests and meet skill requirements
