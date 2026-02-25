# TODO3.md - Agent 3 Work Queue (Sprint 23)

> ⚠️ Rebalanced by Architect on 2026-02-25

## Status: PENDING START

### Design Debt Items

- [ ] [DD-044] Fix Timeline - Button Padding Below Minimum (CRITICAL)
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change button padding from SM (8px) to at least 12px vertical, 24px horizontal
  - 📂 Files: `tempest-app/src/timeline.rs`
  - 🧭 Context: Button padding should be at least 12px vertical, 24px horizontal. Lines 312, 317, 322, 333, 345, 380 need fixes.
  - ✅ Acceptance: All timeline buttons have padding of at least [12, 24]

- [ ] [DD-045] Fix Timeline - Section Spacing Below Minimum (HIGH)
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.spacing(spacing::SM)` to `.spacing(spacing::LG)` at line 284
  - 📂 Files: `tempest-app/src/timeline.rs`
  - 🧭 Context: Section spacing should be at least LG (24px) between distinct sections
  - ✅ Acceptance: Main column spacing between header, timeline bar, controls row, footer is LG (24px) or larger

- [ ] [DD-046] Fix Timeline - Inter-element Spacing Below Minimum (MEDIUM)
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.spacing(spacing::XXS)` to `.spacing(spacing::XS)` or `.spacing(spacing::SM)` at lines 415, 458
  - 📂 Files: `tempest-app/src/timeline.rs`
  - 🧭 Context: Spacing between interactive elements should be at least XS (4px), typically SM (8px). XXS should only be used for icon-to-label gaps.
  - ✅ Acceptance: Timeline tick marks and labels use XS or SM spacing

### AGENT QA
- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run cargo test. If ALL errors fixed and tests pass, create '.agent_done_3' with the current date.
