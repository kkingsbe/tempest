//! Cache Manager UI Component
//!
//! A widget for managing the radar data cache - displaying statistics,
//! clearing cache, and configuring cache limits.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element, Length};
use std::sync::Arc;
use tempest_fetch::{Cache, CacheStats};
use tokio::sync::RwLock;

/// Messages produced by the CacheManager component
#[derive(Debug, Clone)]
pub enum CacheManagerMessage {
    /// Clear the cache button pressed
    ClearCache,
    /// Max size input changed (in MB)
    MaxSizeChanged(String),
    /// Apply the max size setting
    ApplyMaxSize,
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
    cache: Option<Arc<RwLock<Cache>>>,
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
            cache: Some(cache),
            stats,
            max_size_input: max_size_mb.to_string(),
            show_settings: false,
            current_max_size_mb: max_size_mb,
            clearing: false,
        }
    }

    /// Creates a new CacheManager with explicit max size
    #[allow(dead_code)]
    pub fn with_max_size(cache: Arc<RwLock<Cache>>, max_size_mb: u64) -> Self {
        let stats = CacheStats::default();

        Self {
            cache: Some(cache),
            stats,
            max_size_input: max_size_mb.to_string(),
            show_settings: false,
            current_max_size_mb: max_size_mb,
            clearing: false,
        }
    }

    /// Updates the cached statistics (call this periodically)
    #[allow(dead_code)]
    pub async fn refresh_stats(&mut self) {
        if let Some(cache) = &self.cache {
            let cache = cache.read().await;
            self.stats = cache.stats().await;
        }
    }

    /// Sets the current cache statistics
    #[allow(dead_code)]
    pub fn set_stats(&mut self, stats: CacheStats) {
        self.stats = stats;
    }

    /// Handles incoming messages
    pub fn update(&mut self, message: CacheManagerMessage) -> Option<CacheManagerMessage> {
        match message {
            CacheManagerMessage::ClearCache => {
                // Spawn async task to clear the cache
                // We need to do this because update() is synchronous but cache.clear() is async
                if let Some(cache) = self.cache.clone() {
                    self.clearing = true;
                    let cache = cache.clone();
                    tokio::spawn(async move {
                        let result = async {
                            let mut cache = cache.write().await;
                            cache.clear().await
                        }
                        .await;
                        match result {
                            Ok(()) => {
                                tracing::info!("Cache cleared successfully");
                            }
                            Err(e) => {
                                tracing::error!("Failed to clear cache: {}", e);
                            }
                        }
                    });
                }
                None
            }
            CacheManagerMessage::MaxSizeChanged(value) => {
                // Validate and store the input
                self.max_size_input = value;
                None
            }
            CacheManagerMessage::ApplyMaxSize => {
                // Parse the input and apply as the current max size
                if let Ok(size) = self.max_size_input.parse::<u64>() {
                    self.current_max_size_mb = size;
                }
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

    /// Clears the cache asynchronously
    ///
    /// This method must be called from an async context. It acquires a write lock
    /// on the cache and clears all entries.
    ///
    /// Note: Prefer using the ClearCache message which handles this automatically
    /// via tokio::spawn in the update method.
    #[allow(dead_code)]
    pub async fn clear_cache(&mut self) {
        let cache = match &self.cache {
            Some(c) => c,
            None => {
                tracing::warn!("Cache not initialized, cannot clear");
                return;
            }
        };

        self.clearing = true;

        let result = async {
            let mut cache = cache.write().await;
            cache.clear().await
        }
        .await;

        match result {
            Ok(()) => {
                // Refresh stats after clearing
                self.stats = cache.read().await.stats().await;
                tracing::info!("Cache cleared successfully");
            }
            Err(e) => {
                tracing::error!("Failed to clear cache: {}", e);
            }
        }

        self.clearing = false;
    }

    /// Returns the current max size setting (in MB)
    #[must_use]
    #[allow(dead_code)]
    pub fn current_max_size_mb(&self) -> u64 {
        self.current_max_size_mb
    }

    /// Returns whether settings are visible
    #[must_use]
    #[allow(dead_code)]
    pub fn show_settings(&self) -> bool {
        self.show_settings
    }

    /// Returns whether clearing is in progress
    #[must_use]
    #[allow(dead_code)]
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
            button(text("Clearing..."))
                .on_press(CacheManagerMessage::ClearCache)
                .width(Length::Fixed(150.0))
                .padding(10)
        } else {
            button(text("Clear Cache"))
                .on_press(CacheManagerMessage::ClearCache)
                .width(Length::Fixed(150.0))
                .padding(10)
        };

        // Settings toggle button
        let settings_toggle_text = if self.show_settings {
            "▲ Hide Settings"
        } else {
            "▼ Show Settings"
        };
        let settings_toggle = button(text(settings_toggle_text))
            .on_press(CacheManagerMessage::ToggleSettings)
            .padding(5);

        // Settings panel (conditional)
        let settings_panel: Element<'_, CacheManagerMessage> = if self.show_settings {
            let max_size_input = text_input("Max size (MB)", &self.max_size_input)
                .on_input(CacheManagerMessage::MaxSizeChanged)
                .width(Length::Fixed(150.0))
                .padding(8);

            let apply_button = button(text("Apply"))
                .on_press(CacheManagerMessage::ApplyMaxSize)
                .width(Length::Fixed(80.0))
                .padding(8);

            let settings_content = column![
                text("Cache Settings").size(14),
                text("").size(5),
                row![
                    text("Maximum Size (MB):"),
                    max_size_input,
                    apply_button,
                ]
                .spacing(10)
                .align_y(Alignment::Center),
                text(format!("Current limit: {} MB", self.current_max_size_mb))
                    .size(12),
                text("Note: Cache limit requires app restart to take effect")
                    .size(10),
            ]
            .spacing(8)
            .padding(10);

            container(settings_content).padding(10).into()
        } else {
            text("").into()
        };

        // Build main content
        let content = column![
            text("Cache Management").size(20),
            text("").size(10),
            // Statistics section
            text("Current Statistics").size(16),
            text("").size(5),
            row![
                text("Total Size:"),
                text(total_size.clone()),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                text("Entries:"),
                text(entry_count.to_string()),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                text("Usage:"),
                text(format!("{:.1}%", usage_percent)),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text("").size(15),
            // Clear cache section
            text("Actions").size(16),
            text("").size(5),
            clear_button,
            text("").size(15),
            // Settings section
            settings_toggle,
            settings_panel,
        ]
        .spacing(5)
        .align_x(Alignment::Start)
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
        // Return a sensible default with no cache initialized
        // This allows the struct to be used in contexts where a full cache isn't available
        let stats = CacheStats::default();
        let max_size_mb = 1024; // Default 1GB = 1024 MB

        Self {
            cache: None,
            stats,
            max_size_input: max_size_mb.to_string(),
            show_settings: false,
            current_max_size_mb: max_size_mb,
            clearing: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempest_fetch::CacheStats;

    #[test]
    fn test_default_creation() {
        let manager = CacheManager::default();
        
        assert!(manager.cache.is_none());
        assert_eq!(manager.stats.total_size, 0);
        assert_eq!(manager.stats.entry_count, 0);
        assert_eq!(manager.max_size_input, "1024");
        assert!(!manager.show_settings);
        assert_eq!(manager.current_max_size_mb, 1024);
        assert!(!manager.clearing);
    }

    #[test]
    fn test_current_max_size_mb() {
        let manager = CacheManager::default();
        assert_eq!(manager.current_max_size_mb(), 1024);
    }

    #[test]
    fn test_show_settings_default() {
        let manager = CacheManager::default();
        assert!(!manager.show_settings());
    }

    #[test]
    fn test_is_clearing_default() {
        let manager = CacheManager::default();
        assert!(!manager.is_clearing());
    }

    #[test]
    fn test_toggle_settings() {
        let mut manager = CacheManager::default();
        
        // Initially false
        assert!(!manager.show_settings());
        
        // Toggle on
        manager.update(CacheManagerMessage::ToggleSettings);
        assert!(manager.show_settings());
        
        // Toggle off
        manager.update(CacheManagerMessage::ToggleSettings);
        assert!(!manager.show_settings());
    }

    #[test]
    fn test_max_size_changed() {
        let mut manager = CacheManager::default();
        
        // Initial value
        assert_eq!(manager.max_size_input, "1024");
        
        // Update the input
        manager.update(CacheManagerMessage::MaxSizeChanged("2048".to_string()));
        assert_eq!(manager.max_size_input, "2048");
        
        // Update with empty string
        manager.update(CacheManagerMessage::MaxSizeChanged("".to_string()));
        assert_eq!(manager.max_size_input, "");
        
        // Update with invalid input
        manager.update(CacheManagerMessage::MaxSizeChanged("abc".to_string()));
        assert_eq!(manager.max_size_input, "abc");
    }

    #[test]
    fn test_apply_max_size_valid() {
        let mut manager = CacheManager::default();
        
        // Set the input
        manager.update(CacheManagerMessage::MaxSizeChanged("2048".to_string()));
        assert_eq!(manager.max_size_input, "2048");
        
        // Apply the max size
        manager.update(CacheManagerMessage::ApplyMaxSize);
        assert_eq!(manager.current_max_size_mb, 2048);
    }

    #[test]
    fn test_apply_max_size_invalid() {
        let mut manager = CacheManager::default();
        
        // Set invalid input
        manager.update(CacheManagerMessage::MaxSizeChanged("abc".to_string()));
        
        // Apply should not change current_max_size_mb
        manager.update(CacheManagerMessage::ApplyMaxSize);
        assert_eq!(manager.current_max_size_mb, 1024); // Should remain at default
    }

    #[test]
    fn test_apply_max_size_empty() {
        let mut manager = CacheManager::default();
        
        // Set empty input
        manager.update(CacheManagerMessage::MaxSizeChanged("".to_string()));
        
        // Apply should not change current_max_size_mb
        manager.update(CacheManagerMessage::ApplyMaxSize);
        assert_eq!(manager.current_max_size_mb, 1024); // Should remain at default
    }

    #[test]
    fn test_cache_cleared_message() {
        let mut manager = CacheManager { clearing: true, ..Default::default() };
        
        assert!(manager.is_clearing());
        
        // Handle cache cleared message
        manager.update(CacheManagerMessage::CacheCleared);
        assert!(!manager.is_clearing());
    }

    #[test]
    fn test_cache_error_message() {
        let mut manager = CacheManager { clearing: true, ..Default::default() };
        
        assert!(manager.is_clearing());
        
        // Handle cache error message
        manager.update(CacheManagerMessage::CacheError("Test error".to_string()));
        assert!(!manager.is_clearing());
    }

    #[test]
    fn test_set_stats() {
        let mut manager = CacheManager::default();
        
        // Default stats
        assert_eq!(manager.stats.entry_count, 0);
        assert_eq!(manager.stats.total_size, 0);
        
        // Set custom stats
        let custom_stats = CacheStats {
            entry_count: 100,
            total_size: 1024 * 1024 * 100, // 100 MB
            oldest: None,
            newest: None,
        };
        manager.set_stats(custom_stats);
        
        assert_eq!(manager.stats.entry_count, 100);
        assert_eq!(manager.stats.total_size, 1024 * 1024 * 100);
    }

    #[test]
    fn test_format_size_bytes() {
        // Test bytes
        let result = CacheManager::format_size(512);
        assert_eq!(result, "512 B");
        
        // Test exactly 1023 bytes
        let result = CacheManager::format_size(1023);
        assert_eq!(result, "1023 B");
    }

    #[test]
    fn test_format_size_kilobytes() {
        // Test KB
        let result = CacheManager::format_size(1024);
        assert_eq!(result, "1.00 KB");
        
        // Test 1.5 KB
        let result = CacheManager::format_size(1536);
        assert_eq!(result, "1.50 KB");
    }

    #[test]
    fn test_format_size_megabytes() {
        // Test MB
        let result = CacheManager::format_size(1024 * 1024);
        assert_eq!(result, "1.00 MB");
        
        // Test 50 MB
        let result = CacheManager::format_size(50 * 1024 * 1024);
        assert_eq!(result, "50.00 MB");
    }

    #[test]
    fn test_format_size_gigabytes() {
        // Test GB
        let result = CacheManager::format_size(1024 * 1024 * 1024);
        assert_eq!(result, "1.00 GB");
        
        // Test 2.5 GB
        let result = CacheManager::format_size((2.5 * 1024.0 * 1024.0 * 1024.0) as u64);
        assert_eq!(result, "2.50 GB");
    }
}
