# Design Review State

> Last Updated: 2026-02-23T19:35:00.000Z
> Total runs completed: 1

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 1 | 2026-02-23 | DD-001 |
| TimelineState | tempest-app/src/timeline.rs | 1 | 1 | 2026-02-23 | DD-002 |
| StationSelector | tempest-app/src/station_selector.rs | 1 | 1 | 2026-02-23 | DD-003 |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 1 | 0 | never | none |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 1 | 0 | never | none |
| ColorLegend | tempest-app/src/color_legend.rs | 1 | 0 | never | none |
| CacheManager | tempest-app/src/cache_manager.rs | 1 | 0 | never | none |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 1 | 0 | never | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives Caught |
| --- | ---- | ------------------- | -------------- | ---------------------- |
| 1 | 2026-02-23 | App, TimelineState, StationSelector | DD-001, DD-002, DD-003 | 0 |

## Violation Counts (for worst-violations selection)

| Component | Total Violations Found | Unresolved |
| --------- | ---------------------- | ---------- |
| App | 1 | 1 |
| TimelineState | 1 | 1 |
| StationSelector | 1 | 1 |
