# Design Review State

> Last Updated: 2026-02-24T15:09:00Z
> Total runs completed: 19

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 3 | 2026-02-24 | none |
| TimelineState | tempest-app/src/timeline.rs | 1 | 7 | 2026-02-24 | DD-024, DD-031 |
| StationSelector | tempest-app/src/station_selector.rs | 1 | 5 | 2026-02-24 | none |
| ColorLegend | tempest-app/src/color_legend.rs | 6 | 5 | 2026-02-24 | DD-029 |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 5 | 5 | 2026-02-24 | none |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 1 | 4 | 2026-02-24 | DD-026 |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 1 | 4 | 2026-02-24 | none |
| CacheManager | tempest-app/src/cache_manager.rs | 1 | 7 | 2026-02-24 | DD-020 |
| Config | tempest-app/src/config.rs | 0 | 1 | 2026-02-24 | none |
| DecodeTypes | tempest-decode/src/types.rs | High | 1 | 2026-02-24 | DD-032, DD-033 |
| FetchCache | tempest-fetch/src/cache.rs | High | 1 | 2026-02-24 | DD-034, DD-035 |
| OfflineDetection | tempest-app/src/offline_detection.rs | 1 | 1 | 2026-02-24 | DD-036 |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives |
| --- | ---- | ------------------- | -------------- | ---------------- |
| 19 | 2026-02-24 | CacheManager, ColorLegend, OfflineDetection | DD-036 | 0 |
| 18 | 2026-02-24 | DecodeTypes, FetchCache | DD-032, DD-033, DD-034, DD-035 | 2 |
| 17 | 2026-02-24 | CacheManager, OfflineIndicator | 0 | 0 |
| 16 | 2026-02-24 | StationSelector, TimelineState | DD-024, DD-025, DD-031 | 0 |
| 15 | 2026-02-24 | ColorLegend, ElevationTiltSelector | DD-029, DD-030 | 0 |
| 14 | 2026-02-24 | MomentSwitcher, CacheManager | DD-026, DD-027, DD-028 | 2 |
| 13 | 2026-02-24 | TimelineState, Config | DD-022, DD-023, DD-024, DD-025 | 0 |

## Violation Counts (for worst-violations selection)

| Component | Total Violations | Unresolved |
| --------- | ----------------- | ---------- |
| TimelineState | 5 | 2 |
| CacheManager | 4 | 1 |
| StationSelector | 3 | 0 |
| ColorLegend | 4 | 1 |
| MomentSwitcher | 2 | 1 |
| ElevationTiltSelector | 3 | 0 |
| DecodeTypes | 2 | 2 |
| FetchCache | 2 | 2 |
| OfflineDetection | 1 | 1 |
