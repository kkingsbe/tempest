//! Elevation Tilt Selector UI Component
//!
//! A widget for selecting elevation tilts from a NEXRAD volume scan.
//! Displays available elevation angles as buttons for user selection.

use iced::widget::{button, row, text};
use iced::{Element, Length, Theme};


/// Messages produced by the ElevationTiltSelector component
#[derive(Debug, Clone, Copy)]
pub enum ElevationTiltSelectorMessage {
    /// An elevation angle was selected
    SelectElevation(f32),
}

/// A component for selecting elevation tilts from a radar volume scan
///
/// Displays a horizontal row of buttons representing available elevation angles
/// from the volume scan, allowing users to switch between different tilt angles.
#[derive(Debug, Clone)]
pub struct ElevationTiltSelector {
    /// Available elevation angles from the volume scan (in degrees)
    available_elevations: Vec<f32>,
    /// Currently selected elevation angle
    selected_elevation: Option<f32>,
}

impl Default for ElevationTiltSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl ElevationTiltSelector {
    /// Creates a new ElevationTiltSelector with no elevations available
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            available_elevations: Vec::new(),
            selected_elevation: None,
        }
    }

    /// Creates a new ElevationTiltSelector with the given elevation angles
    #[allow(dead_code)]
    pub fn with_elevations(elevations: Vec<f32>) -> Self {
        let selected_elevation = elevations.first().copied();
        Self {
            available_elevations: elevations,
            selected_elevation,
        }
    }

    /// Returns the currently selected elevation angle
    #[allow(dead_code)]
    #[must_use]
    pub fn selected_elevation(&self) -> Option<f32> {
        self.selected_elevation
    }

    /// Returns a reference to the available elevation angles
    #[allow(dead_code)]
    #[must_use]
    pub fn elevations(&self) -> &[f32] {
        &self.available_elevations
    }

    /// Sets the selected elevation angle
    #[allow(dead_code)]
    pub fn set_elevation(&mut self, elevation: f32) {
        // Only set if the elevation is in the available list
        if self.available_elevations.contains(&elevation) {
            self.selected_elevation = Some(elevation);
        }
    }

    /// Updates the available elevations and optionally selects the first one
    #[allow(dead_code)]
    pub fn set_elevations(&mut self, elevations: Vec<f32>) {
        self.available_elevations = elevations;
        // Auto-select first elevation if none selected or current selection not available
        match self.selected_elevation {
            Some(selected) if self.available_elevations.contains(&selected) => {
                // Keep current selection
            }
            _ => {
                // Select first available or none
                self.selected_elevation = self.available_elevations.first().copied();
            }
        }
    }

    /// Handles incoming messages
    pub fn update(&mut self, message: ElevationTiltSelectorMessage) {
        let ElevationTiltSelectorMessage::SelectElevation(elevation) = message;
        // Only update if the elevation is available
        if self.available_elevations.contains(&elevation) {
            self.selected_elevation = Some(elevation);
        }
    }

    /// Returns the view for this component
    pub fn view(&self, theme: &Theme) -> Element<'_, ElevationTiltSelectorMessage> {
        let _p = theme.extended_palette();

        // Styling constants using semantic colors from theme
        // Note: Using direct colors - styling simplified for iced 0.13 compatibility

        // Show placeholder if no elevations available
        if self.available_elevations.is_empty() {
            let content = row![
                text("Elevation Tilt").size(16),
                text("No elevation data available")
                    .size(14),
            ]
            .spacing(12)
            .align_y(iced::Alignment::Center);

            return content.into();
        }

        // Build row of elevation buttons
        let mut elevation_buttons = row!().spacing(8);

        for &elevation in &self.available_elevations {
            let is_selected = self
                .selected_elevation
                .is_some_and(|e| (e - elevation).abs() < 0.001);

            // Format elevation as "0.5°", "1.5°", etc.
            let label = if elevation == elevation.floor() {
                format!("{:.0}°", elevation)
            } else {
                format!("{:.1}°", elevation)
            };

            let btn =
                if is_selected {
                    button(text(label).size(14))
                    .on_press(ElevationTiltSelectorMessage::SelectElevation(elevation))
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(40.0))
                    .style(iced::widget::button::primary)
                } else {
                    button(text(label).size(14))
                    .on_press(ElevationTiltSelectorMessage::SelectElevation(elevation))
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(40.0))
                    .style(iced::widget::button::secondary)
                };

            elevation_buttons = elevation_buttons.push(btn);
        }

        // Current selection info
        let current_info = if let Some(elevation) = self.selected_elevation {
            if elevation == elevation.floor() {
                format!("Selected: {:.0}°", elevation)
            } else {
                format!("Selected: {:.1}°", elevation)
            }
        } else {
            "No elevation selected".to_string()
        };

        let current_text = text(current_info).size(14);

        // Build the layout
        let content = row![
            text("Elevation Tilt").size(16),
            elevation_buttons,
            current_text,
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center);

        content.into()
    }
}
