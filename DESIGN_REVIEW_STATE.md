# Design Review State

> Last Updated: 2026-02-25T05:00:08Z
> Total runs completed: 31

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 4 | 2026-02-25 | DD-059 |
| TimelineState | tempest-app/src/timeline.rs | 1 | 10 | 2026-02-25 | DD-044, DD-046, DD-060 |
| StationSelector | tempest-app/src/station_selector.rs | 6+ | 11 | 2026-02-25 | DD-037, DD-038 |
| ColorLegend | tempest-app/src/color_legend.rs | 6+ | 8 | 2026-02-25 | DD-051, DD-052 |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 5+ | 8 | 2026-02-25 | DD-048, DD-049, DD-050 |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 3+ | 8 | 2026-02-25 | DD-042, DD-056 |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 2+ | 6 | 2026-02-24 | none (scheduled) |
| CacheManager | tempest-app/src/cache_manager.rs | 3+ | 10 | 2026-02-25 | DD-055, DD-057, DD-058 |
| Config | tempest-app/src/config.rs | 0 | 1 | 2026-02-24 | none |
| DecodeTypes | tempest-decode/src/types.ts | High | 1 | 2026-02-24 | none |
| FetchCache | tempest-fetch/src/cache.rs | High | 1 | 2026-02-24 | none |
| OfflineDetection | tempest-app/src/offline_detection.rs | 1 | 2 | 2026-02-25 | none |
| Colors | tempest-app/src/colors.rs | 3 | 1 | 2026-02-25 | none |
| Spacing | tempest-app/src/spacing.rs | 4 | 1 | 2026-02-25 | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives |
| --- | ---- | ------------------- | -------------- | ---------------- |
| 31 | 2026-02-25 | App (main.rs), Timeline | DD-059, DD-060 | 0 |
| 30 | 2026-02-25 | CacheManager, MomentSwitcher, Colors, Spacing | DD-056, DD-057, DD-058 | 1 |
| 29 | 2026-02-25 | ColorLegend, ElevationTiltSelector, OfflineIndicator, MomentSwitcher, OfflineDetection | 0 | 0 |
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
| App | 1 | 1 |
| Timeline | 4 | 3 |
| StationSelector | 6 | 2 |
| ElevationTiltSelector | 6 | 3 |
| ColorLegend | 4 | 2 |
| MomentSwitcher | 4 | 2 |
| CacheManager | 8 | 3 |

