//! Test utilities for GUI testing.
//!
//! This module exposes internal types needed for headless GUI testing
//! without running a window.

pub mod gui_harness {
    use chrono::{DateTime, Utc};
    use iced::Task;
    
    use crate::cache_manager::{CacheManager, CacheManagerMessage};
    use crate::color_legend::{ColorLegend, ColorLegendMessage};
    use crate::config::AppConfig;
    use crate::elevation_tilt_selector::{ElevationTiltSelector, ElevationTiltSelectorMessage};
    use crate::moment_switcher::{MomentSwitcher, MomentSwitcherMessage, Moment};
    use crate::offline_detection;
    use crate::offline_indicator::{OfflineIndicator, OfflineIndicatorMessage};
    use crate::station_selector::{StationSelector, StationSelectorMessage};
    use crate::timeline::{TimelineMessage, TimelineState};
    use crate::PanDirection;
    use tempest_fetch::prefetch::{Prefetcher, PrefetchConfig, PrefetchPrediction};
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tempest_fetch::cache_default;
    use crate::Message;

    /// Application state - re-exported for testing
    #[derive(Debug)]
    pub struct State {
        /// Station selector component
        pub station_selector: StationSelector,
        /// Moment switcher component
        pub moment_switcher: MomentSwitcher,
        /// Elevation tilt selector component
        pub elevation_tilt_selector: ElevationTiltSelector,
        /// Color legend component
        pub color_legend: ColorLegend,
        /// Cache manager component
        pub cache_manager: CacheManager,
        /// Offline indicator component
        pub offline_indicator: OfflineIndicator,
        /// Timeline component
        pub timeline: TimelineState,
        /// Prefetcher for predictive data loading
        pub prefetcher: Prefetcher,
        /// Counter for periodic connectivity checks
        pub connectivity_check_counter: u32,
        /// Application configuration
        pub config: AppConfig,
        /// Playback state
        pub is_playing: bool,
        /// Current zoom level (0 = default, positive = zoomed in, negative = zoomed out)
        pub zoom_level: i32,
        /// Pan offset for map (x, y)
        pub pan_offset: (i32, i32),
        /// Whether settings panel is visible
        pub show_settings: bool,
    }

    /// Direction for panning
    #[derive(Debug, Clone, Copy)]
    pub enum PanDirection {
        Up,
        Down,
        Left,
        Right,
    }

    impl From<crate::PanDirection> for PanDirection {
        fn from(d: crate::PanDirection) -> Self {
            match d {
                crate::PanDirection::Up => PanDirection::Up,
                crate::PanDirection::Down => PanDirection::Down,
                crate::PanDirection::Left => PanDirection::Left,
                crate::PanDirection::Right => PanDirection::Right,
            }
        }
    }

    impl From<PanDirection> for crate::PanDirection {
        fn from(d: PanDirection) -> Self {
            match d {
                PanDirection::Up => crate::PanDirection::Up,
                PanDirection::Down => crate::PanDirection::Down,
                PanDirection::Left => crate::PanDirection::Left,
                PanDirection::Right => crate::PanDirection::Right,
            }
        }
    }

    /// Update the application state with a message
    pub fn update(state: &mut State, message: Message) -> Task<Message> {
        crate::update(state, message)
    }

    impl State {
        /// Create a new State with default values for testing
        pub fn new_for_testing() -> Self {
            // Create a minimal cache for testing
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");

            let cache = runtime.block_on(async {
                cache_default()
                    .await
                    .expect("Failed to create default cache")
            });

            let cache = Arc::new(RwLock::new(cache));
            let cache_manager = CacheManager::new(cache);

            let config = AppConfig::load_or_default();

            Self {
                station_selector: StationSelector::new(),
                moment_switcher: MomentSwitcher::new(),
                elevation_tilt_selector: ElevationTiltSelector::with_elevations(vec![
                    0.5, 1.5, 2.4, 3.3, 4.3,
                ]),
                color_legend: ColorLegend::new(),
                cache_manager,
                offline_indicator: OfflineIndicator::new(offline_detection::is_online()),
                timeline: TimelineState::new(),
                prefetcher: Prefetcher::with_default_config(),
                connectivity_check_counter: 0,
                config,
                is_playing: false,
                zoom_level: 0,
                pan_offset: (0, 0),
                show_settings: false,
            }
        }
    }

    /// Re-export for tests
    pub use crate::Message;
    pub use crate::moment_switcher::Moment;
    pub use tempest_fetch::Station;
}
