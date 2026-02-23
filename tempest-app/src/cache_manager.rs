//! Cache Manager UI Component
//!
//! A widget for managing the radar data cache - displaying statistics,
//! clearing cache, and configuring cache limits.

use iced::widget::{button, column, container, row, text, text_input, Button, Row};
use iced::{Alignment, Element, Length};
use std::sync::Arc;
use tokio::sync::RwLock;
use tempest_fetch::cache::Cache;
use tempest_fetch::types::CacheStats;

/// Messages produced by the CacheManager component
#[derive(Debug, Clone)]
pub enum CacheManagerMessage {
    /// Clear the cache button pressed
    ClearCache,
    /// Max size input changed (in MB)
    MaxSizeChanged(String),
    /// Toggle the settings panel visibility
    ToggleSettings,
    /// Refresh cache statistics
    RefreshStats,
    /// Cache was cleared successfully
    CacheCleared,
    /// Error occurred during cache operation
    CacheError(String),
}

/// A component for managing the radar data cache
///
/// Displays cache statistics and allows users to clear the cache
/// or configure the maximum cache size.
#[derive(Debug)]
pub struct CacheManager {
    /// Reference to the cache (wrapped in Arc<RwLock> for async access)
    cache: Arc<RwLock<Cache>>,
    /// Current cache statistics (cached for display)
    stats: CacheStats,
    /// Input field value for max size (in MB)
    max_size_input: String,
    /// Whether to show the settings panel
    show_settings: bool,
    /// Current max size from config (in MB for display)
    current_max_size_mb: u64,
    /// Whether a clear operation is in progress
    clearing: bool,
}

impl CacheManager {
    /// Creates a new CacheManager with the given cache
    pub fn new(cache: Arc<RwLock<Cache>>) -> Self {
        let stats = CacheStats::default();
        let max_size_mb = 1024; // Default 1GB = 1024 MB

        Self {
            cache,
            stats,
            max_size_input: max_size_mb.to_string(),
            show_settings: false,
            current_max_size_mb: max_size_mb,
            clearing: false,
        }
    }

    /// Creates a new CacheManager with explicit max size
    pub fn with_max_size(cache: Arc<RwLock<Cache>>, max_size_mb: u64) -> Self {
        let stats = CacheStats::default();

        Self {
            cache,
            stats,
            max_size_input: max_size_mb.to_string(),
            show_settings: false,
            current_max_size_mb: max_size_mb,
            clearing: false,
        }
    }

    /// Updates the cached statistics (call this periodically)
    pub async fn refresh_stats(&mut self) {
        let cache = self.cache.read().await;
        self.stats = cache.stats().await;
        drop(cache);

        // Also get the current max size from config
        let config = &self.cache.read().await;
        // We can't access config directly since we only have a read lock
        // But we store it separately for display
    }

    /// Sets the current cache statistics
    pub fn set_stats(&mut self, stats: CacheStats) {
        self.stats = stats;
    }

    /// Handles incoming messages
    pub fn update(&mut self, message: CacheManagerMessage) -> Option<CacheManagerMessage> {
        match message {
            CacheManagerMessage::ClearCache => {
                self.clearing = true;
                // The actual clearing will be done asynchronously
                // Return a message to trigger the async operation
                Some(CacheManagerMessage::ClearCache)
            }
            CacheManagerMessage::MaxSizeChanged(value) => {
                // Validate and store the input
                self.max_size_input = value;
                None
            }
            CacheManagerMessage::ToggleSettings => {
                self.show_settings = !self.show_settings;
                None
            }
            CacheManagerMessage::RefreshStats => {
                // This should be handled by the parent to call refresh_stats() async
                None
            }
            CacheManagerMessage::CacheCleared => {
                self.clearing = false;
                // Stats will be refreshed after clear
                None
            }
            CacheManagerMessage::CacheError(_) => {
                self.clearing = false;
                None
            }
        }
    }

    /// Returns the max size input value
    #[must_use]
    pub fn max_size_input(&self) -> &str {
        &self.max_size_input
    }

    /// Returns whether settings are visible
    #[must_use]
    pub fn show_settings(&self) -> bool {
        self.show_settings
    }

    /// Returns whether clearing is in progress
    #[must_use]
    pub fn is_clearing(&self) -> bool {
        self.clearing
    }

    /// Formats bytes into human-readable string
    fn format_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    /// Returns the view for this component
    pub fn view(&self) -> Element<'_, CacheManagerMessage> {
        // Styling constants (matching other components)
        let heading_style = iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0));
        let label_style = iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7));
        let value_style = iced::theme::Text::Color(iced::Color::from_rgb(0.9, 0.9, 0.9));
        let warning_style = iced::theme::Text::Color(iced::Color::from_rgb(1.0, 0.4, 0.4));
        let success_style = iced::theme::Text::Color(iced::Color::from_rgb(0.4, 1.0, 0.4));

        // Cache statistics
        let total_size = Self::format_size(self.stats.total_size);
        let entry_count = self.stats.entry_count;

        // Calculate usage percentage
        let usage_percent = if self.current_max_size_mb > 0 {
            let max_bytes = self.current_max_size_mb as f64 * 1024.0 * 1024.0;
            (self.stats.total_size as f64 / max_bytes * 100.0).min(100.0)
        } else {
            0.0
        };

        // Clear cache button
        let clear_button = if self.clearing {
            button(text("Clearing...").style(warning_style))
                .on_press(CacheManagerMessage::ClearCache)
                .width(Length::Fixed(150.0))
                .padding(10)
                .style(Button::Secondary)
        } else {
            button(text("Clear Cache").style(value_style))
                .on_press(CacheManagerMessage::ClearCache)
                .width(Length::Fixed(150.0))
                .padding(10)
                .style(Button::Destructive)
        };

        // Settings toggle button
        let settings_toggle_text = if self.show_settings {
            "▲ Hide Settings"
        } else {
            "▼ Show Settings"
        };
        let settings_toggle = button(text(settings_toggle_text).style(label_style))
            .on_press(CacheManagerMessage::ToggleSettings)
            .padding(5)
            .style(Button::Text);

        // Settings panel (conditional)
        let settings_panel: Element<'_, CacheManagerMessage> = if self.show_settings {
            let max_size_input = text_input("Max size (MB)", &self.max_size_input)
                .on_input(CacheManagerMessage::MaxSizeChanged)
                .width(Length::Fixed(150.0))
                .padding(8);

            let settings_content = column![
                text("Cache Settings").style(heading_style).size(14),
                row![
                    text("Maximum Size (MB):").style(label_style),
                    max_size_input,
                ]
                .spacing(10)
                .align_items(Alignment::Center),
                text(format!("Current limit: {} MB", self.current_max_size_mb))
                    .style(label_style)
                    .size(12),
            ]
            .spacing(8)
            .padding(10);

            container(settings_content)
                .padding(10)
                .into()
        } else {
            text("").into()
        };

        // Build main content
        let content = column![
            text("Cache Management").style(heading_style).size(20),
            text("").size(10),
            
            // Statistics section
            text("Current Statistics").style(heading_style).size(16),
            text("").size(5),
            row![
                text("Total Size:").style(label_style),
                text(&total_size).style(value_style),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            row![
                text("Entries:").style(label_style),
                text(entry_count.to_string()).style(value_style),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            row![
                text("Usage:").style(label_style),
                text(format!("{:.1}%", usage_percent)).style(
                    if usage_percent > 90.0 {
                        warning_style
                    } else if usage_percent > 70.0 {
                        label_style
                    } else {
                        success_style
                    }
                ),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            
            text("").size(15),
            
            // Clear cache section
            text("Actions").style(heading_style).size(16),
            text("").size(5),
            clear_button,
            
            text("").size(15),
            
            // Settings section
            settings_toggle,
            settings_panel,
        ]
        .spacing(5)
        .align_items(Alignment::Start)
        .padding(20)
        .width(Length::FillPortion(1));

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        // This won't actually work without a real cache, but required for derive
        panic!("CacheManager::default() should not be called - use CacheManager::new()");
    }
}
