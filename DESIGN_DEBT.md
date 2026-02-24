# Design Debt

> Last Updated: 2026-02-24T05:55:00Z
> Total Open: 2

---

### DD-012: Non-8-point spacing in CacheManager

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** 1 import
- **Priority:** Medium (newly reviewed component, low usage but first review)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
.padding(10)   // Line 259
.padding(10)   // Line 264
.padding(5)    // Line 275
.spacing(10)   // Line 297
.padding(10)   // Line 309
.spacing(10)   // Lines 325, 331, 343
.size(15)      // Lines 345, 350
.spacing(5)    // Line 355
.padding(20)   // Line 357
```
- **Line(s):** 259, 264, 275, 297, 309, 325, 331, 343, 345, 350, 355, 357
- **Expected:** Use only [2, 4, 8, 12, 16, 24, 32, 48, 64] for spacing/padding
- **Suggested fix:** Replace all non-8-point values with nearest 8-point equivalent
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23T22:00:00Z
- **Status:** RESOLVED (Sprint 16)

---

### DD-017: Non-8-point spacing in StationSelector

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** 1 import
- **Priority:** Medium (component reviewed multiple times, spacing violations)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
container(details_column).padding(15).into()  // Line 167
...
.spacing(5)    // Line 207
.padding(20)   // Line 209
```
- **Line(s):** 167, 207, 209
- **Expected:** Use only [2, 4, 8, 12, 16, 24, 32, 48, 64] for spacing/padding
- **Suggested fix:** Replace 15→16(MD), 5→4(XS), 20→16(BASE) or 24(LG)
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T04:07:00Z
- **Status:** RESOLVED (Sprint 16)

---

### DD-018: Non-8-point spacing (zero values) in TimelineState

- **Component:** `tempest-app/src/timeline.rs`
- **Usage count:** 1 import
- **Priority:** Medium (component has existing debt, newly found violations)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
let mut ticks_content = row![].spacing(0).align_y(iced::Alignment::End);  // Line 415
```
```rust
let tick_button = button(tick_with_label)
    .on_press(TimelineMessage::TimelineClicked(tick_position))
    .padding(0);  // Line 475
```
- **Line(s):** 415, 475
- **Expected:** Use only [2, 4, 8, 12, 16, 24, 32, 48, 64] for spacing/padding (0 is not in the 8-point scale)
- **Suggested fix:** Replace spacing(0) with spacing(XXS) or spacing(XS), replace padding(0) with padding(XS) or remove padding
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T05:06:00Z
- **Status:** OPEN

---
