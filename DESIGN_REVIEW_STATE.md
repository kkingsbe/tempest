# Design Review State

> Last Updated: 2026-02-24T13:08:00Z
> Total runs completed: 17

## Component Registry

| Component | Path | Usage Count | Times Reviewed | Last Reviewed | Open Debt Items |
| --------- | ---- | ----------- | -------------- | ------------- | --------------- |
| App (main.rs) | tempest-app/src/main.rs | 1 | 3 | 2026-02-24 | none |
| TimelineState | tempest-app/src/timeline.rs | 1 | 7 | 2026-02-24 | DD-024, DD-025, DD-031 |
| StationSelector | tempest-app/src/station_selector.rs | 1 | 5 | 2026-02-24 | none |
| ColorLegend | tempest-app/src/color_legend.rs | 6 | 4 | 2026-02-24 | DD-019, DD-029 |
| ElevationTiltSelector | tempest-app/src/elevation_tilt_selector.rs | 5 | 5 | 2026-02-24 | DD-030 |
| MomentSwitcher | tempest-app/src/moment_switcher.rs | 1 | 4 | 2026-02-24 | DD-026 |
| OfflineIndicator | tempest-app/src/offline_indicator.rs | 1 | 4 | 2026-02-24 | none |
| CacheManager | tempest-app/src/cache_manager.rs | 1 | 6 | 2026-02-24 | DD-020, DD-027, DD-028 |
| Config | tempest-app/src/config.rs | 0 | 1 | 2026-02-24 | none |

## Recent Runs (last 20 only)

| Run | Date | Components Reviewed | New Debt Items | False Positives |
| --- | ---- | ------------------- | -------------- | ---------------- |
| 17 | 2026-02-24 | CacheManager, OfflineIndicator | 0 | 0 |
| 16 | 2026-02-24 | StationSelector, TimelineState | DD-024, DD-025, DD-031 | 0 |
| 15 | 2026-02-24 | ColorLegend, ElevationTiltSelector | DD-029, DD-030 | 0 |
| 14 | 2026-02-24 | MomentSwitcher, CacheManager | DD-026, DD-027, DD-028 | 2 |
| 13 | 2026-02-24 | TimelineState, Config | DD-022, DD-023, DD-024, DD-025 | 0 |
| 1 | 2026-02-23 | App, TimelineState, StationSelector | DD-001, DD-002, DD-003 | 0 |
| 2 | 2026-02-23 | ColorLegend, ElevationTiltSelector | DD-004, DD-005, DD-006, DD-007 | 0 |
| 3 | 2026-02-23 | MomentSwitcher, OfflineIndicator | DD-008, DD-009, DD-010, DD-011 | 0 |
| 4 | 2026-02-23 | CacheManager, TimelineState | DD-012, DD-013, DD-014 | 0 |
| 5 | 2026-02-23 | All 8 components (verified existing debt) | 0 | 0 |
| 6 | 2026-02-24 | CacheManager, TimelineState, main.rs | 0 | 0 |
| 7 | 2026-02-24 | ColorLegend, ElevationTiltSelector | DD-015, DD-016 | 0 |
| 8 | 2026-02-24 | TimelineState, CacheManager | 0 | 2 |
| 9 | 2026-02-24 | ColorLegend, StationSelector | DD-017 | 0 |
| 10 | 2026-02-24 | MomentSwitcher, OfflineIndicator, TimelineState, CacheManager | DD-018 | 0 |
| 11 | 2026-02-24 | ColorLegend, CacheManager | DD-019, DD-020 | 0 |
| 12 | 2026-02-24 | StationSelector, ElevationTiltSelector | DD-021 | 0 |

## Violation Counts (for worst-violations selection)

| Component | Total Violations | Unresolved |
| --------- | ----------------- | ---------- |
| TimelineState | 5 | 3 |
| CacheManager | 3 | 3 |
| StationSelector | 3 | 0 |
| ColorLegend | 4 | 2 |
| MomentSwitcher | 2 | 1 |
| ElevationTiltSelector | 3 | 1 |
