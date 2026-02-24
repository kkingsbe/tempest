# Design Debt

> Last Updated: 2026-02-24T18:00:06Z
> Total Open: 4

---

### DD-039: MomentSwitcher - Non-8-point Button Dimensions (Medium Priority)

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

### DD-040: CacheManager - Non-8-point Padding in Text Input (Low Priority)

- **Component:** `tempest-app/src/cache_manager.rs`
- **Usage count:** Medium (used in main app)
- **Priority:** Low (design system violation)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: ALL spacing values must come from the 8-point scale"
- **Evidence:**
```rust
let max_size_input = text_input("Max size (MB)", &self.max_size_input)
    .on_input(CacheManagerMessage::MaxSizeChanged)
    .width(Length::Fixed(150.0))
    .padding(10);
```
- **Line(s):** 275
- **Expected:** Padding should use 8-point values (2, 4, 8, 12, 16, 24, 32, 48, 64)
- **Suggested fix:** Change `.padding(10)` to `.padding(8)` or `.padding(12)`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T18:00:06Z
- **Status:** OPEN

