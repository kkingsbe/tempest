//! Color Legend UI Component
//!
//! A widget for displaying a visual color legend for radar data.
//! Shows a vertical color bar with labels indicating the value range
//! and units for the currently selected radar moment.

use crate::spacing;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};

use tempest_render_core::color::{
    cc_ramp, kdp_ramp, reflectivity_ramp, spectrum_width_ramp, velocity_ramp, zdr_ramp, ColorRamp,
    RadarMoment,
};

/// Messages produced by the ColorLegend component
#[derive(Debug, Clone, Copy)]
pub enum ColorLegendMessage {
    /// Update the current radar moment
    SetMoment(RadarMoment),
}

/// A component for displaying a color legend for radar visualization
///
/// Displays a vertical color gradient bar with min/max labels and unit indicator
/// for the currently selected radar moment (Reflectivity, Velocity, etc.)
#[derive(Debug, Clone)]
pub struct ColorLegend {
    /// Currently displayed radar moment
    current_moment: RadarMoment,
}

impl Default for ColorLegend {
    fn default() -> Self {
        Self::new()
    }
}

impl ColorLegend {
    /// Creates a new ColorLegend with default moment (Reflectivity)
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            current_moment: RadarMoment::Reflectivity,
        }
    }

    /// Creates a new ColorLegend with the specified radar moment
    #[allow(dead_code)]
    pub fn with_moment(moment: RadarMoment) -> Self {
        Self {
            current_moment: moment,
        }
    }

    /// Returns the currently displayed radar moment
    #[allow(dead_code)]
    #[must_use]
    pub fn current_moment(&self) -> RadarMoment {
        self.current_moment
    }

    /// Sets the radar moment to display
    #[allow(dead_code)]
    pub fn set_moment(&mut self, moment: RadarMoment) {
        self.current_moment = moment;
    }

    /// Updates the component with a new message
    #[allow(dead_code)]
    pub fn update(&mut self, message: ColorLegendMessage) {
        let ColorLegendMessage::SetMoment(moment) = message;
        self.current_moment = moment;
    }

    /// Get the color ramp for the current moment
    #[must_use]
    fn get_color_ramp(&self) -> ColorRamp {
        match self.current_moment {
            RadarMoment::Reflectivity => reflectivity_ramp(),
            RadarMoment::Velocity => velocity_ramp(),
            RadarMoment::SpectrumWidth => spectrum_width_ramp(),
            RadarMoment::Zdr => zdr_ramp(),
            RadarMoment::Cc => cc_ramp(),
            RadarMoment::Kdp => kdp_ramp(),
        }
    }

    /// Get the display name and unit for the current moment
    #[must_use]
    fn get_label(&self) -> (String, String, f32, f32) {
        match self.current_moment {
            RadarMoment::Reflectivity => {
                ("Reflectivity".to_string(), "dBZ".to_string(), -30.0, 70.0)
            }
            RadarMoment::Velocity => ("Velocity".to_string(), "m/s".to_string(), -50.0, 50.0),
            RadarMoment::SpectrumWidth => {
                ("Spectrum Width".to_string(), "m/s".to_string(), 0.0, 30.0)
            }
            RadarMoment::Zdr => ("ZDR".to_string(), "dB".to_string(), -4.0, 8.0),
            RadarMoment::Cc => (
                "Correlation Coefficient".to_string(),
                "".to_string(),
                0.0,
                1.0,
            ),
            RadarMoment::Kdp => ("KDP".to_string(), "deg/km".to_string(), -2.0, 10.0),
        }
    }

    /// Returns the view for this component
    pub fn view(&self) -> Element<'_, ColorLegendMessage> {
        let (name, unit, min_val, max_val) = self.get_label();
        let ramp = self.get_color_ramp();

        // Build the title row
        let title = if unit.is_empty() {
            text(name.clone()).size(14)
        } else {
            text(format!("{} ({})", name, unit)).size(14)
        };

        // Create the color bar using colored rectangles
        // We'll create a vertical stack of color swatches
        let num_stops = 20; // Number of color swatches in the gradient
        let step = (max_val - min_val) / (num_stops as f32);

        let mut color_bar = column!().spacing(spacing::XXS).width(Length::Fixed(30.0));

        // Add color swatches from top (max) to bottom (min)
        for i in 0..num_stops {
            let value = max_val - (i as f32 * step);
            let color = ramp.get_color(value);
            let iced_color = iced::Color::from_rgb(
                color.r as f32 / 255.0,
                color.g as f32 / 255.0,
                color.b as f32 / 255.0,
            );

            // Create color swatch with the actual radar color
            let swatch = container(iced::widget::Space::new(
                Length::Fixed(30.0),
                Length::Fixed(6.0),
            ))
            .style(move |_theme| container::Style {
                background: Some(iced::Background::Color(iced_color)),
                ..Default::default()
            });

            color_bar = color_bar.push(swatch);
        }

        // Build min/max labels
        let min_label = text(format!("{:.0}", min_val)).size(11);
        let max_label = text(format!("{:.0}", max_val)).size(11);

        // Create the legend content
        let content = column![
            title,
            row![
                color_bar,
                column![
                    iced::widget::Space::new(Length::Fill, Length::Fixed(90.0)),
                    max_label,
                ]
            ]
            .spacing(spacing::XS),
            min_label,
        ]
        .spacing(spacing::SM)
        .padding(spacing::MD);

        container(content).padding(spacing::BASE).into()
    }
}
