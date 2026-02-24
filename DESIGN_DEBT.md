# Design Debt

> Last Updated: 2026-02-24T16:00:08Z
> Total Open: 7

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
- **Line(s):** 196-197, 205-206
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
- **Line(s):** 197
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

### DD-035: CacheEntry - Use of `expect()` in Production Code (Medium Priority)

- **Component:** `tempest-fetch/src/cache.rs`
- **Usage count:** High
- **Priority:** Medium (error handling violation)
- **Skill violated:** `./skills/rust-best-practices/SKILL.md` — "Error Handling: Never `unwrap()`/`expect()` outside tests"
- **Evidence:**
```rust
let lru = LruCache::new(NonZeroUsize::new(10000).expect("Capacity must be non-zero"));
```
- **Line(s):** 101
- **Expected:** Should use proper error handling instead of expect() in production code
- **Suggested fix:** Use `ok_or_else()` or similar to handle the Result properly
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T14:22:00Z
- **Status:** OPEN

---

### DD-034: CacheEntry - Cloning in Loop (Medium Priority)

- **Component:** `tempest-fetch/src/cache.rs`
- **Usage count:** High (central caching system)
- **Priority:** Medium (performance inefficiency)
- **Skill violated:** `./skills/rust-best-practices/SKILL.md` — "Performance: Avoid cloning in loops"
- **Evidence:**
```rust
let entry = CacheEntry::new(key.to_string(), size);
lru.put(key.to_string(), entry);
```
- **Line(s):** 161-162
- **Expected:** Should call `to_string()` only once and reuse the owned String
- **Suggested fix:** Create the owned string once: `let key_owned = key.to_string();` then use `key_owned` for both calls
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T14:22:00Z
- **Status:** OPEN

---

### DD-033: Observation - Use of `String` Instead of `&str` for Parameters (Medium Priority)

- **Component:** `tempest-decode/src/types.rs`
- **Usage count:** High
- **Priority:** Medium (API design issue, not critical)
- **Skill violated:** `./skills/rust-best-practices/SKILL.md` — "Parameters: Use `&str` over `String`"
- **Evidence:**
```rust
pub fn new(station_id: String, timestamp: DateTime<Utc>, vcp: u16) -> Self {
```
- **Line(s):** 389
- **Expected:** Should accept `&str` instead of `String` for station_id parameter to be more flexible
- **Suggested fix:** Change parameter to `station_id: &str` and convert internally if ownership needed
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T14:22:00Z
- **Status:** OPEN

---

### DD-032: Bytes - Incorrect Naming Convention `as_str_lossy` (Medium Priority)

- **Component:** `tempest-decode/src/types.rs`
- **Usage count:** High (core types used across multiple crates)
- **Priority:** Medium (naming convention violation, not critical)
- **Skill violated:** `./skills/coding-guidelines/SKILL.md` — "Conversion: `as_` (cheap ref), `to_` (expensive), `into_` (ownership)"
- **Evidence:**
```rust
pub fn as_str_lossy(&self) -> String {
    String::from_utf8_lossy(&self.0).into_owned()
}
```
- **Line(s):** 58-60
- **Expected:** Method should be named `to_string_lossy()` since it returns an owned String (the `as_` prefix is for cheap references)
- **Suggested fix:** Rename the method to `to_string_lossy()` to follow Rust naming conventions
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T14:22:00Z
- **Status:** OPEN

