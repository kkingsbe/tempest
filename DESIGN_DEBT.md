# Design Debt

> Last Updated: 2026-02-24T12:16:24Z
> Total Open: 5

---

### DD-030: ElevationTiltSelector - Non-8-point Button Dimensions (Medium Priority)

- **Component:** `tempest-app/src/elevation_tilt_selector.rs`
- **Priority:** Medium
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
// Lines 142-143 - Selected elevation button
.width(Length::Fixed(60.0))
.height(Length::Fixed(40.0))

// Lines 148-149 - Unselected elevation button
.width(Length::Fixed(60.0))
.height(Length::Fixed(40.0))
```
- **Line(s):** 142, 143, 148, 149
- **Expected:** Use 8-point values (e.g., 48x40, 56x48, or 64x48)
- **Suggested fix:** Replace with 8-point dimensions: `.width(Length::Fixed(48.0))` and `.height(Length::Fixed(48.0))` for square buttons, or adjust to `.width(Length::Fixed(64.0))` and `.height(Length::Fixed(48.0))`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T11:06:00Z
- **Status:** OPEN

---

### DD-031: TimelineState - Non-8-point TIMELINE_HEIGHT constant (High Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TIMELINE_HEIGHT: f32 = 56.0;
```
- **Line(s):** 397
- **Expected:** Use 48 or 64 (8-point values)
- **Suggested fix:** Change TIMELINE_HEIGHT from 56.0 to 48.0 or 64.0 to maintain 8-point grid
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T12:16:00Z
- **Status:** OPEN

---

### DD-025: TimelineState - Non-8-point tick container height calculation (High Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
// Line 461 - calculation uses 10.0 which is NOT in the 8-point scale
.height(iced::Length::Fixed(TICK_HEIGHT + LABEL_HEIGHT + 10.0))
// Current: 16 + 16 + 10 = 42 (NOT 8-point compliant)
```
- **Line(s):** 461
- **Expected:** Use 8-point values in calculation (e.g., 16 + 16 + 8 = 40, or adjust constants)
- **Suggested fix:** Change 10.0 to 8.0 for 8-point compliant calculation: `.height(iced::Length::Fixed(TICK_HEIGHT + LABEL_HEIGHT + 8.0))` resulting in 40px total
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T12:16:00Z
- **Status:** OPEN

---

### DD-024: TimelineState - Non-8-point TOTAL_HEIGHT calculation (High Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TIMELINE_HEIGHT: f32 = 56.0;
const TICK_HEIGHT: f32 = 16.0;
const LABEL_HEIGHT: f32 = 16.0;
const TOTAL_HEIGHT: f32 = TIMELINE_HEIGHT + TICK_HEIGHT + LABEL_HEIGHT;
// Results in 88px (which is 8-point compliant: 88/8=11)
// But TIMELINE_HEIGHT=56 is NOT in the 8-point scale
```
- **Line(s):** 400
- **Expected:** Use 8-point values for all constants
- **Suggested fix:** Change TIMELINE_HEIGHT from 56 to 48 or 64 to be 8-point compliant
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T12:16:00Z
- **Status:** OPEN

---

### DD-023: TimelineState - Non-8-point LABEL_HEIGHT constant (High Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const LABEL_HEIGHT: f32 = 16.0;
```
- **Line(s):** 399
- **Expected:** 16 is already 8-point compliant (BASE)
- **Status:** FIXED - LABEL_HEIGHT is now 16.0 (was 18.0)
- **Queued:** 2026-02-24T09:20:00Z
- **Resolved:** 2026-02-24T12:16:00Z

---

### DD-022: TimelineState - Non-8-point TICK_HEIGHT constant (FIXED)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High (affects multiple calculations)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TICK_HEIGHT: f32 = 16.0;
```
- **Line(s):** 398
- **Expected:** Use 16 or 24 (8-point values)
- **Status:** FIXED - TICK_HEIGHT is now 16.0 (was 20.0)
- **Queued:** 2026-02-24T09:20:00Z
- **Resolved:** 2026-02-24T12:16:00Z

---
