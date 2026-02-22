# PRD Clarification Questions

The following questions need clarification before implementation can proceed reliably:

1. **Station/S3 Discovery**: How should we list available radar stations/scans? Unsigned S3 requests don't support ListObjects operation.

2. **Map Dark Theme**: PRD claims "dark theme" using OpenStreetMap + shader inversion, but OSM is light-themed. Inversion creates a negative image, not a dark map. What's the desired approach?

3. **Binary Size Target**: Target of <50MB is likely impossible with Rust + wgpu + static linking (realistic: 80-150MB). Is this acceptable?

4. **Playback Speed Definition**: "1x, 2x, 5x, 10x" - is 1x = 1 scan/second or real-time (1 volume per ~6 minutes)?

5. **Color Table Format**: States "data (not code)" but doesn't define format (JSON? TOML?).

6. **Polling During Playback**: Default 60-second polling - should it continue during animated timeline playback?

7. **Visual Regression Threshold**: 1.5% pixel diff is very tight; may cause false CI failures across platforms. Should this be increased?

8. **API Key Handling**: Commercial tile providers (Stadia, MapTiler) require API keys - should we add a configuration mechanism?

Please prioritize questions 1-3 as they affect fundamental architecture.
