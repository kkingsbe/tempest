# Design Debt

> Last Updated: 2026-02-24T15:09:00Z
> Total Open: 7

---

### DD-036: PeriodicConnectivityChecker - Use of `expect()` in Production Code (Medium Priority)

- **Component:** `tempest-app/src/offline_detection.rs`
- **Usage count:** Low (backend utility)
- **Priority:** Medium (error handling violation)
- **Skill violated:** `./skills/rust-best-practices/SKILL.md` — "Error Handling: Never `unwrap()`/`expect()` outside tests"
- **Evidence:**
```rust
TcpStream::connect_timeout(
    &address.parse().expect("Invalid IP address"),
    Duration::from_secs(TIMEOUT_SECS),
)
.is_ok()
```
- **Line(s):** 48
- **Expected:** Should use proper error handling instead of expect() in production code
- **Suggested fix:** Use `ok()` on the parse result and handle the Option, or use `map_err` to convert to a more specific error type
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T15:09:00Z
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

---

### DD-031: TimelineState - Non-8-point TIMELINE_HEIGHT constant (High Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TIMELINE_HEIGHT: f32 = 56.0;
```
- **Line(s):** 397
- **Expected:** Use 48 or 64 (8-point values)
- **Suggested fix:** Change TIMELINE_HEIGHT from 56.0 to 48.0 or 64.0 to maintain 8-point grid
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T12:16:00Z
- **Status:** RESOLVED

---

### DD-024: TimelineState - Non-8-point TOTAL_HEIGHT calculation (High Priority)

- **Component:** `tempest-app/src/timeline.rs`
- **Priority:** High
- **Skill violated:** `./skills/iced-rs/SKILL.md` — "8-Point Spacing: MUST use XXS(2), XS(4), SM(8), MD(12), BASE(16), LG(24), XL(32), XXL(48), XXXL(64)"
- **Evidence:**
```rust
const TIMELINE_HEIGHT: f32 = 56.0;
const TICK_HEIGHT: f32 = 16.0;
const LABEL_HEIGHT: f32 = 16.0;
const TOTAL_HEIGHT: f32 = TIMELINE_HEIGHT + TICK_HEIGHT + LABEL_HEIGHT;
// Results in 88px (which is 8-point compliant: 88/8=11)
// But TIMELINE_HEIGHT=56 is NOT in the 8-point scale
```
- **Line(s):** 400
- **Expected:** Use 8-point values for all constants
- **Suggested fix:** Change TIMELINE_HEIGHT from 56 to 48 or 64 to be 8-point compliant
- **Fix estimate:** S (< 15 min)
- **Queued:** 2026-02-24T12:16:00Z
- **Status:** RESOLVED

---
