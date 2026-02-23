//! View transformation module for map projection.
//!
//! This module provides the `ViewTransform` struct which handles the conversion
//! of geographic coordinates (latitude/longitude) to GPU clip space coordinates.
//! It supports zoom, rotation, and aspect ratio correction.
//!
//! # Design
//!
//! The view transformation pipeline:
//! 1. Convert lat/lon to offset from map center (in degrees)
//! 2. Apply rotation to the offset
//! 3. Apply zoom scaling
//! 4. Apply aspect ratio correction
//! 5. Add center position to get final NDC coordinates
//!
//! # Usage
//!
//! ```ignore
//! use tempest_render::view_transform::ViewTransform;
//!
//! let mut view = ViewTransform::new(35.4183, -97.4514, 1.0, 16.0 / 9.0);
//! let (x, y) = view.to_clip_space(35.5, -97.5);
//! view.set_rotation(45.0);
//! view.set_zoom(2.0);
//! let uniforms = view.to_uniforms();
//! ```

use bytemuck::{Pod, Zeroable};

/// View transformation state for map rendering.
///
/// This struct encapsulates all parameters needed to transform
/// geographic coordinates (WGS84 lat/lon) to GPU clip space coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewTransform {
    /// Map center latitude in degrees (-90 to 90).
    center_lat: f64,
    /// Map center longitude in degrees (-180 to 180).
    center_lon: f64,
    /// Zoom level (affects visible range scale).
    /// Higher values = more zoomed in (smaller visible area).
    zoom: f32,
    /// Map rotation in degrees (clockwise from north).
    rotation: f32,
    /// Screen aspect ratio (width / height) for correction.
    aspect_ratio: f32,
}

impl ViewTransform {
    /// Creates a new ViewTransform with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `center_lat` - Map center latitude in degrees (-90 to 90)
    /// * `center_lon` - Map center longitude in degrees (-180 to 180)
    /// * `zoom` - Zoom level (1.0 = default, higher = more zoomed in)
    /// * `aspect_ratio` - Screen aspect ratio (width / height)
    ///
    /// # Examples
    ///
    /// ```
    /// use tempest_render::view_transform::ViewTransform;
    ///
    /// // Create view centered on Oklahoma City (KTLX radar)
    /// let view = ViewTransform::new(35.4183, -97.4514, 1.0, 16.0 / 9.0);
    /// ```
    pub fn new(center_lat: f64, center_lon: f64, zoom: f32, aspect_ratio: f32) -> Self {
        Self {
            center_lat,
            center_lon,
            zoom: zoom.max(0.01), // Prevent division by zero
            rotation: 0.0,
            aspect_ratio: aspect_ratio.max(0.01), // Prevent division by zero
        }
    }

    /// Converts a geographic coordinate (lat/lon) to clip space coordinates.
    ///
    /// This method performs the full transformation pipeline:
    /// 1. Compute offset from map center
    /// 2. Apply rotation
    /// 3. Apply zoom scaling
    /// 4. Apply aspect ratio correction
    /// 5. Return NDC coordinates (-1 to 1)
    ///
    /// # Arguments
    ///
    /// * `lat` - Latitude in degrees
    /// * `lon` - Longitude in degrees
    ///
    /// # Returns
    ///
    /// A tuple of (x, y) in normalized device coordinates (-1 to 1).
    ///
    /// # Examples
    ///
    /// ```
    /// use tempest_render::view_transform::ViewTransform;
    ///
    /// let view = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);
    /// let (x, y) = view.to_clip_space(35.4183, -97.4514);
    /// // Center point should be at origin
    /// assert!((x - 0.0).abs() < 0.001);
    /// assert!((y - 0.0).abs() < 0.001);
    /// ```
    pub fn to_clip_space(&self, lat: f64, lon: f64) -> (f32, f32) {
        // Step 1: Compute offset from center in degrees
        let delta_lat = lat - self.center_lat;
        let delta_lon = lon - self.center_lon;

        // Step 2: Apply rotation (clockwise for map rotation)
        // Convert rotation to radians (negative for clockwise)
        let rotation_rad = -(self.rotation as f64).to_radians();
        let cos_r = rotation_rad.cos();
        let sin_r = rotation_rad.sin();

        // Rotate the offset (lon is x, lat is y in our coordinate system)
        // Note: In geographic coordinates, positive lat is north, positive lon is east
        let rotated_x = delta_lon * cos_r - delta_lat * sin_r;
        let rotated_y = delta_lon * sin_r + delta_lat * cos_r;

        // Step 3: Apply zoom scaling
        // Higher zoom = smaller visible area = divide by more
        let zoom = self.zoom as f64;

        // Step 4: Apply aspect ratio correction
        // This ensures the map doesn't appear stretched
        let aspect_ratio = self.aspect_ratio as f64;
        let aspect_corrected_x = rotated_x / (aspect_ratio * zoom);
        let aspect_corrected_y = rotated_y / zoom;

        // Convert to f32 and return
        (aspect_corrected_x as f32, aspect_corrected_y as f32)
    }

    /// Updates the map rotation.
    ///
    /// # Arguments
    ///
    /// * `degrees` - Rotation angle in degrees (clockwise from north)
    pub fn set_rotation(&mut self, degrees: f32) {
        // Normalize to 0-360 range
        let normalized = degrees % 360.0;
        if normalized < 0.0 {
            self.rotation = normalized + 360.0;
        } else {
            self.rotation = normalized;
        }
    }

    /// Updates the zoom level.
    ///
    /// # Arguments
    ///
    /// * `zoom` - New zoom level (must be positive, higher = more zoomed in)
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.01);
    }

    /// Returns the current zoom level.
    #[inline]
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Returns the current rotation in degrees.
    #[inline]
    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    /// Returns the current center latitude.
    #[inline]
    pub fn center_lat(&self) -> f64 {
        self.center_lat
    }

    /// Returns the current center longitude.
    #[inline]
    pub fn center_lon(&self) -> f64 {
        self.center_lon
    }

    /// Returns the current aspect ratio.
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Converts the view transform to uniforms for GPU upload.
    ///
    /// Returns a `ViewUniforms` struct that can be uploaded directly
    /// to a wgpu uniform buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempest_render::view_transform::ViewTransform;
    ///
    /// let view = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);
    /// let uniforms = view.to_uniforms();
    /// // Upload uniforms to GPU buffer
    /// ```
    pub fn to_uniforms(&self) -> ViewUniforms {
        ViewUniforms {
            // Center X position in NDC (0 = center of screen)
            center_x: 0.0,
            // Center Y position in NDC (0 = center of screen)
            center_y: 0.0,
            // Scale factor (1/zoom) for converting degrees to NDC
            scale: 1.0 / self.zoom,
            // Aspect ratio for correction
            aspect_ratio: self.aspect_ratio,
            // Rotation in degrees
            rotation: self.rotation,
            // Padding for alignment
            _padding: [0.0; 3],
        }
    }
}

impl Default for ViewTransform {
    /// Creates a default ViewTransform centered on Oklahoma City.
    fn default() -> Self {
        Self::new(35.4183, -97.4514, 1.0, 16.0 / 9.0)
    }
}

/// Uniform buffer structure for view transformation.
///
/// This struct is designed to be uploaded directly to a wgpu uniform buffer.
/// The memory layout is padded to 16-byte alignment for wgpu compatibility.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct ViewUniforms {
    /// Center X position in NDC.
    pub center_x: f32,
    /// Center Y position in NDC.
    pub center_y: f32,
    /// Scale factor for converting geographic units to NDC.
    pub scale: f32,
    /// Aspect ratio for correction.
    pub aspect_ratio: f32,
    /// Rotation angle in degrees.
    pub rotation: f32,
    /// Padding for 16-byte alignment.
    #[doc(hidden)]
    pub _padding: [f32; 3],
}

impl ViewUniforms {
    /// Returns the size of this uniform buffer in bytes.
    pub const fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test creating a ViewTransform with default values.
    #[test]
    fn test_view_transform_default() {
        let view = ViewTransform::default();

        // Check default center (Oklahoma City)
        assert!((view.center_lat() - 35.4183).abs() < 0.001);
        assert!((view.center_lon() - (-97.4514)).abs() < 0.001);

        // Check default zoom
        assert!((view.zoom() - 1.0).abs() < 0.001);

        // Check default aspect ratio
        assert!((view.aspect_ratio() - (16.0 / 9.0)).abs() < 0.001);

        // Check default rotation
        assert!((view.rotation() - 0.0).abs() < 0.001);
    }

    /// Test that center point converts to origin in clip space.
    #[test]
    fn test_center_point_to_clip_space() {
        let view = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);
        let (x, y) = view.to_clip_space(35.4183, -97.4514);

        assert!((x - 0.0).abs() < 0.0001, "x should be 0, got {}", x);
        assert!((y - 0.0).abs() < 0.0001, "y should be 0, got {}", y);
    }

    /// Test zoom level scaling.
    #[test]
    fn test_zoom_scaling() {
        // Create view at zoom 1.0
        let view1 = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);

        // Point 1 degree north of center
        let (x1, y1) = view1.to_clip_space(36.4183, -97.4514);

        // Create view at zoom 2.0 (more zoomed in)
        let view2 = ViewTransform::new(35.4183, -97.4514, 2.0, 1.0);

        // Same geographic point should be at 1/2 the NDC distance (more zoom = smaller values)
        let (x2, y2) = view2.to_clip_space(36.4183, -97.4514);

        // Check that zooming in halves the clip space distance
        assert!(
            (x2 - x1 * 0.5).abs() < 0.001,
            "x2 should be 0.5x x1, got x1={}, x2={}",
            x1,
            x2
        );
        assert!(
            (y2 - y1 * 0.5).abs() < 0.001,
            "y2 should be 0.5x y1, got y1={}, y2={}",
            y1,
            y2
        );
    }

    /// Test aspect ratio correction.
    #[test]
    fn test_aspect_ratio_correction() {
        // Create view with square aspect ratio
        let view_square = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);
        let (x_square, _) = view_square.to_clip_space(35.4183, -96.4514);

        // Create view with 2:1 aspect ratio (wider)
        let view_wide = ViewTransform::new(35.4183, -97.4514, 1.0, 2.0);
        let (x_wide, _) = view_wide.to_clip_space(35.4183, -96.4514);

        // With 2:1 aspect ratio, x should be halved to maintain same visual width
        assert!((x_wide - x_square / 2.0).abs() < 0.001);
    }

    /// Test rotation calculations.
    #[test]
    fn test_rotation() {
        // Create view with no rotation
        let view_no_rot = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);
        let (x_no_rot, y_no_rot) = view_no_rot.to_clip_space(35.4183, -96.4514);

        // Point east of center (positive lon offset) should have positive x
        assert!(x_no_rot > 0.0, "East point should have positive x");
        assert!(y_no_rot.abs() < 0.001, "East point should have near-zero y");

        // Create view with 90 degree rotation
        let mut view_90 = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);
        view_90.set_rotation(90.0);
        let (_x_90, y_90) = view_90.to_clip_space(35.4183, -96.4514);

        // After 90 degree rotation, east should become south (negative y)
        assert!(
            y_90 < 0.0,
            "After 90° rotation, east point should point south"
        );
    }

    /// Test set_rotation method.
    #[test]
    fn test_set_rotation() {
        let mut view = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);

        view.set_rotation(45.0);
        assert!((view.rotation() - 45.0).abs() < 0.001);

        // Test normalization of 405 degrees (45 degrees past 360)
        view.set_rotation(405.0);
        assert!((view.rotation() - 45.0).abs() < 0.001);

        // Test negative rotation -90 should normalize to 270
        view.set_rotation(-90.0);
        assert!((view.rotation() - 270.0).abs() < 0.001);
    }

    /// Test set_zoom method.
    #[test]
    fn test_set_zoom() {
        let mut view = ViewTransform::new(35.4183, -97.4514, 1.0, 1.0);

        view.set_zoom(2.5);
        assert!((view.zoom() - 2.5).abs() < 0.001);

        // Test that negative zoom is clamped
        view.set_zoom(-1.0);
        assert!(
            view.zoom() > 0.0,
            "Zoom should be positive after negative input"
        );
    }

    /// Test to_uniforms conversion.
    #[test]
    fn test_to_uniforms() {
        let view = ViewTransform::new(35.4183, -97.4514, 2.0, 1.5);
        let uniforms = view.to_uniforms();

        // Check uniforms are computed correctly
        assert!((uniforms.scale - 0.5).abs() < 0.001); // 1/zoom = 1/2
        assert!((uniforms.aspect_ratio - 1.5).abs() < 0.001);
        assert!((uniforms.rotation - 0.0).abs() < 0.001);
    }

    /// Test ViewUniforms size is properly aligned.
    #[test]
    fn test_uniforms_alignment() {
        let size = ViewUniforms::size();

        // Should be multiple of 16 bytes for wgpu alignment
        assert_eq!(size % 16, 0, "ViewUniforms should be 16-byte aligned");

        // Should be exactly 5 floats (20 bytes) padded to 32 bytes
        assert_eq!(size, 32, "ViewUniforms should be 32 bytes");
    }

    /// Test that ViewUniforms can be created from ViewTransform.
    #[test]
    fn test_uniforms_from_view_transform() {
        let mut view = ViewTransform::new(40.0, -100.0, 1.5, 1.777); // 16:9 aspect
        view.set_rotation(30.0);

        let uniforms = view.to_uniforms();

        // Verify all fields
        assert_eq!(uniforms.center_x, 0.0);
        assert_eq!(uniforms.center_y, 0.0);
        assert!((uniforms.scale - (1.0 / 1.5)).abs() < 0.001);
        assert!((uniforms.aspect_ratio - 1.777).abs() < 0.001);
        assert!((uniforms.rotation - 30.0).abs() < 0.001);
    }

    /// Test clip space bounds with extreme zoom values.
    #[test]
    fn test_extreme_zoom() {
        let view = ViewTransform::new(35.4183, -97.4514, 0.1, 1.0);

        // With zoom 0.1, 10 degrees should give 10/0.1 = 100 in NDC
        // This is outside the screen (NDC range is -1 to 1)
        let (x, y) = view.to_clip_space(45.4183, -107.4514);

        // Values should be large (100) since zoomed way out
        assert!(
            x.abs() > 1.0,
            "x should be > 1 (outside screen), got {}",
            x.abs()
        );
        assert!(
            y.abs() > 1.0,
            "y should be > 1 (outside screen), got {}",
            y.abs()
        );

        // But with high zoom, points should be within screen
        let view_zoomed = ViewTransform::new(35.4183, -97.4514, 10.0, 1.0);
        let (x_zoom, y_zoom) = view_zoomed.to_clip_space(36.4183, -96.4514);

        // With zoom 10, 1 degree should be 0.1 in NDC
        assert!(
            x_zoom.abs() < 1.0,
            "x should be < 1 with high zoom, got {}",
            x_zoom.abs()
        );
        assert!(
            y_zoom.abs() < 1.0,
            "y should be < 1 with high zoom, got {}",
            y_zoom.abs()
        );
    }
}
