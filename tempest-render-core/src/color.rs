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

/// NEXRAD standard reflectivity (dBZ) color ramp
/// Range: -30 to 70 dBZ
pub fn reflectivity_ramp() -> ColorRamp {
    ColorRamp::new(
        RadarMoment::Reflectivity,
        vec![
            ColorStop { value: -30.0, color: Rgb::new(20, 20, 30) },   // Very light - dark
            ColorStop { value: 5.0, color: Rgb::new(0, 191, 255) },    // Light blue
            ColorStop { value: 10.0, color: Rgb::new(0, 0, 255) },     // Blue
            ColorStop { value: 20.0, color: Rgb::new(0, 255, 255) },   // Cyan
            ColorStop { value: 30.0, color: Rgb::new(0, 255, 0) },     // Green
            ColorStop { value: 40.0, color: Rgb::new(255, 255, 0) },   // Yellow
            ColorStop { value: 50.0, color: Rgb::new(255, 165, 0) },   // Orange
            ColorStop { value: 55.0, color: Rgb::new(255, 0, 0) },     // Red
            ColorStop { value: 60.0, color: Rgb::new(255, 0, 255) },   // Magenta
            ColorStop { value: 70.0, color: Rgb::new(255, 255, 255) }, // White - extreme
        ],
        Rgb::new(20, 20, 30),   // below -30
        Rgb::new(255, 255, 255), // above 70
    )
}

/// Get color for a reflectivity value in dBZ
pub fn reflectivity_color(value: f32) -> Rgb {
    reflectivity_ramp().get_color(value)
}

/// NEXRAD standard velocity color ramp
/// Range: -50 to +50 m/s (toward = blue, away = red)
pub fn velocity_ramp() -> ColorRamp {
    ColorRamp::new(
        RadarMoment::Velocity,
        vec![
            ColorStop { value: -50.0, color: Rgb::new(0, 0, 139) },       // Dark blue - toward
            ColorStop { value: -40.0, color: Rgb::new(0, 0, 255) },        // Blue
            ColorStop { value: -30.0, color: Rgb::new(0, 191, 255) },     // Light blue
            ColorStop { value: -20.0, color: Rgb::new(135, 206, 250) },   // Pale blue
            ColorStop { value: -10.0, color: Rgb::new(240, 248, 255) },   // Very light
            ColorStop { value: 0.0, color: Rgb::new(255, 255, 255) },     // White - zero
            ColorStop { value: 10.0, color: Rgb::new(255, 240, 245) },    // Very light pink
            ColorStop { value: 20.0, color: Rgb::new(255, 182, 193) },    // Pale red
            ColorStop { value: 30.0, color: Rgb::new(255, 0, 0) },         // Light red
            ColorStop { value: 40.0, color: Rgb::new(178, 34, 34) },      // Medium red
            ColorStop { value: 50.0, color: Rgb::new(139, 0, 0) },        // Dark red - away
        ],
        Rgb::new(0, 0, 139),   // below -50
        Rgb::new(139, 0, 0),   // above +50
    )
}

/// Get color for a velocity value in m/s
pub fn velocity_color(value: f32) -> Rgb {
    velocity_ramp().get_color(value)
}

/// NEXRAD standard ZDR (Differential Reflectivity) color ramp
/// Range: -4 to +8 dB (indicates rain vs hail)
pub fn zdr_ramp() -> ColorRamp {
    ColorRamp::new(
        RadarMoment::Zdr,
        vec![
            ColorStop { value: -4.0, color: Rgb::new(0, 0, 139) },         // Dark blue - light rain
            ColorStop { value: -2.0, color: Rgb::new(0, 0, 205) },          // Medium blue
            ColorStop { value: 0.0, color: Rgb::new(0, 128, 0) },           // Green - moderate
            ColorStop { value: 1.0, color: Rgb::new(154, 205, 50) },       // Yellow-green
            ColorStop { value: 2.0, color: Rgb::new(255, 255, 0) },        // Yellow - heavy
            ColorStop { value: 3.0, color: Rgb::new(255, 165, 0) },        // Orange - very heavy
            ColorStop { value: 4.5, color: Rgb::new(255, 0, 0) },          // Red - hail
            ColorStop { value: 6.0, color: Rgb::new(255, 0, 255) },        // Magenta - large hail
            ColorStop { value: 8.0, color: Rgb::new(255, 255, 255) },      // White - giant hail
        ],
        Rgb::new(0, 0, 139),   // below -4
        Rgb::new(255, 255, 255), // above +8
    )
}

/// Get color for a ZDR value in dB
pub fn zdr_color(value: f32) -> Rgb {
    zdr_ramp().get_color(value)
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

    mod reflectivity_tests {
        use super::*;

        #[test]
        fn test_reflectivity_ramp_creates_valid_ramp() {
            let ramp = reflectivity_ramp();
            
            // Check it's for Reflectivity moment
            assert_eq!(ramp.moment, RadarMoment::Reflectivity);
            
            // Check it has 10 stops
            assert_eq!(ramp.stops.len(), 10);
            
            // Check stops are sorted
            for i in 1..ramp.stops.len() {
                assert!(ramp.stops[i].value >= ramp.stops[i - 1].value);
            }
            
            // Check range
            assert_eq!(ramp.stops.first().map(|s| s.value), Some(-30.0));
            assert_eq!(ramp.stops.last().map(|s| s.value), Some(70.0));
        }

        #[test]
        fn test_reflectivity_color_below_minimum() {
            // Below -30 should return below_min_color
            let color = reflectivity_color(-40.0);
            assert_eq!(color, rgb(20, 20, 30));
        }

        #[test]
        fn test_reflectivity_color_at_minimum() {
            // At -30 should return the dark color
            let color = reflectivity_color(-30.0);
            assert_eq!(color, rgb(20, 20, 30));
        }

        #[test]
        fn test_reflectivity_color_light_blue() {
            // At 5.0 should return light blue (#00BFFF = rgb(0, 191, 255))
            let color = reflectivity_color(5.0);
            assert_eq!(color, rgb(0, 191, 255));
        }

        #[test]
        fn test_reflectivity_color_blue() {
            // At 10.0 should return blue
            let color = reflectivity_color(10.0);
            assert_eq!(color, rgb(0, 0, 255));
        }

        #[test]
        fn test_reflectivity_color_cyan() {
            // At 20.0 should return cyan
            let color = reflectivity_color(20.0);
            assert_eq!(color, rgb(0, 255, 255));
        }

        #[test]
        fn test_reflectivity_color_green() {
            // At 30.0 should return green
            let color = reflectivity_color(30.0);
            assert_eq!(color, rgb(0, 255, 0));
        }

        #[test]
        fn test_reflectivity_color_yellow() {
            // At 40.0 should return yellow
            let color = reflectivity_color(40.0);
            assert_eq!(color, rgb(255, 255, 0));
        }

        #[test]
        fn test_reflectivity_color_orange() {
            // At 50.0 should return orange
            let color = reflectivity_color(50.0);
            assert_eq!(color, rgb(255, 165, 0));
        }

        #[test]
        fn test_reflectivity_color_red() {
            // At 55.0 should return red
            let color = reflectivity_color(55.0);
            assert_eq!(color, rgb(255, 0, 0));
        }

        #[test]
        fn test_reflectivity_color_magenta() {
            // At 60.0 should return magenta
            let color = reflectivity_color(60.0);
            assert_eq!(color, rgb(255, 0, 255));
        }

        #[test]
        fn test_reflectivity_color_white() {
            // At 70.0 should return white
            let color = reflectivity_color(70.0);
            assert_eq!(color, rgb(255, 255, 255));
        }

        #[test]
        fn test_reflectivity_color_above_maximum() {
            // Above 70 should return above_max_color (white)
            let color = reflectivity_color(80.0);
            assert_eq!(color, rgb(255, 255, 255));
        }

        #[test]
        fn test_reflectivity_interpolation() {
            // Test interpolation between stops
            // At 15.0 (between 10.0 blue and 20.0 cyan) should be halfway
            let color = reflectivity_color(15.0);
            assert_eq!(color.r, 0);
            assert_eq!(color.g, 128); // 127.5 rounds to 128
            assert_eq!(color.b, 255);
        }

        #[test]
        fn test_reflectivity_interpolation_green_yellow() {
            // At 35.0 (between 30.0 green and 40.0 yellow)
            let color = reflectivity_color(35.0);
            assert_eq!(color.r, 128); // 127.5 rounds to 128
            assert_eq!(color.g, 255);
            assert_eq!(color.b, 0);
        }

        #[test]
        fn test_reflectivity_interpolation_orange_red() {
            // At 52.5 (between 50.0 orange and 55.0 red)
            let color = reflectivity_color(52.5);
            assert_eq!(color.r, 255);
            assert_eq!(color.g, 83); // 82.5 rounds to 83
            assert_eq!(color.b, 0);
        }
    }

    mod velocity_tests {
        use super::*;

        #[test]
        fn test_velocity_ramp_creates_valid_ramp() {
            let ramp = velocity_ramp();
            
            // Check it's for Velocity moment
            assert_eq!(ramp.moment, RadarMoment::Velocity);
            
            // Check it has 11 stops
            assert_eq!(ramp.stops.len(), 11);
            
            // Check stops are sorted
            for i in 1..ramp.stops.len() {
                assert!(ramp.stops[i].value >= ramp.stops[i - 1].value);
            }
            
            // Check range
            assert_eq!(ramp.stops.first().map(|s| s.value), Some(-50.0));
            assert_eq!(ramp.stops.last().map(|s| s.value), Some(50.0));
        }

        #[test]
        fn test_velocity_color_at_minimum_dark_blue() {
            // At -50 should return dark blue (toward radar)
            let color = velocity_color(-50.0);
            assert_eq!(color, rgb(0, 0, 139));
        }

        #[test]
        fn test_velocity_color_at_zero_white() {
            // At 0.0 should return white
            let color = velocity_color(0.0);
            assert_eq!(color, rgb(255, 255, 255));
        }

        #[test]
        fn test_velocity_color_at_maximum_dark_red() {
            // At 50.0 should return dark red (away from radar)
            let color = velocity_color(50.0);
            assert_eq!(color, rgb(139, 0, 0));
        }

        #[test]
        fn test_velocity_color_below_minimum() {
            // Below -50 should return below_min_color (dark blue)
            let color = velocity_color(-60.0);
            assert_eq!(color, rgb(0, 0, 139));
        }

        #[test]
        fn test_velocity_color_above_maximum() {
            // Above 50 should return above_max_color (dark red)
            let color = velocity_color(60.0);
            assert_eq!(color, rgb(139, 0, 0));
        }

        #[test]
        fn test_velocity_color_blue() {
            // At -40 should return blue
            let color = velocity_color(-40.0);
            assert_eq!(color, rgb(0, 0, 255));
        }

        #[test]
        fn test_velocity_color_light_blue() {
            // At -30 should return light blue
            let color = velocity_color(-30.0);
            assert_eq!(color, rgb(0, 191, 255));
        }

        #[test]
        fn test_velocity_color_pale_blue() {
            // At -20 should return pale blue
            let color = velocity_color(-20.0);
            assert_eq!(color, rgb(135, 206, 250));
        }

        #[test]
        fn test_velocity_color_pale_red() {
            // At 20.0 should return pale red
            let color = velocity_color(20.0);
            assert_eq!(color, rgb(255, 182, 193));
        }

        #[test]
        fn test_velocity_color_light_red() {
            // At 30.0 should return light red
            let color = velocity_color(30.0);
            assert_eq!(color, rgb(255, 0, 0));
        }

        #[test]
        fn test_velocity_color_medium_red() {
            // At 40.0 should return medium red
            let color = velocity_color(40.0);
            assert_eq!(color, rgb(178, 34, 34));
        }

        #[test]
        fn test_velocity_interpolation_blue_white() {
            // At -25 (between -30 light blue and -20 pale blue)
            let color = velocity_color(-25.0);
            // Should be halfway between (0, 191, 255) and (135, 206, 250)
            assert_eq!(color.r, 68); // 67.5 rounds to 68
            assert_eq!(color.g, 199); // 198.5 rounds to 199
            assert_eq!(color.b, 253); // 252.5 rounds to 253
        }

        #[test]
        fn test_velocity_interpolation_white_red() {
            // At 25 (between 20 pale red and 30 light red)
            let color = velocity_color(25.0);
            // Should be halfway between (255, 182, 193) and (255, 0, 0)
            assert_eq!(color.r, 255);
            assert_eq!(color.g, 91); // 91 rounds to 91
            assert_eq!(color.b, 97); // 96.5 rounds to 97
        }
    }

    mod zdr_tests {
        use super::*;

        #[test]
        fn test_zdr_ramp_creates_valid_ramp() {
            let ramp = zdr_ramp();
            
            // Check it's for Zdr moment
            assert_eq!(ramp.moment, RadarMoment::Zdr);
            
            // Check it has 9 stops
            assert_eq!(ramp.stops.len(), 9);
            
            // Check stops are sorted
            for i in 1..ramp.stops.len() {
                assert!(ramp.stops[i].value >= ramp.stops[i - 1].value);
            }
            
            // Check range
            assert_eq!(ramp.stops.first().map(|s| s.value), Some(-4.0));
            assert_eq!(ramp.stops.last().map(|s| s.value), Some(8.0));
        }

        #[test]
        fn test_zdr_color_at_minimum_dark_blue() {
            // At -4.0 should return dark blue (light rain)
            let color = zdr_color(-4.0);
            assert_eq!(color, rgb(0, 0, 139));
        }

        #[test]
        fn test_zdr_color_at_zero_green() {
            // At 0.0 should return green (moderate rain)
            let color = zdr_color(0.0);
            assert_eq!(color, rgb(0, 128, 0));
        }

        #[test]
        fn test_zdr_color_at_4_5_red_hail() {
            // At 4.5 should return red (hail indicator)
            let color = zdr_color(4.5);
            assert_eq!(color, rgb(255, 0, 0));
        }

        #[test]
        fn test_zdr_color_at_maximum_white() {
            // At 8.0 should return white (giant hail)
            let color = zdr_color(8.0);
            assert_eq!(color, rgb(255, 255, 255));
        }

        #[test]
        fn test_zdr_color_below_minimum() {
            // Below -4 should return below_min_color (dark blue)
            let color = zdr_color(-10.0);
            assert_eq!(color, rgb(0, 0, 139));
        }

        #[test]
        fn test_zdr_color_above_maximum() {
            // Above 8 should return above_max_color (white)
            let color = zdr_color(10.0);
            assert_eq!(color, rgb(255, 255, 255));
        }

        #[test]
        fn test_zdr_color_medium_blue() {
            // At -2.0 should return medium blue
            let color = zdr_color(-2.0);
            assert_eq!(color, rgb(0, 0, 205));
        }

        #[test]
        fn test_zdr_color_yellow_green() {
            // At 1.0 should return yellow-green
            let color = zdr_color(1.0);
            assert_eq!(color, rgb(154, 205, 50));
        }

        #[test]
        fn test_zdr_color_yellow() {
            // At 2.0 should return yellow (heavy rain)
            let color = zdr_color(2.0);
            assert_eq!(color, rgb(255, 255, 0));
        }

        #[test]
        fn test_zdr_color_orange() {
            // At 3.0 should return orange (very heavy rain/hail)
            let color = zdr_color(3.0);
            assert_eq!(color, rgb(255, 165, 0));
        }

        #[test]
        fn test_zdr_color_magenta() {
            // At 6.0 should return magenta (large hail)
            let color = zdr_color(6.0);
            assert_eq!(color, rgb(255, 0, 255));
        }

        #[test]
        fn test_zdr_interpolation_blue_green() {
            // At -1.0 (between -2.0 medium blue and 0.0 green)
            let color = zdr_color(-1.0);
            // Should interpolate between (0, 0, 205) and (0, 128, 0)
            assert_eq!(color.r, 0);
            assert_eq!(color.g, 64); // 64 rounds to 64
            assert_eq!(color.b, 103); // 102.5 rounds to 103
        }

        #[test]
        fn test_zdr_interpolation_yellow_orange() {
            // At 2.5 (between 2.0 yellow and 3.0 orange)
            let color = zdr_color(2.5);
            // Should interpolate between (255, 255, 0) and (255, 165, 0)
            assert_eq!(color.r, 255);
            assert_eq!(color.g, 210); // 210 rounds to 210
            assert_eq!(color.b, 0);
        }
    }
}
