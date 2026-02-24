//! Semantic color constants for Tempest UI
//!
//! This module provides centralized color definitions for the application,
//! replacing raw RGB values with semantic, reusable constants.

use iced::Color;

/// Surface colors (backgrounds)
pub mod surface {
    use super::Color;

    /// Primary background color
    #[allow(dead_code)]
    pub const BG_PRIMARY: Color = Color::from_rgb(0.1, 0.1, 0.15);
    /// Elevated background color (for cards, panels)
    #[allow(dead_code)]
    pub const BG_ELEVATED: Color = Color::from_rgb(0.15, 0.15, 0.2);
}

/// Text colors
pub mod text {
    use super::Color;

    /// Primary text color (high contrast)
    #[allow(dead_code)]
    pub const PRIMARY: Color = Color::from_rgb(0.93, 0.93, 0.95);
    /// Secondary text color (medium contrast)
    #[allow(dead_code)]
    pub const SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
    /// Muted text color (low contrast)
    #[allow(dead_code)]
    pub const MUTED: Color = Color::from_rgb(0.4, 0.4, 0.45);
}

/// Accent colors (interactive elements, highlights)
pub mod accent {
    use super::Color;

    /// Primary accent color (blue)
    #[allow(dead_code)]
    pub const PRIMARY: Color = Color::from_rgb(0.2, 0.6, 1.0);
    /// Hover state for accent elements
    #[allow(dead_code)]
    pub const HOVER: Color = Color::from_rgb(0.35, 0.75, 1.0);
}

/// Border colors
pub mod border {
    use super::Color;

    /// Default border color
    #[allow(dead_code)]
    pub const DEFAULT: Color = Color::from_rgb(0.15, 0.15, 0.2);
}
