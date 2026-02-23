//! Moment Switcher UI Component
//!
//! A widget for switching between different NEXRAD radar moments (data types).

#![allow(clippy::upper_case_acronyms)]

use iced::widget::{button, row, text};
use iced::{Element, Length};

/// Semantic color constants for this module
mod colors {
    use iced::Color;

    // Accent color for headings and interactive highlights
    pub const ACCENT: Color = Color::from_rgb(0.2, 0.6, 1.0);

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
    pub const TEXT_UNSELECTED: Color = Color::from_rgb(0.7, 0.7, 0.8);
}

/// Supported radar moments/data types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Moment {
    /// Reflectivity (dBZ) - measures precipitation intensity
    #[default]
    REF,
    /// Velocity (m/s) - measures wind speed toward/away from radar
    VEL,
    /// Spectrum Width (m/s) - measures velocity variability
    SW,
    /// Differential Reflectivity (dB) - helps identify hydrometeor types
    ZDR,
    /// Correlation Coefficient (0-1) - helps identify debris/mixed precipitation
    CC,
    /// Specific Differential Phase (deg/km) - helps estimate rainfall rate
    KDP,
}

impl Moment {
    /// Returns a slice of all supported moments
    pub const fn all() -> [Moment; 6] {
        [
            Moment::REF,
            Moment::VEL,
            Moment::SW,
            Moment::ZDR,
            Moment::CC,
            Moment::KDP,
        ]
    }

    /// Returns the human-readable name for this moment
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Moment::REF => "Reflectivity",
            Moment::VEL => "Velocity",
            Moment::SW => "Spectrum Width",
            Moment::ZDR => "Diff. Reflectivity",
            Moment::CC => "Correlation Coef.",
            Moment::KDP => "Spec. Diff. Phase",
        }
    }

    /// Returns the short code for this moment
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Moment::REF => "REF",
            Moment::VEL => "VEL",
            Moment::SW => "SW",
            Moment::ZDR => "ZDR",
            Moment::CC => "CC",
            Moment::KDP => "KDP",
        }
    }

    /// Returns the unit of measurement for this moment
    #[must_use]
    pub fn unit(&self) -> &'static str {
        match self {
            Moment::REF => "dBZ",
            Moment::VEL => "m/s",
            Moment::SW => "m/s",
            Moment::ZDR => "dB",
            Moment::CC => "",
            Moment::KDP => "deg/km",
        }
    }
}

/// Messages produced by the MomentSwitcher component
#[derive(Debug, Clone, Copy)]
pub enum MomentSwitcherMessage {
    /// A moment was selected
    MomentSelected(Moment),
}

/// A component for switching between different radar moments
///
/// Displays a row of buttons for selecting between REF, VEL, SW, ZDR, CC, and KDP.
#[derive(Debug, Clone)]
pub struct MomentSwitcher {
    /// Currently selected moment
    selected_moment: Moment,
}

impl Default for MomentSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl MomentSwitcher {
    /// Creates a new MomentSwitcher with the default moment selected
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            selected_moment: Moment::REF,
        }
    }

    /// Creates a new MomentSwitcher with a specific initial moment
    #[allow(dead_code)]
    pub fn with_moment(moment: Moment) -> Self {
        Self {
            selected_moment: moment,
        }
    }

    /// Returns the currently selected moment
    #[allow(dead_code)]
    #[must_use]
    pub fn selected_moment(&self) -> Moment {
        self.selected_moment
    }

    /// Sets the selected moment
    #[allow(dead_code)]
    pub fn set_moment(&mut self, moment: Moment) {
        self.selected_moment = moment;
    }

    /// Handles incoming messages
    pub fn update(&mut self, message: MomentSwitcherMessage) {
        let MomentSwitcherMessage::MomentSelected(moment) = message;
        self.selected_moment = moment;
    }

    /// Returns the view for this component
    pub fn view(&self) -> Element<'_, MomentSwitcherMessage> {
        // Styling constants using semantic colors
        let heading_style = iced::theme::Text::Color(colors::ACCENT);
        let label_style = iced::theme::Text::Color(colors::TEXT_SECONDARY);

        // Build row of moment buttons
        let mut moment_buttons = row!().spacing(8);

        for moment in Moment::all() {
            let is_selected = self.selected_moment == moment;

            // Style for selected vs unselected buttons using different button styles
            let btn = if is_selected {
                button(
                    text(format!("{}\n{}", moment.code(), moment.name()))
                        .size(12)
                        .style(iced::theme::Text::Color(colors::TEXT_PRIMARY)),
                )
                .on_press(MomentSwitcherMessage::MomentSelected(moment))
                .width(Length::Fixed(110.0))
                .height(Length::Fixed(50.0))
                .style(iced::theme::Button::Primary)
            } else {
                button(
                    text(format!("{}\n{}", moment.code(), moment.name()))
                        .size(12)
                        .style(iced::theme::Text::Color(colors::TEXT_UNSELECTED)),
                )
                .on_press(MomentSwitcherMessage::MomentSelected(moment))
                .width(Length::Fixed(110.0))
                .height(Length::Fixed(50.0))
                .style(iced::theme::Button::Secondary)
            };

            moment_buttons = moment_buttons.push(btn);
        }

        // Current selection info
        let current_info = text(format!(
            "Selected: {} ({}) - Unit: {}",
            self.selected_moment.name(),
            self.selected_moment.code(),
            self.selected_moment.unit()
        ))
        .style(label_style)
        .size(14);

        // Build the layout
        let content = row![
            text("Radar Moment").style(heading_style).size(16),
            moment_buttons,
            current_info,
        ]
        .spacing(12)
        .align_items(iced::Alignment::Center);

        content.into()
    }
}
