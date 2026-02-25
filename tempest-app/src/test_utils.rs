//! Test utilities for GUI testing.
//!
//! This module exposes internal types needed for headless GUI testing
//! without running a window.

pub mod gui_harness {
    // Re-export types from the crate root (lib.rs)
    pub use crate::Message;
    pub use crate::PanDirection;
    pub use crate::State;
    // Use the update function directly from the crate
    pub use crate::update;
    pub use crate::config::AppConfig;
    pub use crate::moment_switcher::Moment;
    pub use crate::station_selector;
    pub use crate::moment_switcher;
    pub use crate::elevation_tilt_selector;
    pub use crate::color_legend;
    pub use crate::cache_manager;
    pub use crate::offline_indicator;
    pub use crate::offline_detection;
    pub use crate::timeline;
    
    // Re-export component types
    pub use crate::cache_manager::CacheManager;
    pub use crate::color_legend::ColorLegend;
    pub use crate::elevation_tilt_selector::ElevationTiltSelector;
    pub use crate::moment_switcher::MomentSwitcher;
    pub use crate::offline_indicator::OfflineIndicator;
    pub use crate::station_selector::StationSelector;
    pub use crate::timeline::TimelineState;
    pub use tempest_fetch::prefetch::Prefetcher;
    pub use tempest_fetch::Station;
    
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tempest_fetch::cache_default;

    /// Initialize the application state for testing.
    /// This creates a State instance with default/empty values suitable for testing.
    pub fn initialize() -> State {
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

        State {
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
