# Sprint 25 - Agent 4 Tasks

**Focus:** timeline.rs, cache_manager.rs

## Design Debt Items

- [ ] [DD-046] Fix Timeline - Inter-element Spacing Below Minimum (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change `.spacing(spacing::XXS)` to `.spacing(spacing::XS)` or `.spacing(spacing::SM)` for spacing between tick marks and tick labels
  - 📂 Files: `tempest-app/src/timeline.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-046 for details
  - ✅ Acceptance: Inter-element spacing uses XS or SM, build passes, tests pass

- [ ] [DD-055] Fix CacheManager - Inter-element Spacing Below Minimum (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change `.padding(12)` to `.padding(spacing::BASE)` or `.padding(spacing::LG)` for settings panel containers
  - 📂 Files: `tempest-app/src/cache_manager.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-055 for details
  - ✅ Acceptance: Container padding is at least 16px, build passes, tests pass

- [ ] [DD-060] Fix Timeline - Container Padding Below Minimum (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Ensure container padding is at least BASE (16px) for timeline containers
  - 📂 Files: `tempest-app/src/timeline.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-060 for details
  - ✅ Acceptance: Container padding is at least 16px, build passes, tests pass

- [ ] [DD-063] Fix CacheManager - Non-8-point Dimensions (Medium)
  - 📚 SKILLS: `./skills/iced-rs.md`, `./skills/rust-best-practices.md`
  - 🎯 Goal: Change Space heights from 12px, 8px, 16px to 24px (LG) and column spacing from 8px to 24px for proper section spacing
  - 📂 Files: `tempest-app/src/cache_manager.rs`
  - 🧭 Context: See DESIGN_DEBT.md DD-063 for details
  - ✅ Acceptance: All spacing uses 8-point values, build passes, tests pass

- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run full test suite. If ALL errors fixed and tests pass, create '.agent_done_4' with the current date. If ALL '.agent_done_*' files exist, also create '.sprint_complete'.
