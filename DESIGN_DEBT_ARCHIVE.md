# Design Debt Archive

> Last Updated: 2026-02-25T13:08:00Z
> Total Archived: 51

| ID     | Component  | Priority | Status   | Summary                                           | Resolved   |
| ------ | ---------- | -------- | -------- | ------------------------------------------------- | ---------- |
| DD-037 | station_selector.rs | Medium | RESOLVED | Button padding now uses spacing::MD (12px) - fixed | 2026-02-25 |
| DD-052 | color_legend.rs | Medium | RESOLVED | Section spacing now uses LG (24px) - fixed | 2026-02-25 |
| DD-051 | color_legend.rs | Medium | RESOLVED | Within-group spacing now uses LG (24px) - fixed | 2026-02-25 |
| DD-049 | elevation_tilt_selector.rs | High | RESOLVED | Button padding now has [12, 24] - fixed | 2026-02-25 |
| DD-048 | elevation_tilt_selector.rs | Medium | RESOLVED | Container padding now uses 16px (BASE) - fixed | 2026-02-25 |

| ID     | Component  | Priority | Status   | Summary                                           | Resolved   |
| ------ | ---------- | -------- | -------- | ------------------------------------------------- | ---------- |
| DD-041 | station_selector.rs | High | SCHEDULED | Column spacing (4px) + Visual Proximity - scheduled for Sprint 23 | 2026-02-25 |
| DD-039 | color_legend.rs | High | SCHEDULED | Container padding below minimum (SM=8 < 16) - scheduled for Sprint 23 | 2026-02-25 |
| DD-044 | timeline.rs | High | SCHEDULED | Button padding (SM=8 < 12) + Section spacing (SM=8 < 24) - scheduled Sprint 23 | 2026-02-25 |
| DD-053 | offline_indicator.rs | High | SCHEDULED | Container padding (8,12 < 16) + Within-group spacing (4 < 8) - scheduled Sprint 23 | 2026-02-25 |
| DD-056 | station_selector.rs | Medium | SCHEDULED | Form field padding (8 < 12) - scheduled for Sprint 23 | 2026-02-25 |
| DD-040 | cache_manager.rs | Low | RESOLVED | Non-8-point padding (10) - fixed | 2026-02-24 |
| DD-035 | cache.rs | Medium | RESOLVED | expect() in production - fixed (now uses ok_or_else) | 2026-02-24 |
| DD-034 | cache.rs | Medium | RESOLVED | Cloning in loop - fixed (only calls to_string once) | 2026-02-24 |
| DD-033 | types.rs | Medium | RESOLVED | String vs &str - fixed (now accepts &str) | 2026-02-24 |
| DD-032 | types.rs | Medium | RESOLVED | as_str_lossy naming - fixed (now named to_string_lossy) | 2026-02-24 |
| DD-036 | offline_detection.rs | Medium | RESOLVED | expect() in production - fixed (replaced with proper error handling) | 2026-02-24 |
| DD-031 | timeline.rs | High | RESOLVED | Non-8-point TIMELINE_HEIGHT (56→48) - fixed | 2026-02-24 |
| DD-024 | timeline.rs | High | RESOLVED | Non-8-point TOTAL_HEIGHT calculation - fixed | 2026-02-24 |
| DD-030 | elevation_tilt_selector.rs | Medium | RESOLVED | Non-8-point button dimensions (60x40 → 48x48) - fixed | 2026-02-24 |
| DD-025 | timeline.rs | High | RESOLVED | Non-8-point tick container height (10→8) - fixed | 2026-02-24 |
| DD-022 | timeline.rs | Medium | RESOLVED | TICK_HEIGHT fixed 20→16 | 2026-02-24 |
| DD-023 | timeline.rs | Medium | RESOLVED | LABEL_HEIGHT fixed 18→16 | 2026-02-24 |
| DD-029 | timeline.rs | High | RESOLVED | Non-8-point LABEL_HEIGHT constant - fixed 2026-02-24 | 2026-02-24 |
| DD-029 | color_legend.rs | Medium | SCHEDULED | Non-8-point spacing values (30, 6, 90) - scheduled for Sprint 19 | 2026-02-24 |
| DD-028 | cache_manager.rs | Medium | SCHEDULED | Typography scale violation (.size(5)) - scheduled for Sprint 19 | 2026-02-24 |
| DD-027 | cache_manager.rs | High | SCHEDULED | Button padding below minimum (8, 4 instead of 12, 24) - scheduled for Sprint 19 | 2026-02-24 |
| DD-026 | moment_switcher.rs | Medium | SCHEDULED | Unused semantic colors - scheduled for Sprint 19 | 2026-02-24 |
| DD-025 | timeline.rs | Medium | SCHEDULED | Non-8-point TICK_HEIGHT calculation - scheduled for Sprint 19 | 2026-02-24 |
| DD-024 | timeline.rs | High | SCHEDULED | Non-8-point TOTAL_HEIGHT calculation - scheduled for Sprint 19 | 2026-02-24 |
| DD-023 | timeline.rs | High | SCHEDULED | Non-8-point LABEL_HEIGHT constant (18→16) - scheduled for Sprint 19 | 2026-02-24 |
| DD-018 | timeline.rs | High | OUTDATED | Non-8-point spacing - code no longer exists at cited lines | 2026-02-24 |
| DD-021 | station_selector.rs | Medium | SCHEDULED | Inline RGB colors instead of semantic constants - scheduled for sprint 18 | 2026-02-24 |
| DD-020 | cache_manager.rs | High | SCHEDULED | Non-8-point padding values (10, 5, 20) - scheduled for sprint 18 | 2026-02-24 |
| DD-019 | color_legend.rs | Medium | SCHEDULED | Non-8-point spacing (zero) - scheduled for sprint 18 | 2026-02-24 |
| DD-017 | StationSelector | Medium | RESOLVED | Non-8-point spacing (15, 5, 20) - fixed in Sprint 16 | 2026-02-24 |
| DD-012 | CacheManager | Medium | RESOLVED | Non-8-point spacing (10, 5, 20) - fixed in Sprint 16 | 2026-02-24 |
| DD-014 | TimelineState | Medium | OUTDATED | Raw RGB colors resolved (now uses semantic colors), new spacing violations in DD-018 | 2026-02-24 |
| DD-016 | ColorLegend  | Medium | RESOLVED | Container padding added - now has .padding(12) | 2026-02-24 |
| DD-013 | CacheManager | Low | OUTDATED | false positive - no Color::from_rgb found | 2026-02-24 |
| DD-015 | ColorLegend  | Low | OUTDATED | false positive - has compliant padding   | 2026-02-24 |
| DD-001 | tempest-app/src/main.rs | High | SCHEDULED | Deprecated Sandbox API - scheduled for Sprint 15 | 2026-01-24 |
| DD-002 | tempest-app/src/timeline.rs | High | SCHEDULED | Arbitrary Spacing Values - scheduled for Sprint 15 | 2026-01-24 |
| DD-004 | tempest-app/src/color_legend.rs | High | SCHEDULED | Raw RGB Color Values in ColorLegend - scheduled | 2026-01-24 |
| DD-003 | tempest-app/src/station_selector.rs | High | SCHEDULED | Raw RGB Color Values | 2026-01-23 |
| DD-005 | tempest-app/src/color_legend.rs | High | SCHEDULED | Non-8-Point Spacing in ColorLegend | 2026-01-23 |
| DD-006 | tempest-app/src/elevation_tilt_selector.rs | High | SCHEDULED | Raw RGB Color Values in ElevationTiltSelector | 2026-01-23 |
| DD-007 | tempest-app/src/elevation_tilt_selector.rs | High | SCHEDULED | Non-8-Point Spacing in ElevationTiltSelector | 2026-01-23 |
| DD-008 | tempest-app/src/moment_switcher.rs | High | SCHEDULED | Raw RGB Color Values in MomentSwitcher | 2026-01-23 |
| DD-009 | tempest-app/src/moment_switcher.rs | High | SCHEDULED | Non-8-Point Spacing in MomentSwitcher | 2026-01-23 |
| DD-010 | tempest-app/src/offline_indicator.rs | High | SCHEDULED | Raw RGB Color Values in OfflineIndicator | 2026-01-23 |
| DD-011 | tempest-app/src/offline_indicator.rs | High | SCHEDULED | Non-8-Point Padding in OfflineIndicator | 2026-01-23 |
