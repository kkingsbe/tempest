//! Rendering configuration for radar display.
//!
//! This module provides configuration types for controlling radar rendering
//! including display dimensions, rendering thresholds, and visualization options.

use std::fmt;

/// Maximum default radar range in meters (approximately 460km, typical NEXRAD range).
const DEFAULT_MAX_RANGE_METERS: f32 = 460_000.0;

/// Default minimum reflectivity to display in dBZ.
const DEFAULT_REFLECTIVITY_THRESHOLD: f32 = 5.0;

/// Default radar overlay opacity (0.0 = fully transparent, 1.0 = fully opaque).
const DEFAULT_OPACITY: f32 = 0.7;

/// Default sweep rotation angle in degrees.
const DEFAULT_SWEEP_ROTATION: f32 = 0.0;

/// Rendering configuration for radar display.
///
/// This struct holds all configuration parameters needed to render radar data
/// including screen dimensions, display thresholds, and visualization options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderConfig {
    /// Screen/canvas width in pixels.
    pub width: u32,
    /// Screen/canvas height in pixels.
    pub height: u32,
    /// Maximum radar range to render in meters.
    pub max_range_meters: f32,
    /// Minimum dBZ value to display (values below are clipped).
    pub reflectivity_threshold: f32,
    /// Default opacity for radar overlay (0.0 - 1.0).
    pub default_opacity: f32,
    /// Initial sweep rotation angle in degrees.
    pub sweep_rotation: f32,
    /// Whether to render velocity data.
    pub show_velocity: bool,
    /// Whether to render reflectivity data.
    pub show_reflectivity: bool,
}

impl Default for RenderConfig {
    /// Creates a RenderConfig with default values.
    ///
    /// Default configuration:
    /// - 800x600 resolution
    /// - 460km max range
    /// - 5 dBZ threshold
    /// - 0.7 opacity
    /// - 0° sweep rotation
    /// - Both velocity and reflectivity enabled
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            max_range_meters: DEFAULT_MAX_RANGE_METERS,
            reflectivity_threshold: DEFAULT_REFLECTIVITY_THRESHOLD,
            default_opacity: DEFAULT_OPACITY,
            sweep_rotation: DEFAULT_SWEEP_ROTATION,
            show_velocity: true,
            show_reflectivity: true,
        }
    }
}

impl RenderConfig {
    /// Creates a new RenderConfig with the specified dimensions and default values.
    ///
    /// # Arguments
    ///
    /// * `width` - Screen width in pixels
    /// * `height` - Screen height in pixels
    ///
    /// # Returns
    ///
    /// A new RenderConfig with default values for all other fields.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RenderConfig;
    /// let config = RenderConfig::new(1920, 1080);
    /// assert_eq!(config.width, 1920);
    /// assert_eq!(config.height, 1080);
    /// assert!(config.show_reflectivity);
    /// ```
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            max_range_meters: DEFAULT_MAX_RANGE_METERS,
            reflectivity_threshold: DEFAULT_REFLECTIVITY_THRESHOLD,
            default_opacity: DEFAULT_OPACITY,
            sweep_rotation: DEFAULT_SWEEP_ROTATION,
            show_velocity: true,
            show_reflectivity: true,
        }
    }

    /// Calculates the aspect ratio of the render surface.
    ///
    /// # Returns
    ///
    /// The aspect ratio (width / height) as a f32.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RenderConfig;
    /// let config = RenderConfig::new(1920, 1080);
    /// assert!((config.aspect_ratio() - 1.777_f32).abs() < 0.001);
    /// ```
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            return 1.0;
        }
        self.width as f32 / self.height as f32
    }

    /// Creates a RenderConfig with custom max range.
    ///
    /// # Arguments
    ///
    /// * `width` - Screen width in pixels
    /// * `height` - Screen height in pixels
    /// * `max_range_meters` - Maximum radar range in meters
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RenderConfig;
    /// let config = RenderConfig::with_range(800, 600, 230_000.0);
    /// assert_eq!(config.max_range_meters, 230_000.0);
    /// ```
    #[must_use]
    pub fn with_range(width: u32, height: u32, max_range_meters: f32) -> Self {
        Self {
            width,
            height,
            max_range_meters,
            reflectivity_threshold: DEFAULT_REFLECTIVITY_THRESHOLD,
            default_opacity: DEFAULT_OPACITY,
            sweep_rotation: DEFAULT_SWEEP_ROTATION,
            show_velocity: true,
            show_reflectivity: true,
        }
    }

    /// Creates a RenderConfig with custom opacity.
    ///
    /// # Arguments
    ///
    /// * `width` - Screen width in pixels
    /// * `height` - Screen height in pixels
    /// * `opacity` - Radar overlay opacity (0.0 - 1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RenderConfig;
    /// let config = RenderConfig::with_opacity(800, 600, 0.5);
    /// assert_eq!(config.default_opacity, 0.5);
    /// ```
    #[must_use]
    pub fn with_opacity(width: u32, height: u32, opacity: f32) -> Self {
        Self {
            width,
            height,
            max_range_meters: DEFAULT_MAX_RANGE_METERS,
            reflectivity_threshold: DEFAULT_REFLECTIVITY_THRESHOLD,
            default_opacity: opacity.clamp(0.0, 1.0),
            sweep_rotation: DEFAULT_SWEEP_ROTATION,
            show_velocity: true,
            show_reflectivity: true,
        }
    }
}

impl fmt::Display for RenderConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RenderConfig {}x{} (aspect: {:.2}, range: {}km, threshold: {}dBZ, opacity: {:.2})",
            self.width,
            self.height,
            self.aspect_ratio(),
            self.max_range_meters / 1000.0,
            self.reflectivity_threshold,
            self.default_opacity
        )
    }
}

/// Radar display style/visualization mode.
///
/// This enum defines different ways to visualize radar data,
/// each showing different properties of the weather data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RadarStyle {
    /// Show reflectivity (dBZ) - measure of precipitation intensity.
    #[default]
    Reflectivity,
    /// Show radial velocity - motion toward/away from radar.
    Velocity,
    /// Show spectrum width - turbulence/intensity variance.
    SpectrumWidth,
    /// Show differential reflectivity (Zdr) - hail detection/particle shape.
    Zdr,
    /// Show correlation coefficient (Cc) - precipitation type identification.
    Cc,
    /// Show differential phase (Kdp) - rain intensity estimation.
    Kdp,
    /// Show combined/composite view of multiple data types.
    Composite,
}

impl RadarStyle {
    /// Returns the display name for this radar style.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RadarStyle;
    /// assert_eq!(RadarStyle::Reflectivity.display_name(), "Reflectivity");
    /// assert_eq!(RadarStyle::Velocity.display_name(), "Velocity");
    /// ```
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            RadarStyle::Reflectivity => "Reflectivity",
            RadarStyle::Velocity => "Velocity",
            RadarStyle::SpectrumWidth => "Spectrum Width",
            RadarStyle::Zdr => "Differential Reflectivity",
            RadarStyle::Cc => "Correlation Coefficient",
            RadarStyle::Kdp => "Differential Phase",
            RadarStyle::Composite => "Composite",
        }
    }

    /// Returns true if this style requires velocity data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RadarStyle;
    /// assert!(!RadarStyle::Reflectivity.requires_velocity());
    /// assert!(RadarStyle::Velocity.requires_velocity());
    /// ```
    #[must_use]
    pub fn requires_velocity(&self) -> bool {
        matches!(self, RadarStyle::Velocity | RadarStyle::SpectrumWidth)
    }

    /// Returns true if this style requires reflectivity data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_render::config::RadarStyle;
    /// assert!(RadarStyle::Reflectivity.requires_reflectivity());
    /// assert!(!RadarStyle::Velocity.requires_reflectivity());
    /// ```
    #[must_use]
    pub fn requires_reflectivity(&self) -> bool {
        matches!(self, RadarStyle::Reflectivity | RadarStyle::Composite)
    }
}

impl fmt::Display for RadarStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests for RenderConfig Default implementation.
    mod render_config_default {
        use super::*;

        #[test]
        fn test_default_values() {
            let config = RenderConfig::default();

            assert_eq!(config.width, 800);
            assert_eq!(config.height, 600);
            assert!((config.max_range_meters - 460_000.0).abs() < 1.0);
            assert!((config.reflectivity_threshold - 5.0).abs() < 0.001);
            assert!((config.default_opacity - 0.7).abs() < 0.001);
            assert!((config.sweep_rotation - 0.0).abs() < 0.001);
            assert!(config.show_velocity);
            assert!(config.show_reflectivity);
        }

        #[test]
        fn test_default_is_singleton() {
            let config1 = RenderConfig::default();
            let config2 = RenderConfig::default();
            assert_eq!(config1, config2);
        }
    }

    /// Tests for RenderConfig::new implementation.
    mod render_config_new {
        use super::*;

        #[test]
        fn test_new_with_dimensions() {
            let config = RenderConfig::new(1920, 1080);

            assert_eq!(config.width, 1920);
            assert_eq!(config.height, 1080);
            assert!((config.max_range_meters - 460_000.0).abs() < 1.0);
            assert!((config.reflectivity_threshold - 5.0).abs() < 0.001);
            assert!((config.default_opacity - 0.7).abs() < 0.001);
        }

        #[test]
        fn test_new_preserves_defaults() {
            let config = RenderConfig::new(1024, 768);

            assert!(config.show_velocity);
            assert!(config.show_reflectivity);
            assert!((config.sweep_rotation - 0.0).abs() < 0.001);
        }
    }

    /// Tests for aspect_ratio calculation.
    mod aspect_ratio {
        use super::*;

        #[test]
        fn test_standard_16_9() {
            let config = RenderConfig::new(1920, 1080);
            let ratio = config.aspect_ratio();
            // 1920/1080 = 1.777... (16:9 aspect ratio)
            assert!((ratio - 1.7777777).abs() < 0.001);
        }

        #[test]
        fn test_standard_4_3() {
            let config = RenderConfig::new(800, 600);
            let ratio = config.aspect_ratio();
            // 800/600 = 1.333... (4:3 aspect ratio)
            assert!((ratio - 1.3333333).abs() < 0.001);
        }

        #[test]
        fn test_square() {
            let config = RenderConfig::new(512, 512);
            assert!((config.aspect_ratio() - 1.0).abs() < 0.001);
        }

        #[test]
        fn test_zero_height_returns_one() {
            let config = RenderConfig {
                width: 800,
                height: 0,
                ..Default::default()
            };
            assert!((config.aspect_ratio() - 1.0).abs() < 0.001);
        }
    }

    /// Tests for RenderConfig builder methods.
    mod render_config_builder {
        use super::*;

        #[test]
        fn test_with_range() {
            let config = RenderConfig::with_range(800, 600, 230_000.0);
            assert!((config.max_range_meters - 230_000.0).abs() < 1.0);
            assert_eq!(config.width, 800);
            assert_eq!(config.height, 600);
        }

        #[test]
        fn test_with_opacity() {
            let config = RenderConfig::with_opacity(800, 600, 0.5);
            assert!((config.default_opacity - 0.5).abs() < 0.001);
        }

        #[test]
        fn test_with_opacity_clamped() {
            let config = RenderConfig::with_opacity(800, 600, 1.5);
            assert!((config.default_opacity - 1.0).abs() < 0.001);

            let config2 = RenderConfig::with_opacity(800, 600, -0.5);
            assert!((config2.default_opacity - 0.0).abs() < 0.001);
        }
    }

    /// Tests for RenderConfig Display implementation.
    mod render_config_display {
        use super::*;

        #[test]
        fn test_display_format() {
            let config = RenderConfig::default();
            let display = format!("{}", config);

            assert!(display.contains("RenderConfig"));
            assert!(display.contains("800x600"));
            assert!(display.contains("aspect:"));
            assert!(display.contains("range:"));
            assert!(display.contains("threshold:"));
            assert!(display.contains("opacity:"));
        }
    }

    /// Tests for RadarStyle enum.
    mod radar_style {
        use super::*;

        #[test]
        fn test_default_is_reflectivity() {
            assert_eq!(RadarStyle::default(), RadarStyle::Reflectivity);
        }

        #[test]
        fn test_display_names() {
            assert_eq!(RadarStyle::Reflectivity.display_name(), "Reflectivity");
            assert_eq!(RadarStyle::Velocity.display_name(), "Velocity");
            assert_eq!(RadarStyle::SpectrumWidth.display_name(), "Spectrum Width");
            assert_eq!(RadarStyle::Zdr.display_name(), "Differential Reflectivity");
            assert_eq!(RadarStyle::Cc.display_name(), "Correlation Coefficient");
            assert_eq!(RadarStyle::Kdp.display_name(), "Differential Phase");
            assert_eq!(RadarStyle::Composite.display_name(), "Composite");
        }

        #[test]
        fn test_requires_velocity() {
            assert!(!RadarStyle::Reflectivity.requires_velocity());
            assert!(RadarStyle::Velocity.requires_velocity());
            assert!(RadarStyle::SpectrumWidth.requires_velocity());
            assert!(!RadarStyle::Zdr.requires_velocity());
            assert!(!RadarStyle::Cc.requires_velocity());
            assert!(!RadarStyle::Kdp.requires_velocity());
            assert!(!RadarStyle::Composite.requires_velocity());
        }

        #[test]
        fn test_requires_reflectivity() {
            assert!(RadarStyle::Reflectivity.requires_reflectivity());
            assert!(!RadarStyle::Velocity.requires_reflectivity());
            assert!(!RadarStyle::SpectrumWidth.requires_reflectivity());
            assert!(!RadarStyle::Zdr.requires_reflectivity());
            assert!(!RadarStyle::Cc.requires_reflectivity());
            assert!(!RadarStyle::Kdp.requires_reflectivity());
            assert!(RadarStyle::Composite.requires_reflectivity());
        }

        #[test]
        fn test_display_trait() {
            assert_eq!(format!("{}", RadarStyle::Reflectivity), "Reflectivity");
            assert_eq!(format!("{}", RadarStyle::Velocity), "Velocity");
            assert_eq!(format!("{}", RadarStyle::SpectrumWidth), "Spectrum Width");
            assert_eq!(format!("{}", RadarStyle::Zdr), "Differential Reflectivity");
            assert_eq!(format!("{}", RadarStyle::Cc), "Correlation Coefficient");
            assert_eq!(format!("{}", RadarStyle::Kdp), "Differential Phase");
            assert_eq!(format!("{}", RadarStyle::Composite), "Composite");
        }
    }

    /// Tests for RadarStyle equality.
    mod radar_style_equality {
        use super::*;

        #[test]
        fn test_copy_is_equal() {
            let style1 = RadarStyle::Reflectivity;
            let style2 = RadarStyle::Reflectivity;
            assert_eq!(style1, style2);
        }

        #[test]
        fn test_different_styles_not_equal() {
            assert_ne!(RadarStyle::Reflectivity, RadarStyle::Velocity);
            assert_ne!(RadarStyle::Velocity, RadarStyle::SpectrumWidth);
            assert_ne!(RadarStyle::SpectrumWidth, RadarStyle::Zdr);
            assert_ne!(RadarStyle::Zdr, RadarStyle::Cc);
            assert_ne!(RadarStyle::Cc, RadarStyle::Kdp);
            assert_ne!(RadarStyle::Kdp, RadarStyle::Composite);
        }
    }
}
