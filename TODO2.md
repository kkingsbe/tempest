# Sprint 24 - Agent 2

## Tasks

- [ ] Golden Reference Images CLI
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - 🎯 Goal: Create CLI tool to manage golden reference images for visual regression testing with update/verify commands
  - 📂 Files: New CLI crate or module in `tempest-app/`
  - 🧭 Context: Key testing infrastructure item from BACKLOG.md for visual regression testing. PRD specifies 1.5% threshold for visual differences
  - ✅ Acceptance: CLI supports 'update' and 'verify' commands, can capture and compare screenshots

- [ ] [DD-056] Fix MomentSwitcher - Missing Button Padding
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Add proper button padding (≥12px vertical, ≥24px horizontal) to moment switcher buttons
  - 📂 Files: `tempest-app/src/moment_switcher.rs`
  - 🧭 Context: Lines 187-198 - both primary and secondary buttons lack .padding() call, violating iced-rs button padding requirement
  - ✅ Acceptance: Both button definitions have .padding(12) or higher

- [ ] [DD-042] Fix MomentSwitcher - Non-8-point Button Dimensions
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change button dimensions to use 8-point values (e.g., 48x48, 112x48)
  - 📂 Files: `tempest-app/src/moment_switcher.rs`
  - 🧭 Context: Lines 189-190, 195-196 - button dimensions (110x50) violate 8-point spacing rule
  - ✅ Acceptance: Button dimensions follow 8-point scale

- [ ] AGENT QA: Verify all tasks in TODO2.md are complete and correct
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`
  - 🎯 Goal: Review and verify all completed work meets acceptance criteria
  - 📂 Files: All modified files
  - 🧭 Context: Final quality check before marking sprint complete
  - ✅ Acceptance: All tasks have passing tests and meet skill requirements
