//! Offline indicator component for Tempest application.
//!
//! Displays network connectivity status in the UI.

use iced::widget::{container, row, text};
use iced::{Color, Element, Length};

/// Semantic color palette for the OfflineIndicator component
mod colors {
    use iced::Color;
    pub const SUCCESS: Color = Color::from_rgb(0.3, 0.9, 0.3);
    pub const DANGER: Color = Color::from_rgb(0.9, 0.3, 0.3);
}

/// Offline indicator colors
const ONLINE_COLOR: Color = colors::SUCCESS;
const OFFLINE_COLOR: Color = colors::DANGER;

/// Messages for the offline indicator component
#[derive(Debug, Clone, Copy, Default)]
pub enum OfflineIndicatorMessage {
    /// Request to check connectivity
    #[default]
    CheckConnectivity,
}

/// Offline indicator component that displays network status
#[derive(Debug, Clone)]
pub struct OfflineIndicator {
    /// Whether the system is currently online
    is_online: bool,
}

impl OfflineIndicator {
    /// Create a new offline indicator
    ///
    /// # Arguments
    /// * `is_online` - Initial online status
    pub fn new(is_online: bool) -> Self {
        Self { is_online }
    }

    /// Create an offline indicator with default offline status
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self { is_online: false }
    }

    /// Update the online status
    pub fn update(&mut self, message: OfflineIndicatorMessage) {
        match message {
            OfflineIndicatorMessage::CheckConnectivity => {
                // Status is updated externally via set_online
            }
        }
    }

    /// Set the online status
    #[allow(dead_code)]
    pub fn set_online(&mut self, online: bool) {
        self.is_online = online;
    }

    /// Get the current online status
    pub fn is_online(&self) -> bool {
        self.is_online
    }

    /// Render the offline indicator view
    ///
    /// Shows a colored dot with "Online" or "Offline" text
    pub fn view(&self) -> Element<'_, OfflineIndicatorMessage> {
        let (color, status_text) = if self.is_online {
            (ONLINE_COLOR, "Online")
        } else {
            (OFFLINE_COLOR, "Offline")
        };

        // Create the indicator row with a colored dot and text
        let indicator = row![
            // Colored dot indicator
            container(text("●").size(16))
                .width(Length::Shrink)
                .padding(8),
            // Status text
            text(status_text)
                .size(14),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center);

        // Wrap in a container positioned at top-right
        container(indicator)
            .padding(12)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into()
    }
}

// Manual Default impl removed - using derive instead
