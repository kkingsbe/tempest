//! Color types and color ramps for radar visualization.
//!
//! This module provides RGB/RGBA color representations and color ramps
//! for mapping radar values to colors.

use std::fmt;

/// RGB color with u8 components (0-255)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
}

impl Rgb {
    /// Create a new RGB color.
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    ///
    /// # Examples
    ///
    /// ```
    /// use tempest_render_core::color::Rgb;
    ///
    /// let red = Rgb::new(255, 0, 0);
    /// assert_eq!(red.r, 255);
    /// assert_eq!(red.g, 0);
    /// assert_eq!(red.b, 0);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Convert to RGBA with full opacity (alpha = 255).
    pub fn to_rgba(self) -> super::Rgba {
        super::Rgba::new(self.r, self.g, self.b, 255)
    }

    /// Convert to RGBA with custom alpha.
    pub fn to_rgba_with_alpha(self, a: u8) -> super::Rgba {
        super::Rgba::new(self.r, self.g, self.b, a)
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

/// RGBA color with u8 components.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    /// Red component (0-255)
    pub r: u8,
    /// Green component (0-255)
    pub g: u8,
    /// Blue component (0-255)
    pub b: u8,
    /// Alpha component (0-255)
    pub a: u8,
}

impl Rgba {
    /// Create a new RGBA color.
    ///
    /// # Arguments
    /// * `r` - Red component (0-255)
    /// * `g` - Green component (0-255)
    /// * `b` - Blue component (0-255)
    /// * `a` - Alpha component (0-255)
    ///
    /// # Examples
    ///
    /// ```
    /// use tempest_render_core::color::Rgba;
    ///
    /// let red = Rgba::new(255, 0, 0, 255);
    /// assert_eq!(red.r, 255);
    /// assert_eq!(red.a, 255);
    /// ```
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create a fully transparent RGBA color.
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    /// Create a fully opaque RGBA color.
    pub const OPAQUE: Self = Self::new(0, 0, 0, 255);
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

/// Radar moment types that can be rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RadarMoment {
    /// Reflectivity (dBZ) - precipitation intensity
    Reflectivity,
    /// Velocity (m/s) - radial wind speed
    Velocity,
    /// Spectrum Width (m/s) - turbulence
    SpectrumWidth,
    /// Differential Reflectivity (dB) - rain vs hail
    Zdr,
    /// Correlation Coefficient (0-1) - particle shape
    Cc,
    /// Differential Phase (degrees/km) - rain rate
    Kdp,
}

impl RadarMoment {
    /// Get the two-character NEXRAD moment code.
    pub fn code(&self) -> &'static str {
        match self {
            RadarMoment::Reflectivity => "REF",
            RadarMoment::Velocity => "VEL",
            RadarMoment::SpectrumWidth => "SW",
            RadarMoment::Zdr => "ZDR",
            RadarMoment::Cc => "CC",
            RadarMoment::Kdp => "KDP",
        }
    }
}

/// A color stop defines a value and its corresponding color.
///
/// Color stops are used in color ramps to define the mapping
/// between radar values and colors.
#[derive(Debug, Clone, Copy)]
pub struct ColorStop {
    /// The radar value at this stop.
    pub value: f32,
    /// The color for this value.
    pub color: Rgb,
}

impl ColorStop {
    /// Create a new color stop.
    pub const fn new(value: f32, color: Rgb) -> Self {
        Self { value, color }
    }
}

/// A color ramp is a sequence of color stops for mapping values to colors.
///
/// Color ramps interpolate between stops to provide smooth color transitions.
#[derive(Debug, Clone)]
pub struct ColorRamp {
    /// The radar moment this color ramp is for.
    pub moment: RadarMoment,
    /// Color stops sorted by value (ascending).
    stops: Vec<ColorStop>,
    /// Color for values below the minimum stop.
    pub below_min_color: Rgb,
    /// Color for values above the maximum stop.
    pub above_max_color: Rgb,
}

impl ColorRamp {
    /// Create a new color ramp.
    ///
    /// The stops should be sorted by value in ascending order.
    ///
    /// # Arguments
    /// * `moment` - The radar moment type
    /// * `stops` - Color stops sorted by value (ascending)
    /// * `below_min_color` - Color for values below the minimum stop
    /// * `above_max_color` - Color for values above the maximum stop
    ///
    /// # Examples
    ///
    /// ```
    /// use tempest_render_core::color::{ColorRamp, ColorStop, RadarMoment, Rgb};
    ///
    /// let stops = vec![
    ///     ColorStop::new(0.0, Rgb::new(0, 0, 255)),
    ///     ColorStop::new(50.0, Rgb::new(255, 0, 0)),
    /// ];
    /// let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, Rgb::new(0, 0, 0), Rgb::new(255, 255, 255));
    /// ```
    pub fn new(
        moment: RadarMoment,
        stops: Vec<ColorStop>,
        below_min_color: Rgb,
        above_max_color: Rgb,
    ) -> Self {
        Self {
            moment,
            stops,
            below_min_color,
            above_max_color,
        }
    }

    /// Get the color for a given radar value.
    ///
    /// Uses linear interpolation between color stops.
    ///
    /// # Arguments
    /// * `value` - The radar value to get the color for
    ///
    /// # Returns
    /// The interpolated RGB color.
    pub fn get_color(&self, value: f32) -> Rgb {
        // Handle empty stops
        if self.stops.is_empty() {
            return self.below_min_color;
        }

        // Get the minimum and maximum values
        let min_value = self.stops.first().map(|s| s.value).expect("stops not empty");
        let max_value = self.stops.last().map(|s| s.value).expect("stops not empty");

        // Handle below minimum
        if value < min_value {
            return self.below_min_color;
        }

        // Handle above maximum
        if value > max_value {
            return self.above_max_color;
        }

        // Handle exact match on first stop
        if (value - min_value).abs() < f32::EPSILON {
            return self.stops[0].color;
        }

        // Handle exact match on last stop
        if (value - max_value).abs() < f32::EPSILON {
            return self.stops[self.stops.len() - 1].color;
        }

        // Handle single stop
        if self.stops.len() == 1 {
            return self.stops[0].color;
        }

        // Find the two stops to interpolate between
        let mut lower_idx = 0;
        for (i, stop) in self.stops.iter().enumerate() {
            if value < stop.value {
                break;
            }
            lower_idx = i;
        }

        // Get the upper stop (but not if we're at the last stop)
        let upper_idx = if lower_idx < self.stops.len() - 1 {
            lower_idx + 1
        } else {
            lower_idx
        };

        let lower_stop = &self.stops[lower_idx];
        let upper_stop = &self.stops[upper_idx];

        // Avoid division by zero if stops have the same value
        if (upper_stop.value - lower_stop.value).abs() < f32::EPSILON {
            return lower_stop.color;
        }

        // Calculate interpolation factor
        let t = (value - lower_stop.value) / (upper_stop.value - lower_stop.value);
        let t = t.clamp(0.0, 1.0);

        // Interpolate color components using round-nearest
        let r = ((lower_stop.color.r as f32
            + (upper_stop.color.r as f32 - lower_stop.color.r as f32) * t)
            .round()) as u8;
        let g = ((lower_stop.color.g as f32
            + (upper_stop.color.g as f32 - lower_stop.color.g as f32) * t)
            .round()) as u8;
        let b = ((lower_stop.color.b as f32
            + (upper_stop.color.b as f32 - lower_stop.color.b as f32) * t)
            .round()) as u8;

        Rgb::new(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create RGB color.
    fn rgb(r: u8, g: u8, b: u8) -> Rgb {
        Rgb::new(r, g, b)
    }

    /// Helper to create ColorStop.
    fn stop(value: f32, r: u8, g: u8, b: u8) -> ColorStop {
        ColorStop::new(value, rgb(r, g, b))
    }

    mod rgb_tests {
        use super::*;

        #[test]
        fn test_rgb_new() {
            let color = Rgb::new(255, 128, 64);
            assert_eq!(color.r, 255);
            assert_eq!(color.g, 128);
            assert_eq!(color.b, 64);
        }

        #[test]
        fn test_rgb_display() {
            let color = Rgb::new(255, 128, 64);
            assert_eq!(format!("{}", color), "rgb(255, 128, 64)");
        }

        #[test]
        fn test_rgb_to_rgba() {
            let color = Rgb::new(255, 128, 64);
            let rgba = color.to_rgba();
            assert_eq!(rgba.r, 255);
            assert_eq!(rgba.g, 128);
            assert_eq!(rgba.b, 64);
            assert_eq!(rgba.a, 255);
        }

        #[test]
        fn test_rgb_to_rgba_with_alpha() {
            let color = Rgb::new(255, 128, 64);
            let rgba = color.to_rgba_with_alpha(128);
            assert_eq!(rgba.a, 128);
        }
    }

    mod color_interpolation_tests {
        use super::*;

        #[test]
        fn test_exact_stop_value() {
            let stops = vec![stop(0.0, 0, 0, 255), stop(50.0, 255, 0, 0)];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 0), rgb(255, 255, 255));

            // Exact match on first stop
            let color = ramp.get_color(0.0);
            assert_eq!(color, rgb(0, 0, 255));

            // Exact match on second stop
            let color = ramp.get_color(50.0);
            assert_eq!(color, rgb(255, 0, 0));
        }

        #[test]
        fn test_interpolation_between_stops() {
            let stops = vec![stop(0.0, 0, 0, 255), stop(100.0, 255, 0, 0)];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 0), rgb(255, 255, 255));

            // Middle value should be exactly in the middle (with rounding)
            let color = ramp.get_color(50.0);
            assert_eq!(color.r, 128); // 127.5 rounds to 128
            assert_eq!(color.g, 0);
            assert_eq!(color.b, 128); // 127.5 rounds to 128
        }

        #[test]
        fn test_below_minimum() {
            let stops = vec![stop(10.0, 0, 255, 0), stop(50.0, 255, 0, 0)];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 255), rgb(255, 255, 255));

            let color = ramp.get_color(0.0);
            assert_eq!(color, rgb(0, 0, 255)); // below_min_color
        }

        #[test]
        fn test_above_maximum() {
            let stops = vec![stop(10.0, 0, 255, 0), stop(50.0, 255, 0, 0)];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 0), rgb(255, 255, 255));

            let color = ramp.get_color(100.0);
            assert_eq!(color, rgb(255, 255, 255)); // above_max_color
        }

        #[test]
        fn test_empty_stops() {
            let stops: Vec<ColorStop> = vec![];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 255), rgb(255, 0, 0));

            let color = ramp.get_color(50.0);
            assert_eq!(color, rgb(0, 0, 255)); // below_min_color for empty
        }

        #[test]
        fn test_single_stop() {
            let stops = vec![stop(50.0, 128, 128, 128)];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 255), rgb(255, 0, 0));

            let color = ramp.get_color(50.0);
            assert_eq!(color, rgb(128, 128, 128));
        }

        #[test]
        fn test_multiple_stops_interpolation() {
            // Three-stop ramp: blue -> green -> red
            let stops = vec![
                stop(0.0, 0, 0, 255),
                stop(50.0, 0, 255, 0),
                stop(100.0, 255, 0, 0),
            ];
            let ramp = ColorRamp::new(RadarMoment::Reflectivity, stops, rgb(0, 0, 0), rgb(255, 255, 255));

            // At 25.0, should be halfway between blue and green (with rounding)
            let color = ramp.get_color(25.0);
            assert_eq!(color.r, 0);
            assert_eq!(color.g, 128); // 127.5 rounds to 128
            assert_eq!(color.b, 128); // 127.5 rounds to 128

            // At 75.0, should be halfway between green and red (with rounding)
            let color = ramp.get_color(75.0);
            assert_eq!(color.r, 128); // 127.5 rounds to 128
            assert_eq!(color.g, 128); // 127.5 rounds to 128
            assert_eq!(color.b, 0);
        }
    }
}
