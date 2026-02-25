# TODO2.md - Agent 2 Work Queue (Sprint 23)

> ⚠️ Rebalanced by Architect on 2026-02-25

## Status: PENDING START

### Design Debt Items

- [ ] [DD-041] Fix StationSelector - Column Spacing Below Minimum
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.spacing(4)` to `.spacing(spacing::SM)` or `.spacing(spacing::MD)` at line 124
  - 📂 Files: `tempest-app/src/station_selector.rs`
  - 🧭 Context: Element spacing within a group should be at least SM (8px). The skill says "Element spacing within a group: At least SM (8px), typically MD (12px)."
  - ✅ Acceptance: No spacing(4) in Column/Row definitions; uses spacing::SM or larger

- [ ] [DD-042] Fix StationSelector - Visual Proximity Rule Violated
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Increase between-group spacing to BASE (16px) or LG (24px) - currently both within-group and between-group are 4px
  - 📂 Files: `tempest-app/src/station_selector.rs`
  - 🧭 Context: "Space BETWEEN groups must always be LARGER than space WITHIN groups." Lines 124, 161, 200 need adjustment.
  - ✅ Acceptance: Between-group spacing > within-group spacing; uses LG (24px) or larger for sections

- [ ] [DD-056] Fix StationSelector - Form Field Padding Below Minimum
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.padding(8)` to `.padding(12)` or `.padding(spacing::MD)` at line 121
  - 📂 Files: `tempest-app/src/station_selector.rs`
  - 🧭 Context: Form field padding should be at least 12px vertical for comfortable touch targets
  - ✅ Acceptance: Input field padding is MD (12px) or larger

### AGENT QA
- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run cargo test. If ALL errors fixed and tests pass, create '.agent_done_2' with the current date.
