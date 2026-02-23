//! Tempest Map - Phase 5: Interactive base map
//! 
//! This crate provides the Web Mercator tile coordinate system and related
//! functionality for rendering interactive weather radar maps.

pub mod tile;
pub mod tile_manager;

pub use tile::{
    lat_lng_to_pixel,
    lat_lng_to_tile,
    pixel_to_lat_lng,
    tile_to_lat_lng,
    tile_to_tile_url,
    tiles_at_zoom,
    TileCoordinate,
    TileSource,
    Viewport,
    MAX_ZOOM,
    MIN_ZOOM,
};

pub use tile_manager::{
    Tile,
    TileCache,
    TileError,
    TileManager,
};
