# Design Review State

> Last Updated: 2026-02-25T13:08:00Z
> Total runs completed: 39

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 4 | 2026-02-25 | DD-059 |
| TimelineState | tempest-app/src/timeline.rs | 30 | 13 | 2026-02-25 | DD-046, DD-060, DD-064 |
| StationSelector | tempest-app/src/station_selector.rs | 26 | 15 | 2026-02-25 | DD-038 |
| ColorLegend | tempest-app/src/color_legend.rs | 19 | 10 | 2026-02-25 | none |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 26 | 12 | 2026-02-25 | DD-050, DD-062 |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 27 | 9 | 2026-02-25 | DD-042, DD-056, DD-061 |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 19 | 7 | 2026-02-25 | none |
| CacheManager | tempest-app/src/cache_manager.rs | 57 | 13 | 2026-02-25 | DD-055, DD-057, DD-058, DD-063 |
| Config | tempest-app/src/config.rs | 0 | 1 | 2026-02-24 | none |
| Colors | tempest-app/src/colors.rs | 3 | 1 | 2026-02-25 | none |
| Spacing | tempest-app/src/spacing.rs | 4 | 1 | 2026-02-25 | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives |
| --- | ---- | ------------------- | -------------- | ---------------- |
| 39 | 2026-02-25 | StationSelector, Colors, Spacing | DD-037 resolved | 0 |
| 38 | 2026-02-25 | StationSelector, ColorLegend, ElevationTiltSelector, Timeline | 0 | 0 |
| 37 | 2026-02-25 | Timeline, CacheManager | DD-064 | 0 |
| 36 | 2026-02-25 | ElevationTiltSelector, CacheManager | DD-048, DD-049 resolved | 0 |
| 34 | 2026-02-25 | ColorLegend, MomentSwitcher, ElevationTiltSelector, Timeline | DD-061, DD-062 | 0 |
| 33 | 2026-02-25 | OfflineIndicator, ElevationTiltSelector | 0 | 0 |
| 32 | 2026-02-25 | StationSelector, CacheManager | 0 | 0 |
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
| Timeline | 5 | 3 |
| StationSelector | 6 | 1 |
| ElevationTiltSelector | 4 | 2 |
| MomentSwitcher | 4 | 3 |
| CacheManager | 9 | 4 |
