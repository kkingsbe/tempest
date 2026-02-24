# Design Debt

> Last Updated: 2026-02-24T07:07:00Z
> Total Open: 4

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

### DD-019: Non-8-point spacing (zero) in ColorLegend

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6 imports
- **Priority:** Medium (high-usage component, spacing violation)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
let mut color_bar = column!().spacing(0).width(Length::Fixed(30.0));  // Line 129
```
- **Line(s):** 129
- **Expected:** Use only [2, 4, 8, 12, 16, 24, 32, 48, 64] for spacing (0 is not in the 8-point scale)
- **Suggested fix:** Replace spacing(0) with spacing(XXS=2) or use a small positive value
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T06:06:00Z
- **Status:** IN SPRINT 18 (TODO2)

---

### DD-020: Non-8-point padding values in CacheManager

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** 1 import
- **Priority:** High (multiple violations in component with history)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
button(text("Clear Cache"))
    .on_press(CacheManagerMessage::ClearCache)
    .width(Length::Fixed(150.0))
    .padding(10)   // Line 252
```
```rust
let settings_toggle = button(text(settings_toggle_text))
    .on_press(CacheManagerMessage::ToggleSettings)
    .padding(5);  // Line 268
```
```rust
]
.spacing(8)
.padding(10);  // Line 298
```
```rust
container(settings_content).padding(10).into()  // Line 300
```
```rust
]
.spacing(8)
.align_x(Alignment::Start)
.padding(20)  // Line 342
.width(Length::FillPortion(1));
```
- **Line(s):** 252, 257, 268, 298, 300, 342
- **Expected:** Use only [2, 4, 8, 12, 16, 24, 32, 48, 64] for padding
- **Suggested fix:** Replace padding values: 10→8(SM) or 12(MD), 5→4(XS), 20→16(BASE) or 24(LG)
- **Fix estimate:** M (15–45 min)
- **Queued:** 2026-02-24T06:06:00Z
- **Status:** IN SPRINT 18 (TODO1)

---

### DD-021: Inline RGB colors instead of semantic constants in StationSelector

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** 1 import
- **Priority:** Medium (component has semantic color constants defined but not used in view)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Never use raw hex/RGB values scattered throughout your view code. Define a semantic color palette and use it everywhere."
- **Evidence:**
```rust
// Lines 138-141: Inline RGB colors instead of using colors::ACCENT and colors::TEXT_SECONDARY
let btn = button(text(format!("{} - {}", station.id, station.name)).color(
    if is_selected {
        iced::Color::from_rgb(0.2, 0.6, 1.0)  // Should use colors::ACCENT
    } else {
        iced::Color::from_rgb(0.8, 0.8, 0.8)  // Should use colors::TEXT_SECONDARY
    },
))
```
```rust
// Lines 200-202: Inline RGB color instead of using colors::TEXT_SECONDARY
text(count_text.clone())
    .color(iced::Color::from_rgb(0.5, 0.5, 0.5))  // Should use colors::TEXT_SECONDARY
    .size(12),
```
- **Line(s):** 138-141, 200-202
- **Expected:** Use the semantic color constants already defined in the file (colors::ACCENT, colors::TEXT_SECONDARY)
- **Suggested fix:** Replace inline Color::from_rgb calls with the semantic constants from the colors module
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T07:07:00Z
- **Status:** IN SPRINT 18 (TODO3)

---
