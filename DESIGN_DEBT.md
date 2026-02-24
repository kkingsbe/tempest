# Design Debt

> Last Updated: 2026-02-24T20:05:00Z
> Total Open: 6

---

### DD-041: StationSelector - Column Spacing Below Minimum (High Priority)

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** 6+ usages
- **Priority:** High (violates foundational spacing rule)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Spacing Scale: ALL spacing values must use 8-point grid tokens"
- **Evidence:**
```rust
let mut station_buttons = Column::new().spacing(4);
```
- **Line(s):** 124
- **Expected:** Element spacing within a group should be at least SM (8px). The skill says "Element spacing within a group: At least SM (8px), typically MD (12px)."
- **Suggested fix:** Change `.spacing(4)` to `.spacing(spacing::SM)` or `.spacing(spacing::MD)`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T19:00:00Z
- **Status:** OPEN

---

### DD-042: StationSelector - Visual Proximity Rule Violated (High Priority)

- **Component:** `tempest-app/src/station_selector.rs`
- **Usage count:** 6+ usages
- **Priority:** High (violates visual hierarchy rule)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Visual Proximity Rule: Space BETWEEN groups > space WITHIN groups"
- **Evidence:**
```rust
// Line 124: within station list group
let mut station_buttons = Column::new().spacing(4);
// Line 161: within details column  
.spacing(4)
// Line 200: main column (between groups)
.spacing(spacing::XS)
```
- **Line(s):** 124, 161, 200
- **Expected:** "Space BETWEEN groups must always be LARGER than space WITHIN groups." Currently both are 4px, destroying visual hierarchy.
- **Suggested fix:** Increase between-group spacing to BASE (16px) or LG (24px)
- **Fix estimate:** M (15-45 min)
- **Queued:** 2026-02-24T19:00:00Z
- **Status:** OPEN

---

### DD-043: ColorLegend - Container Padding Below Minimum (High Priority)

- **Component:** `tempest-app/src/color_legend.rs`
- **Usage count:** 6+ usages
- **Priority:** High (violates mandatory minimum)
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "Minimum Padding: Container internal padding at least BASE (16px)"
- **Evidence:**
```rust
container(content).padding(spacing::SM).into()
```
- **Line(s):** 173
- **Expected:** Container internal padding should be at least BASE (16px). The skill says "Container internal padding: At least BASE (16px). Cards and panels use LG (24px) or XL (32px)"
- **Suggested fix:** Change `.padding(spacing::SM)` to `.padding(spacing::BASE)` or `.padding(spacing::LG)`
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T19:00:00Z
- **Status:** OPEN

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


