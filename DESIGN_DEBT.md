# Design Debt

> Last Updated: 2026-02-24T00:00:00Z
> Total Open: 5

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

### DD-015: Missing padding on ColorLegend outermost container

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6 imports (highest usage)
- **Priority:** Medium (violates Minimum Padding requirement from iced-rs skill)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Minimum Padding: Window≥32px, Section≥24px, Element≥8px, Button: 12px vertical, 24px horizontal"
- **Evidence:**
```rust
container(content).into()
```
- **Line(s):** 170
- **Expected:** The outermost container should have `.padding(MD)` or similar (≥8px for Element level)
- **Suggested fix:** Add `.padding(8)` or use the spacing module constant to the container
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24
- **Status:** OPEN

---

### DD-016: Missing outermost container/padding on ElevationTiltSelector

- **Component:** `tempest-app/src/elevation_tilt_selector.rs`
- **Usage count:** 5 imports (second highest)
- **Priority:** Medium (violates Minimum Padding requirement from iced-rs skill)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Minimum Padding: Window≥32px, Section≥24px, Element≥8px"
- **Evidence:**
```rust
content.into()
```
- **Line(s):** 178
- **Expected:** Content should be wrapped in a container with padding (≥8px for Element level)
- **Suggested fix:** Wrap content in `container(content).padding(MD)` or similar
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24
- **Status:** OPEN

---
