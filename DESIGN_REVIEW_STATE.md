# Design Review State

> Last Updated: 2026-02-23T19:58:00.000Z
> Total runs completed: 2

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 2 | 2026-02-23 | DD-001 |
| TimelineState | tempest-app/src/timeline.rs | 1 | 2 | 2026-02-23 | DD-002 |
| StationSelector | tempest-app/src/station_selector.rs | 1 | 2 | 2026-02-23 | DD-003 |
| ColorLegend | tempest-app/src/color_legend.rs | 6 | 1 | 2026-02-23 | DD-004, DD-005 |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 5 | 1 | 2026-02-23 | DD-006, DD-007 |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 1 | 1 | 2026-02-23 | (debt found but not queued this run - skip) |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 1 | 1 | 2026-02-23 | (debt found but not queued this run - skip) |
| CacheManager | tempest-app/src/cache_manager.rs | 1 | 0 | never | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives Caught |
| --- | ---- | ------------------- | -------------- | ---------------------- |
| 1 | 2026-02-23 | App, TimelineState, StationSelector | DD-001, DD-002, DD-003 | 0 |
| 2 | 2026-02-23 | ColorLegend, ElevationTiltSelector | DD-004, DD-005, DD-006, DD-007 | 0 |

## Violation Counts (for worst-violations selection)

| Component | Total Violations Found | Unresolved |
| --------- | ---------------------- | ---------- |
| App | 1 | 1 |
| TimelineState | 1 | 1 |
| StationSelector | 1 | 1 |
| ColorLegend | 2 | 2 |
| ElevationTiltSelector | 2 | 2 |
| MomentSwitcher | 2 | 2 |
| OfflineIndicator | 2 | 2 |
