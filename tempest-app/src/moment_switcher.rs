//! Moment Switcher UI Component
//!
//! A widget for switching between different NEXRAD radar moments (data types).

#![allow(clippy::upper_case_acronyms)]

use iced::widget::{button, row, text};
use iced::{Border, Element, Length};

mod colors {
    pub use crate::colors::{accent, border, surface, text};
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
        // Custom button style for primary/selected state (filled accent)
        let primary_button_style = |_theme: &iced::Theme, status: button::Status| {
            let is_selected =
                status == button::Status::Pressed || status == button::Status::Hovered;
            button::Style {
                background: Some(if is_selected {
                    colors::accent::HOVER.into()
                } else {
                    colors::accent::PRIMARY.into()
                }),
                text_color: colors::text::PRIMARY,
                border: Border::default().rounded(8),
                ..button::Style::default()
            }
        };

        // Custom button style for secondary/unselected state (outlined/muted)
        let secondary_button_style = |_theme: &iced::Theme, status: button::Status| {
            let is_hovered = status == button::Status::Hovered;
            button::Style {
                background: Some(if is_hovered {
                    colors::surface::BG_ELEVATED.into()
                } else {
                    colors::surface::BG_PRIMARY.into()
                }),
                text_color: colors::text::PRIMARY,
                border: Border::default()
                    .rounded(8)
                    .width(1)
                    .color(colors::border::DEFAULT),
                ..button::Style::default()
            }
        };

        // Build row of moment buttons
        let mut moment_buttons = row!().spacing(8);

        for moment in Moment::all() {
            let is_selected = self.selected_moment == moment;

            // Style for selected vs unselected buttons using custom semantic styles
            let btn = if is_selected {
                button(text(format!("{}\n{}", moment.code(), moment.name())).size(12))
                    .on_press(MomentSwitcherMessage::MomentSelected(moment))
                    .width(Length::Fixed(112.0))
                    .height(Length::Fixed(48.0))
                    .padding([12, 24])
                    .style(primary_button_style)
            } else {
                button(text(format!("{}\n{}", moment.code(), moment.name())).size(12))
                    .on_press(MomentSwitcherMessage::MomentSelected(moment))
                    .width(Length::Fixed(112.0))
                    .height(Length::Fixed(48.0))
                    .padding([12, 24])
                    .style(secondary_button_style)
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
        .size(14);

        // Build the layout
        let content = row![text("Radar Moment").size(16), moment_buttons, current_info,]
            .spacing(12)
            .align_y(iced::Alignment::Center);

        content.into()
    }
}
