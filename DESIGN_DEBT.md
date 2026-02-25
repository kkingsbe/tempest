# Design Debt

> Last Updated: 2026-02-25T05:00:08Z
> Total Open: 15

---

### DD-042: MomentSwitcher - Non-8-point Button Dimensions (Medium Priority)

- **Component:** `tempest-app/src/moment_switcher.rs`
- **Usage count:** Medium (used in main app)
- **Priority:** Medium (design system violation)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: Button dimensions should follow 8-point scale"
- **Evidence:**
```rust
.width(Length::Fixed(110.0))
.height(Length::Fixed(50.0))
```
- **Line(s):** 189-190, 195-196
- **Expected:** Button dimensions should use 8-point values (e.g., 48x48, 56x48, 112x48)
- **Suggested fix:** Change button dimensions to 48x48 or 112x48 to be 8-point compliant
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T16:00:08Z
- **Status:** OPEN

---

### DD-038: StationSelector - Typography Scale Violation (Medium Priority)

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** Medium (used in main app)
- **Priority:** Medium (design system violation)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Typography Scale: Micro is 10-11px"
- **Evidence:**
```rust
text("").size(5),
```
- **Line(s):** 195
- **Expected:** Typography sizes should follow the scale: Micro(10-11), Caption(12-13), Body(14-16), H3(18-20), etc.
- **Suggested fix:** Use a valid size from the typography scale (e.g., 10 for spacing with XXS, or remove the spacer text and use proper spacing)
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T16:00:08Z
- **Status:** OPEN

---

### DD-037: StationSelector - Button Padding Below Minimum (Medium Priority)

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** Medium (used in main app)
- **Priority:** Medium (design system violation)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Button padding: At least 12px vertical, 24px horizontal"
- **Evidence:**
```rust
.on_press(StationSelectorMessage::StationSelected(station.clone()))
.width(Length::Fill)
.padding(8);
```
- **Line(s):** 140
- **Expected:** Button padding should be at least 12px vertical, 24px horizontal
- **Suggested fix:** Change `.padding(8)` to `.padding([12, 24])` or use a larger padding value
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T16:00:08Z
- **Status:** OPEN

---

### DD-046: Timeline - Inter-element Spacing Below Minimum (Medium Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Usage count:** High (timeline tick marks)
- **Priority:** Medium (violates proximity rule)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "XXS (2px) is for icon-to-label gaps inside compact elements"
- **Evidence:**
```rust
// Line 415: Between tick marks
row![].spacing(spacing::XXS)  // XXS = 2px

// Line 458: Between tick and label
.spacing(spacing::XXS)  // XXS = 2px
```
- **Line(s):** 415, 458
- **Expected:** Spacing between interactive elements should be at least XS (4px), typically SM (8px). XXS should only be used for icon-to-label gaps.
- **Suggested fix:** Change `.spacing(spacing::XXS)` to `.spacing(spacing::XS)` or `.spacing(spacing::SM)`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T21:11:00Z
- **Status:** OPEN

---

### DD-048: ElevationTiltSelector - Container Padding Below Minimum

- **Component:** `tempest-app/src/elevation_tilt_selector.rs` (Line 175)
- **Usage count:** 5 (second most-used component)
- **Priority:** Medium
- **Skill violated:** `skills/iced-rs/SKILL.md` — "Container internal padding: At least BASE (16px)"
- **Evidence:**
```rust
container(content).padding(12).into()
```
- **Line(s):** 175
- **Expected:** Container padding ≥ BASE (16px)
- **Suggested fix:** Change `.padding(12)` to `.padding(16)` or use `spacing::BASE`
- **Fix estimate:** S
- **Queued:** 2026-02-24T21:00:00Z
- **Status:** OPEN

---

### DD-049: ElevationTiltSelector - Button Padding Missing

- **Component:** `tempest-app/src/elevation_tilt_selector.rs` (Lines 136-148)
- **Usage count:** 5
- **Priority:** High
- **Skill violated:** `skills/iced-rs/SKILL.md` — "Button padding: At least 12px vertical, 24px horizontal"
- **Evidence:**
```rust
button(text(label).size(14))
    .on_press(...)
    .width(Length::Fixed(48.0))
    .height(Length::Fixed(48.0))
    .style(iced::widget::button::primary)
```
- **Line(s):** 136-148
- **Expected:** Button padding ≥12px vertical, ≥24px horizontal
- **Suggested fix:** Add `.padding([12, 16])` to provide adequate breathing room
- **Fix estimate:** S
- **Queued:** 2026-02-24T21:00:00Z
- **Status:** OPEN

---

### DD-050: ElevationTiltSelector - Hardcoded Spacing Values

- **Component:** `tempest-app/src/elevation_tilt_selector.rs` (Lines 115, 122, 172)
- **Usage count:** 5
- **Priority:** Low
- **Skill violated:** `skills/iced-rs/SKILL.md` — Use consistent spacing constants from the 8-point scale
- **Evidence:**
```rust
// Line 115: .spacing(12)
// Line 122: let mut elevation_buttons = row!().spacing(8);
// Line 172: .spacing(12)
// Line 175: .padding(12)
```
- **Line(s):** 115, 122, 172, 175
- **Expected:** Use spacing module constants (spacing::MD, spacing::SM, etc.)
- **Suggested fix:** Add `use crate::spacing;` and replace hardcoded values with constants
- **Fix estimate:** S
- **Queued:** 2026-02-24T21:00:00Z
- **Status:** OPEN

---

### DD-051: ColorLegend - Section Spacing Below Minimum (Medium Priority)

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6+ usages
- **Priority:** Medium (violates mandatory section spacing rule)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Section spacing: At least LG (24px) between distinct sections"
- **Evidence:**
```rust
// Line 170: Between title, color bar row, and min label
.spacing(spacing::SM)  // SM = 8px
```
- **Line(s):** 170
- **Expected:** Section spacing should be at least LG (24px) between distinct sections of a view
- **Suggested fix:** Change `.spacing(spacing::SM)` to `.spacing(spacing::LG)` or larger
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T22:07:00Z
- **Status:** OPEN

---

### DD-055: CacheManager - Container Padding Below Minimum (Medium Priority)

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** 1 (used in main app)
- **Priority:** Medium (violates mandatory minimum)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Container internal padding: At least BASE (16px)"
- **Evidence:**
```rust
// Line 292: Settings panel container
]
.spacing(8)
.padding(12);  // 12px < 16px BASE

// Line 294: Another container
container(settings_content).padding(12).into()  // 12px < 16px BASE
```
- **Line(s):** 292, 294
- **Expected:** Container internal padding should be at least BASE (16px)
- **Suggested fix:** Change `.padding(12)` to `.padding(spacing::BASE)` or `.padding(spacing::LG)`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T23:05:00Z
- **Status:** OPEN

---

### DD-052: ColorLegend - Within-Group Spacing Below Minimum (Medium Priority)

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6+ usages
- **Priority:** Medium (violates minimum element spacing rule)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Element spacing within a group: At least SM (8px), typically MD (12px)"
- **Evidence:**
```rust
// Line 167: Between color bar and max label within the row
.spacing(spacing::XS)  // XS = 4px
```
- **Line(s):** 167
- **Expected:** Element spacing within a group should be at least SM (8px), typically MD (12px)
- **Suggested fix:** Change `.spacing(spacing::XS)` to `.spacing(spacing::SM)` or `.spacing(spacing::MD)`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T22:07:00Z
- **Status:** OPEN

---

### DD-056: MomentSwitcher - Missing Button Padding

- **Component:** `tempest-app/src/moment_switcher.rs`
- **Usage count:** 5 imports
- **Priority:** High
- **Skill violated:** `skills/iced-rs/SKILL.md` - "Button padding: At least 12px vertical, 24px horizontal"
- **Evidence:**
```rust
button(text(format!("{}\n{}", moment.code(), moment.name())).size(12))
    .on_press(MomentSwitcherMessage::MomentSelected(moment))
    .width(Length::Fixed(110.0))
    .height(Length::Fixed(50.0))
    .style(primary_button_style)
```
- **Line(s):** 187-198 (both primary and secondary buttons lack .padding() call)
- **Expected:** Add .padding(12) or higher for vertical, .padding(24) or higher for horizontal
- **Suggested fix:** Add .padding(12) to both button definitions
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-25T02:00:30Z
- **Status:** OPEN

---

### DD-057: CacheManager - Text Input Padding Below Minimum

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** 4 imports
- **Priority:** Medium
- **Skill violated:** `skills/iced-rs/SKILL.md` - "Form field spacing: At least 12px"
- **Evidence:**
```rust
let max_size_input = text_input("Max size (MB)", &self.max_size_input)
    .on_input(CacheManagerMessage::MaxSizeChanged)
    .width(Length::Fixed(150.0))
    .padding(10);  // 10px < 12px minimum
```
- **Line(s):** 275
- **Expected:** Change .padding(10) to .padding(12) or higher
- **Suggested fix:** Update padding to 12px minimum
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-25T02:00:30Z
- **Status:** OPEN

---

### DD-058: CacheManager - Non-8-Point Button Width

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** 4 imports
- **Priority:** Low
- **Skill violated:** `skills/iced-rs/SKILL.md` - "8-Point Spacing: All spacing values MUST come from this scale"
- **Evidence:**
```rust
button(text("Clear Cache"))
    .on_press(CacheManagerMessage::ClearCache)
    .width(Length::Fixed(150.0))  // 150 not in 8-point scale (valid: 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160)
    .padding([12, 24])
```
- **Line(s):** 251, 256
- **Expected:** Use 8-point grid values (e.g., 144px or 152px instead of 150px)
- **Suggested fix:** Round width to nearest 8-point value
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-25T02:00:30Z
- **Status:** OPEN

---

### DD-059: Main Layout - Missing Component Spacing (High Priority)

- **Component:** `tempest-app/src/main.rs`
- **Usage count:** 1 (root app component)
- **Priority:** High (root layout violation affects entire app)
- **Skill violated:** `skills/iced-rs/SKILL.md` — "Element spacing within a group: At least SM (8px), typically MD (12px)"
- **Evidence:**
```rust
// Lines 165-192: Main view column has no spacing between components
column![
    state.station_selector.view().map(Message::StationSelector),
    state.moment_switcher.view().map(Message::MomentSwitcher),
    state.elevation_tilt_selector.view(&Theme::Dark).map(Message::ElevationTiltSelector),
    state.color_legend.view().map(Message::ColorLegend),
    state.timeline.view().map(Message::Timeline),
    state.offline_indicator.view().map(Message::OfflineIndicator),
    state.cache_manager.view().map(Message::CacheManager),
    text(format!...))
]
.into()  // No .spacing() call - defaults to 0px
```
- **Line(s):** 165-192
- **Expected:** Add .spacing(spacing::LG) or .spacing(spacing::BASE) to separate distinct sections
- **Suggested fix:** Add .spacing(spacing::LG) after the column![] definition to provide proper section spacing
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-25T05:00:08Z
- **Status:** OPEN

---

### DD-060: Timeline - Typography Scale Violation for Timestamps (Medium Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Usage count:** 1 (timeline component)
- **Priority:** Medium
- **Skill violated:** `skills/iced-rs/SKILL.md` — "Typography Scale: Caption (12-13px) for timestamps"
- **Evidence:**
```rust
// Line 454: Time labels use size(10) which is Micro
let label = text(label_text).size(10).font(iced::font::Font::MONOSPACE);
```
- **Line(s):** 454
- **Expected:** Typography sizes should follow the scale: Caption (12-13px) for timestamps, Micro (10-11px) for badges/status labels
- **Suggested fix:** Change .size(10) to .size(12) or .size(13) for timestamp labels
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-25T05:00:08Z
- **Status:** OPEN

