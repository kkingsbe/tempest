# TODO4.md - Agent 4 Work Queue (Sprint 23)

> ⚠️ Rebalanced by Architect on 2026-02-25

## Status: PENDING START

### Design Debt Items

- [ ] [DD-043] Fix ColorLegend - Container Padding Below Minimum (HIGH)
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.padding(spacing::SM)` to `.padding(spacing::BASE)` or `.padding(spacing::LG)` at line 173
  - 📂 Files: `tempest-app/src/color_legend.rs`
  - 🧭 Context: Container internal padding should be at least BASE (16px). The skill says "Container internal padding: At least BASE (16px). Cards and panels use LG (24px) or XL (32px)"
  - ✅ Acceptance: Container padding is BASE (16px) or larger

- [ ] [DD-051] Fix ColorLegend - Section Spacing Below Minimum (MEDIUM)
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.spacing(spacing::SM)` to `.spacing(spacing::LG)` at line 170
  - 📂 Files: `tempest-app/src/color_legend.rs`
  - 🧭 Context: Section spacing should be at least LG (24px) between distinct sections
  - ✅ Acceptance: Section spacing between title, color bar row, and min label is LG (24px) or larger

- [ ] [DD-053] Fix OfflineIndicator - Container Padding Below Minimum (HIGH)
  - 📚 SKILLS: ./skills/iced-rs/SKILL.md
  - 🎯 Goal: Change `.padding(8)` to `.padding(spacing::BASE)` and `.padding(12)` to `.padding(spacing::BASE)` or `.padding(spacing::LG)` at lines 85, 94
  - 📂 Files: `tempest-app/src/offline_indicator.rs`
  - 🧭 Context: Container internal padding should be at least BASE (16px)
  - ✅ Acceptance: All container padding is BASE (16px) or larger

### AGENT QA
- [ ] AGENT QA: Run cargo build FIRST to verify compilation. Fix ALL build errors. Then run cargo test. If ALL errors fixed and tests pass, create '.agent_done_4' with the current date.
