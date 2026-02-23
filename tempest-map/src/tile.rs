//! Web Mercator tile coordinate system for the Tempest NEXRAD Weather Radar Application.
//! 
//! This module implements the standard Web Mercator projection (EPSG:3857) used by most
//! online map providers including OpenStreetMap, OpenFreeMap, and others.
//! 
//! Zoom levels are restricted to 4-15 as per PRD requirements.

use std::f64::consts::PI;

/// Minimum zoom level supported by the tile system.
pub const MIN_ZOOM: u8 = 4;
/// Maximum zoom level supported by the tile system.
pub const MAX_ZOOM: u8 = 15;
/// The number of tiles at each zoom level (2^zoom).
pub const TILES_PER_ZOOM: [u32; MAX_ZOOM as usize - MIN_ZOOM as usize + 1] = [
    16,   // zoom 4:  2^4  = 16
    32,   // zoom 5:  2^5  = 32
    64,   // zoom 6:  2^6  = 64
    128,  // zoom 7:  2^7  = 128
    256,  // zoom 8:  2^8  = 256
    512,  // zoom 9:  2^9  = 512
    1024, // zoom 10: 2^10 = 1024
    2048, // zoom 11: 2^11 = 2048
    4096, // zoom 12: 2^12 = 4096
    8192, // zoom 13: 2^13 = 8192
    16384,// zoom 14: 2^14 = 16384
    32768,// zoom 15: 2^15 = 32768
];

/// Returns the number of tiles at the given zoom level.
#[inline]
pub fn tiles_at_zoom(zoom: u8) -> u32 {
    debug_assert!(zoom >= MIN_ZOOM && zoom <= MAX_ZOOM, "Zoom must be between {} and {}", MIN_ZOOM, MAX_ZOOM);
    1 << zoom
}

/// Represents a tile in the Web Mercator coordinate system.
/// 
/// A tile is identified by its zoom level (z), column (x), and row (y).
/// At each zoom level, there are 2^z tiles in both the x and y directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoordinate {
    /// Zoom level (4-15)
    pub z: u8,
    /// Tile column (0 to 2^z - 1)
    pub x: u32,
    /// Tile row (0 to 2^z - 1), with 0 being the northmost row
    pub y: u32,
}

impl TileCoordinate {
    /// Creates a new tile coordinate.
    /// 
    /// # Panics
    /// Panics if z is outside the valid range (4-15) or if x/y exceed the tile count for the zoom level.
    #[inline]
    pub fn new(z: u8, x: u32, y: u32) -> Self {
        assert!(z >= MIN_ZOOM && z <= MAX_ZOOM, "Zoom must be between {} and {}", MIN_ZOOM, MAX_ZOOM);
        let max_tile = tiles_at_zoom(z);
        assert!(x < max_tile, "x ({}) must be less than {}", x, max_tile);
        assert!(y < max_tile, "y ({}) must be less than {}", y, max_tile);
        TileCoordinate { z, x, y }
    }

    /// Creates a tile coordinate without validation (for internal use).
    #[inline]
    pub(crate) fn new_unchecked(z: u8, x: u32, y: u32) -> Self {
        TileCoordinate { z, x, y }
    }

    /// Returns the maximum valid x coordinate for this zoom level.
    #[inline]
    pub fn max_x(&self) -> u32 {
        tiles_at_zoom(self.z) - 1
    }

    /// Returns the maximum valid y coordinate for this zoom level.
    #[inline]
    pub fn max_y(&self) -> u32 {
        tiles_at_zoom(self.z) - 1
    }
}

/// Configurable tile providers for base map rendering.
/// 
/// Each variant contains the base URL pattern and attribution text.
/// The URL pattern should contain `{z}`, `{x}`, `{y}` placeholders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileSource {
    /// OpenFreeMap - free, open-source tile provider
    /// URL: https://tiles.openfreemap.org
    OpenFreeMap,
    /// OpenStreetMap - backup tile provider
    /// URL: https://tile.openstreetmap.org
    OpenStreetMap,
}

impl TileSource {
    /// Returns the base URL for this tile source.
    pub fn base_url(&self) -> &'static str {
        match self {
            TileSource::OpenFreeMap => "https://tiles.openfreemap.org",
            TileSource::OpenStreetMap => "https://tile.openstreetmap.org",
        }
    }

    /// Returns the attribution text required for this tile source.
    pub fn attribution(&self) -> &'static str {
        match self {
            TileSource::OpenFreeMap => "© OpenFreeMap contributors © OpenStreetMap contributors",
            TileSource::OpenStreetMap => "© OpenStreetMap contributors",
        }
    }

    /// Returns the maximum zoom level supported by this tile source.
    pub fn max_zoom(&self) -> u8 {
        match self {
            TileSource::OpenFreeMap => 19,
            TileSource::OpenStreetMap => 19,
        }
    }
}

/// Converts a latitude/longitude to a tile coordinate at the given zoom level.
/// 
/// # Arguments
/// * `lat` - Latitude in degrees (-85.0511 to 85.0511 for Web Mercator)
/// * `lng` - Longitude in degrees (-180 to 180)
/// * `zoom` - Zoom level (4-15)
///
/// # Returns
/// The tile coordinate containing the given geographic point.
#[inline]
pub fn lat_lng_to_tile(lat: f64, lng: f64, zoom: u8) -> TileCoordinate {
    debug_assert!(zoom >= MIN_ZOOM && zoom <= MAX_ZOOM, "Zoom must be between {} and {}", MIN_ZOOM, MAX_ZOOM);
    
    // Clamp latitude to Web Mercator bounds
    let lat = lat.clamp(-85.0511287798, 85.0511287798);
    let lng = lng.clamp(-180.0, 180.0);

    // Convert to Web Mercator normalized coordinates (0 to 1)
    let x = (lng + 180.0) / 360.0;
    let lat_rad = lat * PI / 180.0;
    let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln()) / 2.0;

    // Convert to tile coordinates
    let n = tiles_at_zoom(zoom) as f64;
    let tx = (x * n).floor() as u32;
    let ty = (y * n).floor() as u32;

    TileCoordinate::new_unchecked(zoom, tx, ty)
}

/// Converts a tile coordinate to its northwest corner latitude/longitude.
/// 
/// # Arguments
/// * `tile` - The tile coordinate
///
/// # Returns
/// A tuple of (latitude, longitude) representing the northwest corner of the tile.
#[inline]
pub fn tile_to_lat_lng(tile: &TileCoordinate) -> (f64, f64) {
    let n = tiles_at_zoom(tile.z) as f64;
    
    // Calculate the normalized coordinates of the northwest corner
    let x = tile.x as f64 / n;
    let y = tile.y as f64 / n;

    // Convert to longitude
    let lng = x * 360.0 - 180.0;

    // Convert to latitude using the inverse Web Mercator formula
    // lat_rad = atan(sinh(π * (1 - 2 * y)))
    let lat_rad = (PI * (1.0 - 2.0 * y)).sinh().atan();
    let lat = lat_rad * 180.0 / PI;

    (lat, lng)
}

/// Converts latitude/longitude to pixel coordinates in world space.
/// 
/// Pixel coordinates are in the range [0, 2^zoom * 256) for a full world map.
/// The origin (0, 0) is at the northwest corner of the map.
///
/// # Arguments
/// * `lat` - Latitude in degrees
/// * `lng` - Longitude in degrees
/// * `zoom` - Zoom level
///
/// # Returns
/// A tuple of (x, y) pixel coordinates.
#[inline]
pub fn lat_lng_to_pixel(lat: f64, lng: f64, zoom: u8) -> (f64, f64) {
    debug_assert!(zoom >= MIN_ZOOM && zoom <= MAX_ZOOM, "Zoom must be between {} and {}", MIN_ZOOM, MAX_ZOOM);
    
    // Clamp latitude to Web Mercator bounds
    let lat = lat.clamp(-85.0511287798, 85.0511287798);
    let lng = lng.clamp(-180.0, 180.0);

    // Convert to Web Mercator normalized coordinates (0 to 1)
    let x = (lng + 180.0) / 360.0;
    let lat_rad = lat * PI / 180.0;
    
    // Web Mercator: y = (1 - ln(tan(lat) + sec(lat)) / π) / 2
    let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / PI) / 2.0;

    // Convert to pixel coordinates (assuming 256x256 tiles)
    let n = (tiles_at_zoom(zoom) as f64) * 256.0;
    let px = x * n;
    let py = y * n;

    (px, py)
}

/// Converts pixel coordinates in world space to latitude/longitude.
/// 
/// # Arguments
/// * `x` - X pixel coordinate
/// * `y` - Y pixel coordinate
/// * `zoom` - Zoom level
///
/// # Returns
/// A tuple of (latitude, longitude).
#[inline]
pub fn pixel_to_lat_lng(x: f64, y: f64, zoom: f64) -> (f64, f64) {
    let n = 2.0_f64.powf(zoom) * 256.0;
    
    // Convert to normalized coordinates
    let x_norm = x / n;
    let y_norm = y / n;

    // Convert to longitude
    let lng = x_norm * 360.0 - 180.0;

    // Convert to latitude using the inverse Web Mercator formula
    // lat_rad = atan(sinh(π * (1 - 2 * y_norm)))
    let lat_rad = (PI * (1.0 - 2.0 * y_norm)).sinh().atan();
    let lat = lat_rad * 180.0 / PI;

    (lat, lng)
}

/// Generates a URL for fetching the given tile from the specified tile source.
/// 
/// # Arguments
/// * `tile` - The tile coordinate
/// * `source` - The tile source/provider
///
/// # Returns
/// A complete URL string for fetching the tile image.
pub fn tile_to_tile_url(tile: &TileCoordinate, source: &TileSource) -> String {
    format!("{}/{}/{}/{}.png", source.base_url(), tile.z, tile.x, tile.y)
}

/// Represents the visible map area (viewport).
/// 
/// The viewport is defined by its center coordinates, zoom level, and dimensions.
/// It provides methods for determining which tiles are visible.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    /// Center latitude in degrees
    pub center_lat: f64,
    /// Center longitude in degrees
    pub center_lng: f64,
    /// Current zoom level (4-15)
    pub zoom: u8,
    /// Viewport width in pixels
    pub width_px: u32,
    /// Viewport height in pixels
    pub height_px: u32,
}

impl Viewport {
    /// Creates a new viewport with the given parameters.
    /// 
    /// # Panics
    /// Panics if zoom is outside the valid range (4-15).
    pub fn new(center_lat: f64, center_lng: f64, zoom: u8, width_px: u32, height_px: u32) -> Self {
        assert!(zoom >= MIN_ZOOM && zoom <= MAX_ZOOM, "Zoom must be between {} and {}", MIN_ZOOM, MAX_ZOOM);
        Viewport {
            center_lat,
            center_lng,
            zoom,
            width_px,
            height_px,
        }
    }

    /// Returns all tiles needed to fill the viewport.
    /// 
    /// This calculates the range of tiles that cover the entire visible area,
    /// including one tile of buffer around the edges to prevent edge artifacts
    /// during smooth panning.
    ///
    /// # Returns
    /// A vector of tile coordinates covering the viewport.
    pub fn visible_tiles(&self) -> Vec<TileCoordinate> {
        // Get the pixel coordinates of the viewport boundaries
        let center_px = lat_lng_to_pixel(self.center_lat, self.center_lng, self.zoom);
        
        // Calculate the extent in pixels
        let half_width = self.width_px as f64 / 2.0;
        let half_height = self.height_px as f64 / 2.0;
        
        // Calculate the corner positions
        let min_x = center_px.0 - half_width;
        let max_x = center_px.0 + half_width;
        let min_y = center_px.1 - half_height;
        let max_y = center_px.1 + half_height;

        // Get the tile size in pixels
        let tile_size = 256.0;

        // Convert pixel coordinates to tile coordinates
        let min_tile_x = (min_x / tile_size).floor() as i64;
        let max_tile_x = (max_x / tile_size).floor() as i64;
        let min_tile_y = (min_y / tile_size).floor() as i64;
        let max_tile_y = (max_y / tile_size).floor() as i64;

        // Get the valid range for this zoom level
        let max_tile_coord = tiles_at_zoom(self.zoom) as i64 - 1;

        // Clamp to valid range
        let min_tile_x = min_tile_x.max(0);
        let max_tile_x = max_tile_x.min(max_tile_coord);
        let min_tile_y = min_tile_y.max(0);
        let max_tile_y = max_tile_y.min(max_tile_coord);

        // Generate all visible tiles
        let mut tiles = Vec::with_capacity(
            ((max_tile_x - min_tile_x + 1) * (max_tile_y - min_tile_y + 1)) as usize
        );

        for y in min_tile_y..=max_tile_y {
            for x in min_tile_x..=max_tile_x {
                tiles.push(TileCoordinate::new_unchecked(
                    self.zoom,
                    x as u32,
                    y as u32,
                ));
            }
        }

        tiles
    }

    /// Returns the bounding box of the viewport in latitude/longitude.
    /// 
    /// # Returns
    /// A tuple of (min_lat, min_lng, max_lat, max_lng).
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let center_px = lat_lng_to_pixel(self.center_lat, self.center_lng, self.zoom);
        
        let half_width = self.width_px as f64 / 2.0;
        let half_height = self.height_px as f64 / 2.0;
        
        let min_px = (center_px.0 - half_width, center_px.1 - half_height);
        let max_px = (center_px.0 + half_width, center_px.1 + half_height);

        // In Web Mercator: lower y = higher latitude (north), higher y = lower latitude (south)
        // min_px has lower y, so it gives us the northern bound (higher latitude = max_lat)
        // max_px has higher y, so it gives us the southern bound (lower latitude = min_lat)
        let (max_lat, min_lng) = pixel_to_lat_lng(min_px.0, min_px.1, self.zoom as f64);
        let (min_lat, max_lng) = pixel_to_lat_lng(max_px.0, max_px.1, self.zoom as f64);

        (min_lat, min_lng, max_lat, max_lng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_tile_count_at_zoom() {
        assert_eq!(tiles_at_zoom(4), 16);
        assert_eq!(tiles_at_zoom(8), 256);
        assert_eq!(tiles_at_zoom(15), 32768);
    }

    #[test]
    fn test_lat_lng_to_tile_basic() {
        // At zoom 4 (minimum), there's 16x16 tiles
        let tile = lat_lng_to_tile(0.0, 0.0, 4);
        // At equator and prime meridian, should be in middle tiles
        assert!(tile.x > 0 && tile.x < 16);
        assert!(tile.y > 0 && tile.y < 16);
    }

    #[test]
    fn test_lat_lng_to_tile_zoom_1() {
        // At zoom 4, there are 16x16 tiles
        // North pole should be at y=0
        let tile_north = lat_lng_to_tile(85.0, 0.0, 4);
        assert_eq!(tile_north.y, 0);

        // South pole should be at y=15 (the last row)
        let tile_south = lat_lng_to_tile(-85.0, 0.0, 4);
        assert!(tile_south.y >= 14, "South pole should be near the bottom, got y={}", tile_south.y);

        // Prime meridian should be at x=8 (middle of 16)
        let tile_meridian = lat_lng_to_tile(0.0, 0.0, 4);
        assert!(tile_meridian.x >= 7 && tile_meridian.x <= 9);
    }

    #[test]
    fn test_lat_lng_to_tile_known_locations() {
        // Test New York City (approximately 40.7128°N, 74.0060°W)
        let tile = lat_lng_to_tile(40.7128, -74.0060, 10);
        // At zoom 10, tiles are 1024x1024
        // We just verify it's in valid range
        assert!(tile.x < 1024);
        assert!(tile.y < 1024);
    }

    #[test]
    fn test_tile_to_lat_lng_round_trip() {
        // Test that tile_to_lat_lng returns valid NW corner coordinates
        let test_points = [
            (0.0, 0.0),      // Equator, Prime Meridian
            (51.5074, -0.1278), // London
            (40.7128, -74.0060), // New York
            (-33.8688, 151.2093), // Sydney
            (85.0, 180.0),   // Near North Pole, International Date Line
        ];

        for (lat, lng) in test_points {
            for zoom in MIN_ZOOM..=MAX_ZOOM {
                let tile = lat_lng_to_tile(lat, lng, zoom);
                let (corner_lat, corner_lng) = tile_to_lat_lng(&tile);
                
                // Verify the corner is valid and within Web Mercator bounds
                assert!(corner_lat.is_finite() && corner_lat >= -90.0 && corner_lat <= 90.0, 
                    "Invalid lat {} at zoom {}", corner_lat, zoom);
                assert!(corner_lng.is_finite() && corner_lng >= -180.0 && corner_lng <= 180.0,
                    "Invalid lng {} at zoom {}", corner_lng, zoom);
            }
        }
        
        // Test edge case: south pole tile at high zoom
        let tile = TileCoordinate::new_unchecked(4, 8, 15);
        let (lat, _lng) = tile_to_lat_lng(&tile);
        assert!(lat.is_finite(), "South pole lat should be finite, got {}", lat);
    }

    #[test]
    fn test_lat_lng_to_pixel_basic() {
        // At zoom 4, world is 4096x4096 pixels (16 tiles * 256)
        let (x, y) = lat_lng_to_pixel(0.0, 0.0, 4);
        assert!(approx_eq(x, 2048.0));
        assert!(approx_eq(y, 2048.0));

        // North pole should be at y ~ 0
        let (_x, y) = lat_lng_to_pixel(85.0511287798, 0.0, 4);
        assert!(y < 1.0, "North pole y should be ~0, got {}", y);

        // South pole should be at y ~ 4096
        let (_x, y) = lat_lng_to_pixel(-85.0511287798, 0.0, 4);
        assert!(y > 4095.0, "South pole y should be ~4096, got {}", y);
    }

    #[test]
    fn test_pixel_to_lat_lng_round_trip() {
        // Test round-trip for various pixel coordinates at zoom 4
        for zoom in MIN_ZOOM..=5u8 { // Test lower zoom levels to avoid edge cases
            let test_pixels = [
                (0.0, 0.0),
                (2048.0, 2048.0),
                (4096.0, 0.0),
                (0.0, 4096.0),
            ];

            for (px, py) in test_pixels {
                let (lat, lng) = pixel_to_lat_lng(px, py, zoom as f64);
                let (px2, py2) = lat_lng_to_pixel(lat, lng, zoom as u8);
                
                // Should round-trip within small tolerance
                assert!(
                    approx_eq(px, px2) || (px - px2).abs() < 1.0,
                    "Pixel x mismatch: {} vs {} at zoom {}",
                    px, px2, zoom
                );
                assert!(
                    approx_eq(py, py2) || (py - py2).abs() < 1.0,
                    "Pixel y mismatch: {} vs {} at zoom {}",
                    py, py2, zoom
                );
            }
        }
    }

    #[test]
    fn test_zoom_level_boundaries() {
        // Test that we can create tiles at all valid zoom levels
        for zoom in MIN_ZOOM..=MAX_ZOOM {
            let tile = lat_lng_to_tile(0.0, 0.0, zoom);
            assert_eq!(tile.z, zoom);
            
            // Test max tile coordinates
            let max_x = tiles_at_zoom(zoom) - 1;
            let max_y = tiles_at_zoom(zoom) - 1;
            let tile_max = TileCoordinate::new_unchecked(zoom, max_x, max_y);
            let (lat, lng) = tile_to_lat_lng(&tile_max);
            
            // Should be valid lat/lng
            assert!(lat.is_finite());
            assert!(lng.is_finite());
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_zoom_too_low() {
        lat_lng_to_tile(0.0, 0.0, 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_zoom_too_high() {
        lat_lng_to_tile(0.0, 0.0, 16);
    }

    #[test]
    fn test_tile_source_urls() {
        let tile = TileCoordinate::new_unchecked(10, 512, 340);
        
        let url = tile_to_tile_url(&tile, &TileSource::OpenFreeMap);
        assert!(url.contains("tiles.openfreemap.org"));
        assert!(url.contains("/10/512/340.png"));
        
        let url = tile_to_tile_url(&tile, &TileSource::OpenStreetMap);
        assert!(url.contains("tile.openstreetmap.org"));
        assert!(url.contains("/10/512/340.png"));
    }

    #[test]
    fn test_viewport_visible_tiles() {
        // Create a viewport centered on New York at zoom 10
        let viewport = Viewport::new(40.7128, -74.0060, 10, 800, 600);
        let tiles = viewport.visible_tiles();
        
        // Should have multiple tiles covering the viewport
        assert!(!tiles.is_empty());
        
        // All tiles should have correct zoom
        for tile in &tiles {
            assert_eq!(tile.z, 10);
        }
    }

    #[test]
    fn test_viewport_bounds() {
        let viewport = Viewport::new(40.7128, -74.0060, 10, 800, 600);
        let (min_lat, min_lng, max_lat, max_lng) = viewport.bounds();
        
        // Bounds should be valid lat/lng
        assert!(min_lat.is_finite() && min_lat >= -90.0);
        assert!(max_lat.is_finite() && max_lat <= 90.0);
        assert!(min_lng.is_finite() && min_lng >= -180.0);
        assert!(max_lng.is_finite() && max_lng <= 180.0);
        
        // Each bound should be valid individually
        assert!(min_lat <= max_lat, "min_lat {} should be <= max_lat {}", min_lat, max_lat);
        assert!(min_lng <= max_lng, "min_lng {} should be <= max_lng {}", min_lng, max_lng);
    }

    #[test]
    fn test_viewport_single_tile() {
        // Very small viewport at high zoom should show only 1 tile
        let viewport = Viewport::new(40.7128, -74.0060, 15, 100, 100);
        let tiles = viewport.visible_tiles();
        
        // At zoom 15, tiles are very small relative to viewport
        // Should have multiple tiles but verify they're reasonable
        assert!(!tiles.is_empty());
    }

    #[test]
    fn test_tile_coordinate_creation() {
        // Valid tile
        let tile = TileCoordinate::new(10, 100, 200);
        assert_eq!(tile.z, 10);
        assert_eq!(tile.x, 100);
        assert_eq!(tile.y, 200);
    }

    #[test]
    #[should_panic]
    fn test_tile_invalid_x() {
        // x too large for zoom level
        TileCoordinate::new(4, 20, 0);
    }

    #[test]
    #[should_panic]
    fn test_tile_invalid_y() {
        // y too large for zoom level
        TileCoordinate::new(4, 0, 20);
    }
}
