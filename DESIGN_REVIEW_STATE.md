# Design Review State

> Last Updated: 2026-02-25T00:00:00Z
> Total runs completed: 28

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 3 | 2026-02-24 | none |
| TimelineState | tempest-app/src/timeline.rs | 1 | 9 | 2026-02-25 | DD-044, DD-045, DD-046 |
| StationSelector | tempest-app/src/station_selector.rs | 1 | 11 | 2026-02-25 | DD-037, DD-038, DD-041, DD-042, DD-056 |
| ColorLegend | tempest-app/src/color_legend.rs | 6 | 7 | 2026-02-24 | DD-043, DD-051, DD-052 |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 5 | 7 | 2026-02-24 | DD-048, DD-049, DD-050 |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 1 | 7 | 2026-02-24 | DD-039 |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 1 | 6 | 2026-02-24 | DD-053, DD-054 |
| CacheManager | tempest-app/src/cache_manager.rs | 1 | 9 | 2026-02-24 | DD-055 |
| Config | tempest-app/src/config.rs | 0 | 1 | 2026-02-24 | none |
| DecodeTypes | tempest-decode/src/types.rs | High | 1 | 2026-02-24 | none |
| FetchCache | tempest-fetch/src/cache.rs | High | 1 | 2026-02-24 | none |
| OfflineDetection | tempest-app/src/offline_detection.rs | 1 | 1 | 2026-02-24 | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives |
| --- | ---- | ------------------- | -------------- | ---------------- |
| 28 | 2026-02-25 | StationSelector, Timeline | DD-056 | 0 |
| 27 | 2026-02-24 | OfflineIndicator, CacheManager | DD-053, DD-054, DD-055 | 0 |
| 26 | 2026-02-24 | ColorLegend, ElevationTiltSelector | DD-051, DD-052 | 0 |
| 25 | 2026-02-24 | ElevationTiltSelector, Timeline | DD-044, DD-045, DD-046, DD-047, DD-048, DD-049, DD-050 | 0 |
| 23 | 2026-02-24 | StationSelector, ColorLegend | DD-041, DD-042, DD-043 | 1 |
| 22 | 2026-02-24 | CacheManager, ColorLegend | DD-040 | 0 |
| 21 | 2026-02-24 | ElevationTiltSelector, OfflineIndicator, StationSelector, MomentSwitcher | 0 | 0 |
| 20 | 2026-02-24 | StationSelector, MomentSwitcher | DD-037, DD-038, DD-039 | 0 |
| 19 | 2026-02-24 | CacheManager, ColorLegend, OfflineDetection | DD-036 | 0 |

## Violation Counts (for worst-violations selection)

| Component | Total Violations | Unresolved |
| --------- | ---------------- | ---------- |
| Timeline | 3 | 3 |
| StationSelector | 6 | 5 |
| MomentSwitcher | 3 | 1 |
| CacheManager | 6 | 1 |
| ColorLegend | 4 | 3 |
| OfflineIndicator | 3 | 3 |
