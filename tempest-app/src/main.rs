//! Tempest - NEXRAD Weather Radar Application
//!
//! Main application entry point using iced UI framework.

use iced::{Element, Task, Theme};
use tempest_app::{
    cache_manager::CacheManager,
    color_legend::ColorLegend,
    config::AppConfig,
    elevation_tilt_selector::ElevationTiltSelector,
    moment_switcher::MomentSwitcher,
    offline_detection,
    offline_indicator::OfflineIndicator,
    station_selector::StationSelector,
    timeline::TimelineState,
    update,
    Message, State,
};
use tempest_fetch::cache_default;
use tempest_fetch::prefetch::Prefetcher;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application title
fn title(_state: &State) -> String {
    String::from("Tempest")
}

/// Build the view for the application
fn view(state: &State) -> Element<'_, Message> {
    use iced::widget::{column, text};

    // Build the main view from all components
    column![
        // Station selector at top
        state.station_selector.view().map(Message::StationSelector),
        // Moment switcher
        state.moment_switcher.view().map(Message::MomentSwitcher),
        // Elevation tilt selector
        state
            .elevation_tilt_selector
            .view(&Theme::Dark)
            .map(Message::ElevationTiltSelector),
        // Color legend
        state.color_legend.view().map(Message::ColorLegend),
        // Timeline at bottom
        state.timeline.view().map(Message::Timeline),
        // Offline indicator if needed
        state
            .offline_indicator
            .view()
            .map(Message::OfflineIndicator),
        // Cache manager
        state.cache_manager.view().map(Message::CacheManager),
        // Debug info
        text(format!(
            "Zoom: {} | Pan: {:?} | Playing: {}",
            state.zoom_level, state.pan_offset, state.is_playing
        ))
    ]
    .into()
}

/// Get the application theme
fn theme(_state: &State) -> Theme {
    Theme::Dark
}

/// Initialize the application state with async cache setup
fn initialize() -> (State, Task<Message>) {
    // Load configuration
    let config = AppConfig::load_or_default();

    // Create a tokio runtime for async cache initialization
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create tokio runtime");

    // Create the cache with default configuration
    let cache = runtime.block_on(async {
        cache_default()
            .await
            .expect("Failed to create default cache")
    });

    // Wrap cache in Arc<RwLock> for shared access
    let cache = Arc::new(RwLock::new(cache));

    // Create the cache manager with the cache
    let cache_manager = CacheManager::new(cache);

    // Build initial state
    let state = State {
        station_selector: StationSelector::new(),
        moment_switcher: MomentSwitcher::new(),
        elevation_tilt_selector: ElevationTiltSelector::with_elevations(vec![
            0.5, 1.5, 2.4, 3.3, 4.3,
        ]),
        color_legend: ColorLegend::new(),
        cache_manager,
        // Initialize offline indicator with current connectivity status
        offline_indicator: OfflineIndicator::new(offline_detection::is_online()),
        timeline: TimelineState::new(),
        prefetcher: Prefetcher::with_default_config(),
        connectivity_check_counter: 0,
        config,
        is_playing: false,
        zoom_level: 0,
        pan_offset: (0, 0),
        show_settings: false,
    };

    (state, Task::none())
}

/// Run the application
fn main() -> iced::Result {
    iced::application(title, update, view)
        .theme(theme)
        .run_with(initialize)
}
