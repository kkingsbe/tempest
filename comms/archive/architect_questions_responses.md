# PRD Clarification Questions - Responses

## 1. Station/S3 Discovery
**Question**: How should we list available radar stations/scans? Unsigned S3 requests don't support ListObjects operation.

**Response**: Adjust as needed (not specified - implementer to determine approach)

---

## 2. Map Dark Theme
**Question**: PRD claims "dark theme" using OpenStreetMap + shader inversion, but OSM is light-themed. Inversion creates a negative image, not a dark map. What's the desired approach?

**Response**: Yes (proceed with dark theme approach)

---

## 3. Binary Size Target
**Question**: Target of <50MB is likely impossible with Rust + wgpu + static linking (realistic: 80-150MB). Is this acceptable?

**Response**: That's fine

---

## 4. Playback Speed Definition
**Question**: "1x, 2x, 5x, 10x" - is 1x = 1 scan/second or real-time (1 volume per ~6 minutes)?

**Response**: 1x = 1 frame per second

---

## 5. Color Table Format
**Question**: States "data (not code)" but doesn't define format (JSON? TOML?).

**Response**: Up to you (implementer discretion)

---

## 6. Polling During Playback
**Question**: Default 60-second polling - should it continue during animated timeline playback?

**Response**: Yes

---

## 7. Visual Regression Threshold
**Question**: 1.5% pixel diff is very tight; may cause false CI failures across platforms. Should this be increased?

**Response**: Loosen to 3%

---

## 8. API Key Handling
**Question**: Commercial tile providers (Stadia, MapTiler) require API keys - should we add a configuration mechanism?

**Response**: Use OpenFreeMap (free tiles): https://tiles.openfreemap.org
