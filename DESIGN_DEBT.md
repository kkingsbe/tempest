# Design Debt

> Last Updated: 2026-02-24T10:25:16Z
> Total Open: 7

---

### DD-028: CacheManager - Typography Scale Violation (Medium Priority)

- **Component:** `tempest-app/src/cache_manager.rs`
- **Priority:** Medium
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Typography Scale: Micro(10-11) minimum"
- **Evidence:**
```rust
// Line 284 - Spacer in settings content
text("").size(5),

// Line 311 - Spacer in statistics section
text("").size(5),

// Line 333 - Spacer in actions section
text("").size(5),
```
- **Line(s):** 284, 311, 333
- **Expected:** Minimum 10px (Micro range: 10-11px) - replace with `.size(10)` or use `Space::with_height()`
- **Suggested fix:** Replace `.size(5)` with `.size(10)` or use `Space::with_height(spacing::XXS)` for spacers
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T10:00:00Z
- **Status:** OPEN

---

### DD-027: CacheManager - Button Padding Below Minimum (High Priority)

- **Component:** `tempest-app/src/cache_manager.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Button padding: At least 12px vertical, 24px horizontal"
- **Evidence:**
```rust
// Line 252 - "Clearing..." button
.padding(8)

// Line 257 - "Clear Cache" button  
.padding(8)

// Line 268 - settings_toggle button
.padding(4)

// Line 275 - max_size_input
.padding(8)

// Line 280 - apply_button
.padding(8)
```
- **Line(s):** 252, 257, 268, 275, 280
- **Expected:** `.padding([12, 24])` for all buttons (12px vertical, 24px horizontal)
- **Suggested fix:** Replace all `.padding(8)` with `.padding([12, 24])` and `.padding(4)` with `.padding([12, 24])`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T10:00:00Z
- **Status:** OPEN

---

### DD-026: MomentSwitcher - Unused Semantic Colors (Medium Priority)

- **Component:** `tempest-app/src/moment_switcher.rs`
- **Priority:** Medium
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Color System: Semantic naming (BG_PRIMARY, TEXT_SECONDARY, ACCENT, SUCCESS, WARNING, DANGER)"
- **Evidence:**
```rust
// Lines 11-31 - Semantic colors defined but never used
mod colors {
    use iced::Color;
    #[allow(dead_code)]
    pub const ACCENT: Color = Color::from_rgb(0.35, 0.55, 1.0);
    #[allow(dead_code)]
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.93, 0.93, 0.95);
    #[allow(dead_code)]
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.6, 0.6, 0.65);
    #[allow(dead_code)]
    pub const TEXT_MUTED: Color = Color::from_rgb(0.4, 0.4, 0.45);
    // ... more unused colors
}
// Lines 174-192 - Uses built-in button styles instead
let btn = if is_selected {
    button(...).style(iced::widget::button::primary)
} else {
    button(...).style(iced::widget::button::secondary)
};
```
- **Line(s):** 11-31 (defined), 174-192 (used instead)
- **Expected:** Use the defined semantic colors (ACCENT, TEXT_PRIMARY, etc.) instead of built-in button styles
- **Suggested fix:** Replace `iced::widget::button::primary/secondary` with custom style using semantic colors
- **Fix estimate:** M (15-45 min)
- **Queued:** 2026-02-24T10:00:00Z
- **Status:** OPEN

---

### DD-025: Non-8-point size calculation using TICK_HEIGHT

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** Medium
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
container(text("|").size((TICK_HEIGHT + 8.0) as u16))
```
- **Line(s):** 444-445
- **Expected:** Use 8-point value (will be 16+8=24 or 24+8=32 after fix)
- **Suggested fix:** Will auto-fix when TICK_HEIGHT corrected
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T09:20:00Z
- **Status:** OPEN

---

### DD-024: Non-8-point TOTAL_HEIGHT calculation

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High (cascades from above)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TOTAL_HEIGHT: f32 = TIMELINE_HEIGHT + TICK_HEIGHT + LABEL_HEIGHT;
```
- **Line(s):** 400
- **Expected:** Result should be 8-point value (currently 48+20+18=86)
- **Suggested fix:** Will auto-fix when TICK_HEIGHT and LABEL_HEIGHT are corrected
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T09:20:00Z
- **Status:** OPEN

---

### DD-023: Non-8-point LABEL_HEIGHT constant

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High (affects multiple calculations)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const LABEL_HEIGHT: f32 = 18.0;
```
- **Line(s):** 399
- **Expected:** Use 16 (8-point value)
- **Suggested fix:** Change LABEL_HEIGHT from 18.0 to 16.0
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T09:20:00Z
- **Status:** OPEN

---

### DD-022: Non-8-point TICK_HEIGHT constant

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High (affects multiple calculations)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TICK_HEIGHT: f32 = 20.0;
```
- **Line(s):** 398
- **Expected:** Use 16 or 24 (8-point values)
- **Suggested fix:** Change TICK_HEIGHT from 20.0 to 16.0 or 24.0
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T09:20:00Z
- **Status:** OPEN

---
