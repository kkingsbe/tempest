# Design Debt

> Last Updated: 2026-02-23T23:05:00Z
> Total Open: 6

---

### DD-001: Deprecated Sandbox API

- **Component:** `tempest-app/src/main.rs`
- **Usage count:** Application entry point - 1 use
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "The older trait-based `Sandbox`/`Application` API (≤0.12) is deprecated — do NOT use it." (line 7)
- **Evidence:**
```rust
impl Sandbox for App {
    type Message = Message;
```
- **Line(s):** 153
- **Expected:** Use the modern function-based API (iced 0.13+) with `fn run()` and separate `update`/`view` functions
- **Suggested fix:** Migrate from `Sandbox` trait to the function-based `run()` API with explicit `update` and `view` functions
- **Fix estimate:** L (45+ min)
- **Queued:** 2026-02-23
- **Status:** SCHEDULED for Sprint 15

---

### DD-002: Arbitrary Spacing Values

- **Component:** `tempest-app/src/timeline.rs`
- **Usage count:** 1 import in main.rs
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "ALL spacing values in your application MUST come from this scale" (8-point scale, lines 21-31)
- **Evidence:**
```rust
        .spacing(12)
        .align_items(iced::Alignment::Start),
]
.spacing(8)
.align_items(iced::Alignment::Center);
```
- **Line(s):** 262, 271, 276, 282
- **Expected:** Use spacing tokens: XS=4, SM=8, MD=12, BASE=16, LG=24. Values like 12 are borderline, but inconsistent mixing of values like 4, 5, 8, 12, 15, 20 violates the principle
- **Suggested fix:** Define a `mod spacing` with tokens and use throughout: `.spacing(spacing::SM)` instead of `.spacing(8)`
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23
- **Status:** SCHEDULED for Sprint 15

---

### DD-004: Raw RGB Color Values in ColorLegend

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6 (from grep analysis)
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Color system: "Semantic naming (accent, danger, success) — never raw hex values"
- **Evidence:**
```rust
let heading_style = iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0));
let label_style = iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7));
let dark_bg = iced::Background::Color(iced::Color::from_rgb(0.1, 0.1, 0.15));
// ...
color: iced::Color::from_rgb(0.15, 0.15, 0.2),
```
- **Line(s):** 113, 114, 115, 186
- **Expected:** Use semantic color constants like `colors::ACCENT`, `colors::TEXT_SECONDARY`, `colors::BG_PRIMARY`
- **Suggested fix:** Create a colors module with semantic color constants and replace all raw RGB values
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23T19:56:00Z
- **Status:** SCHEDULED for Sprint 15

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
- **Status:** OPEN

---

### DD-013: Raw RGB colors in CacheManager

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** 1 import
- **Priority:** Medium (newly reviewed component, low usage but first review)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Color System: Semantic naming, use theme extended_palette(), never raw hex scattered"
- **Evidence:**
```rust
iced::Color::from_rgb(0.2, 0.6, 1.0)   // Line 236 - blue accent
iced::Color::from_rgb(0.7, 0.7, 0.7)   // Line 237 - gray
iced::Color::from_rgb(0.9, 0.9, 0.9)   // Line 238 - light gray
iced::Color::from_rgb(1.0, 0.4, 0.4)   // Line 239 - red warning
iced::Color::from_rgb(0.4, 1.0, 0.4)   // Line 240 - green success
```
- **Line(s):** 236, 237, 238, 239, 240
- **Expected:** Use theme extended_palette() with semantic color names
- **Suggested fix:** Define semantic color constants using theme palette
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23T22:00:00Z
- **Status:** OPEN

---

### DD-014: Raw RGB colors in TimelineState

- **Component:** `tempest-app/src/timeline.rs`
- **Usage count:** 1 import
- **Priority:** Medium (has existing open debt DD-002)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Color System: Semantic naming, use theme extended_palette(), never raw hex scattered"
- **Evidence:**
```rust
iced::Color::from_rgb(0.2, 0.6, 1.0)   // Line 215 - blue accent
iced::Color::from_rgb(0.12, 0.12, 0.18)// Line 216 - dark background
iced::Color::from_rgb(0.7, 0.7, 0.7)   // Line 217 - gray
iced::Color::from_rgb(0.4, 0.7, 1.0)   // Line 433 - lighter blue
iced::Color::from_rgb(0.3, 0.3, 0.4)   // Line 435 - dark gray
```
- **Line(s):** 215, 216, 217, 433, 435
- **Expected:** Use theme extended_palette() with semantic color names
- **Suggested fix:** Define semantic color constants using theme palette
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23T22:00:00Z
- **Status:** OPEN

---
