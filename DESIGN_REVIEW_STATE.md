# Design Review State

> Last Updated: 2026-02-24T20:05:00Z
> Total runs completed: 24

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 3 | 2026-02-24 | none |
| TimelineState | tempest-app/src/timeline.rs | 1 | 7 | 2026-02-24 | none |
| StationSelector | tempest-app/src/station_selector.rs | 1 | 10 | 2026-02-24 | DD-037, DD-038, DD-041, DD-042 |
| ColorLegend | tempest-app/src/color_legend.rs | 6 | 6 | 2026-02-24 | DD-043 |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 5 | 6 | 2026-02-24 | none |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 1 | 7 | 2026-02-24 | DD-039 |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 1 | 5 | 2026-02-24 | none |
| CacheManager | tempest-app/src/cache_manager.rs | 1 | 8 | 2026-02-24 | none |
| Config | tempest-app/src/config.rs | 0 | 1 | 2026-02-24 | none |
| DecodeTypes | tempest-decode/src/types.rs | High | 1 | 2026-02-24 | none |
| FetchCache | tempest-fetch/src/cache.rs | High | 1 | 2026-02-24 | none |
| OfflineDetection | tempest-app/src/offline_detection.rs | 1 | 1 | 2026-02-24 | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives |
| --- | ---- | ------------------- | -------------- | ---------------- |
| 24 | 2026-02-24 | StationSelector (verified debt) | 0 | 0 |
| 23 | 2026-02-24 | StationSelector, ColorLegend | DD-041, DD-042, DD-043 | 1 |
| 22 | 2026-02-24 | CacheManager, ColorLegend | DD-040 | 0 |
| 21 | 2026-02-24 | ElevationTiltSelector, OfflineIndicator, StationSelector, MomentSwitcher | 0 | 0 |
| 20 | 2026-02-24 | StationSelector, MomentSwitcher | DD-037, DD-038, DD-039 | 0 |
| 19 | 2026-02-24 | CacheManager, ColorLegend, OfflineDetection | DD-036 | 0 |

## Violation Counts (for worst-violations selection)

| Component | Total Violations | Unresolved |
| --------- | ---------------- | ---------- |
| StationSelector | 5 | 4 |
| MomentSwitcher | 3 | 1 |
| CacheManager | 5 | 0 |
| ColorLegend | 4 | 1 |
