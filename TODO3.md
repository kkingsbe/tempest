# Sprint 24 - Agent 3

## Tasks

- [ ] Coverage Ratchet Policy
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`, `./skills/rust-best-practices/SKILL.md`
  - 🎯 Goal: Define and implement coverage ratchet policy - automated enforcement that prevents coverage from decreasing with threshold at 1.5% decrease allowed
  - 📂 Files: `scripts/coverage-ratchet.sh`, `Cargo.toml`
  - 🧭 Context: Key testing infrastructure from BACKLOG.md to enforce minimum coverage standards
  - ✅ Acceptance: Ratchet script enforces coverage thresholds, fails CI if coverage drops >1.5%

- [ ] [DD-048] Fix ElevationTiltSelector - Container Padding Below Minimum
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Increase container padding from 12px to BASE (16px) or higher
  - 📂 Files: `tempest-app/src/elevation_tilt_selector.rs`
  - 🧭 Context: Line 175 - container padding (12px) violates mandatory minimum of BASE (16px)
  - ✅ Acceptance: Container padding is at least BASE (16px)

- [ ] [DD-046] Fix Timeline - Inter-element Spacing Below Minimum
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change spacing from XXS (2px) to XS (4px) or SM (8px) between interactive elements
  - 📂 Files: `tempest-app/src/timeline.rs`
  - 🧭 Context: Lines 415, 458 - spacing between tick marks and labels violates proximity rule
  - ✅ Acceptance: Spacing between interactive elements is at least XS (4px)

- [ ] AGENT QA: Verify all tasks in TODO3.md are complete and correct
  - 📚 SKILLS: `./skills/test-driven-development/SKILL.md`
  - 🎯 Goal: Review and verify all completed work meets acceptance criteria
  - 📂 Files: All modified files
  - 🧭 Context: Final quality check before marking sprint complete
  - ✅ Acceptance: All tasks have passing tests and meet skill requirements
