# Design Debt

> Last Updated: 2026-02-23T21:04:00.000Z
> Total Open: 11

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
- **Status:** OPEN

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
- **Status:** OPEN

---

### DD-003: Raw RGB Color Values

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** 1 import in main.rs
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Never use raw hex/RGB values scattered throughout your view code. Define a semantic color palette and use it everywhere." (lines 151-178)
- **Evidence:**
```rust
let label_style = iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7));
let value_style = iced::theme::Text::Color(iced::Color::from_rgb(0.9, 0.9, 0.9));
let heading_style = iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0));
```
- **Line(s):** 105-107
- **Expected:** Define semantic colors in a `mod colors` (e.g., TEXT_SECONDARY, TEXT_PRIMARY, ACCENT) and use those, or leverage theme's `extended_palette()`
- **Suggested fix:** Create `mod colors` with semantic tokens and use `colors::TEXT_SECONDARY` instead of inline `Color::from_rgb(0.7, 0.7, 0.7)`
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23
- **Status:** OPEN

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
- **Status:** OPEN

---

### DD-005: Non-8-Point Spacing in ColorLegend

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Spacing Scale: "Must use 8-point scale: XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32)"
- **Evidence:**
```rust
.padding(10)
```
- **Line(s):** 178
- **Expected:** Use MD(12) or SM(8) - prefer MD(12) for container padding
- **Suggested fix:** Change `.padding(10)` to `.padding(12)` (MD) or `.padding(8)` (SM)
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-23T19:56:00Z
- **Status:** OPEN

---

### DD-006: Raw RGB Color Values in ElevationTiltSelector

- **Component:** `tempest-app/src/elevation_tilt_selector.rs`
- **Usage count:** 5
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Color system: "Semantic naming (accent, danger, success) — never raw hex values"
- **Evidence:**
```rust
let heading_style = iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0));
let label_style = iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7));
// ...
iced::Color::from_rgb(1.0, 1.0, 1.0),
// ...
iced::Color::from_rgb(0.7, 0.7, 0.8),
```
- **Line(s):** 105, 106, 140, 148
- **Expected:** Use semantic color constants
- **Suggested fix:** Create/use semantic color constants and replace all raw RGB values
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23T19:56:00Z
- **Status:** OPEN

---

### DD-007: Non-8-Point Spacing in ElevationTiltSelector

- **Component:** `tempest-app/src/elevation_tilt_selector.rs`
- **Usage count:** 5
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Spacing Scale: "Must use 8-point scale"
- **Evidence:**
```rust
.spacing(15)  // Line 116
.spacing(15)  // Line 178
```
- **Line(s):** 116, 178
- **Expected:** Use MD(12) or BASE(16) - 15px falls between, use nearest scale value
- **Suggested fix:** Change `.spacing(15)` to `.spacing(12)` (MD) or `.spacing(16)` (BASE)
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-23T19:56:00Z
- **Status:** OPEN

---

### DD-008: Raw RGB Color Values in MomentSwitcher

- **Component:** `tempest-app/src/moment_switcher.rs`
- **Usage count:** 1 import in main.rs
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Color system: "Semantic naming (accent, danger, success) — never raw hex values" (lines 151-178)
- **Evidence:**
```rust
let heading_style = iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0));
let label_style = iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7));
// ... in buttons:
iced::Color::from_rgb(1.0, 1.0, 1.0),
iced::Color::from_rgb(0.7, 0.7, 0.8),
```
- **Line(s):** 142, 143, 156-158, 168-170
- **Expected:** Use semantic color constants like `colors::ACCENT`, `colors::TEXT_SECONDARY`, `colors::TEXT_PRIMARY`
- **Suggested fix:** Create a `mod colors` with semantic tokens and replace all raw RGB values
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-23T21:04:00Z
- **Status:** OPEN

---

### DD-009: Non-8-Point Spacing in MomentSwitcher

- **Component:** `tempest-app/src/moment_switcher.rs`
- **Usage count:** 1 import in main.rs
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Spacing Scale: "Must use 8-point scale: XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32)" (lines 21-31)
- **Evidence:**
```rust
        .spacing(15)
```
- **Line(s):** 197
- **Expected:** Use MD(12) or BASE(16) - 15px falls between, use nearest scale value
- **Suggested fix:** Change `.spacing(15)` to `.spacing(12)` (MD) or `.spacing(16)` (BASE)
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-23T21:04:00Z
- **Status:** OPEN

---

### DD-010: Raw RGB Color Values in OfflineIndicator

- **Component:** `tempest-app/src/offline_indicator.rs`
- **Usage count:** 1 import in main.rs
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Color system: "Semantic naming (accent, danger, success) — never raw hex values" (lines 151-178)
- **Evidence:**
```rust
const ONLINE_COLOR: Color = Color::from_rgb(0.3, 0.9, 0.3);
const OFFLINE_COLOR: Color = Color::from_rgb(0.9, 0.3, 0.3);
```
- **Line(s):** 9, 10
- **Expected:** Use semantic color constants like `colors::SUCCESS`, `colors::DANGER`
- **Suggested fix:** Create a `mod colors` with semantic tokens and use `colors::SUCCESS` instead of raw RGB
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-23T21:04:00Z
- **Status:** OPEN

---

### DD-011: Non-8-Point Padding in OfflineIndicator

- **Component:** `tempest-app/src/offline_indicator.rs`
- **Usage count:** 1 import in main.rs
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — Spacing Scale: "Must use 8-point scale: XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32)" (lines 21-31)
- **Evidence:**
```rust
            .padding(10)
```
- **Line(s):** 88
- **Expected:** Use MD(12) or SM(8) - prefer MD(12) for container padding
- **Suggested fix:** Change `.padding(10)` to `.padding(12)` (MD) or `.padding(8)` (SM)
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-23T21:04:00Z
- **Status:** OPEN

---
