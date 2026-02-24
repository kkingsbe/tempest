# Design Debt

> Last Updated: 2026-02-24T06:06:00Z
> Total Open: 3

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
- **Status:** OPEN

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
- **Status:** OPEN

---
