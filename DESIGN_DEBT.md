# Design Debt

> Last Updated: 2026-02-23T19:34:00.000Z
> Total Open: 3

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
