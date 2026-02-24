//! Tempest - NEXRAD Weather Radar Application
//!
//! Main application entry point using iced UI framework.

mod cache_manager;
mod color_legend;
mod colors;
mod config;
mod elevation_tilt_selector;
mod moment_switcher;
mod offline_detection;
mod offline_indicator;
mod spacing;
mod station_selector;
mod timeline;

use cache_manager::{CacheManager, CacheManagerMessage};
use color_legend::{ColorLegend, ColorLegendMessage};
use elevation_tilt_selector::{ElevationTiltSelector, ElevationTiltSelectorMessage};
use iced::{Element, Task, Theme};
use moment_switcher::{MomentSwitcher, MomentSwitcherMessage};
use offline_indicator::{OfflineIndicator, OfflineIndicatorMessage};
use station_selector::{StationSelector, StationSelectorMessage};
use std::sync::Arc;
use tempest_fetch::cache_default;
use tempest_fetch::prefetch::{PlaybackDirection, PlaybackState, Prefetcher};
use tempest_render_core::color::RadarMoment;
use timeline::{TimelineMessage, TimelineState};
use tokio::sync::RwLock;

/// Application state
#[derive(Debug)]
pub struct State {
    /// Station selector component
    station_selector: StationSelector,
    /// Moment switcher component
    moment_switcher: MomentSwitcher,
    /// Elevation tilt selector component
    elevation_tilt_selector: ElevationTiltSelector,
    /// Color legend component
    color_legend: ColorLegend,
    /// Cache manager component
    cache_manager: CacheManager,
    /// Offline indicator component
    offline_indicator: OfflineIndicator,
    /// Timeline component
    timeline: TimelineState,
    /// Prefetcher for predictive data loading
    prefetcher: Prefetcher,
    /// Counter for periodic connectivity checks
    #[allow(dead_code)]
    connectivity_check_counter: u32,
    /// Application configuration
    config: config::AppConfig,
    /// Playback state
    #[allow(dead_code)]
    is_playing: bool,
    /// Current zoom level (0 = default, positive = zoomed in, negative = zoomed out)
    zoom_level: i32,
    /// Pan offset for map (x, y)
    pan_offset: (i32, i32),
    /// Whether settings panel is visible
    show_settings: bool,
}

impl State {
    /// Sync the prefetcher with current timeline state
    fn sync_prefetcher_with_timeline(&mut self) {
        // Get timeline state
        let is_playing = self.timeline.is_playing();
        let current_index = self.timeline.current_index();
        let speed = self.timeline.playback_speed() as f64;

        // Determine direction
        let direction = if is_playing {
            // Default to Forward when playing
            PlaybackDirection::Forward
        } else {
            PlaybackDirection::Paused
        };

        // Set available scans (use scan times as keys - format as strings)
        let scan_keys: Vec<String> = self
            .timeline
            .scan_times()
            .iter()
            .map(|t| t.format("%Y%m%d%H%M").to_string())
            .collect();
        self.prefetcher.set_available_scans(scan_keys);

        // Update playback state
        let state = PlaybackState {
            current_index,
            total_scans: self.timeline.scan_count(),
            direction,
            speed: if is_playing { speed } else { 0.0 },
            last_update: chrono::Utc::now(),
        };
        self.prefetcher.update_playback_state(state);
    }
}

/// Direction for panning
#[derive(Debug, Clone, Copy)]
pub enum PanDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Messages that can be sent to the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Station selector internal messages
    StationSelector(StationSelectorMessage),
    /// Moment switcher internal messages
    MomentSwitcher(MomentSwitcherMessage),
    /// Elevation tilt selector internal messages
    ElevationTiltSelector(ElevationTiltSelectorMessage),
    /// Color legend internal messages
    ColorLegend(ColorLegendMessage),
    /// Cache manager internal messages
    CacheManager(CacheManagerMessage),
    /// Offline indicator internal messages
    OfflineIndicator(OfflineIndicatorMessage),
    /// Timeline internal messages
    Timeline(TimelineMessage),
    /// Prefetcher updated with keys to prefetch
    PrefetcherUpdated(Vec<String>),
    /// Toggle settings panel
    ToggleSettings,
    /// Update cache size from settings
    SettingsCacheSizeChanged(u64),
    /// Update default moment from settings
    SettingsDefaultMomentChanged(String),
    /// Update polling interval from settings
    SettingsPollingIntervalChanged(u64),
    /// Play/Pause toggle
    PlayPause,
    /// Step backward
    StepBackward,
    /// Step forward
    StepForward,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Pan in a direction
    Pan(PanDirection),
    /// Keyboard event handler
    Keyboard(iced::keyboard::Key),
}

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

/// Handle messages and update state
fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::StationSelector(selector_message) => {
            state.station_selector.update(selector_message);
        }
        Message::MomentSwitcher(switcher_message) => {
            state.moment_switcher.update(switcher_message);
            // Sync color legend with the selected moment
            let selected_moment = state.moment_switcher.selected_moment();
            let radar_moment = match selected_moment {
                moment_switcher::Moment::REF => RadarMoment::Reflectivity,
                moment_switcher::Moment::VEL => RadarMoment::Velocity,
                moment_switcher::Moment::SW => RadarMoment::SpectrumWidth,
                moment_switcher::Moment::ZDR => RadarMoment::Zdr,
                moment_switcher::Moment::CC => RadarMoment::Cc,
                moment_switcher::Moment::KDP => RadarMoment::Kdp,
            };
            state.color_legend.set_moment(radar_moment);
        }
        Message::ElevationTiltSelector(selector_message) => {
            state.elevation_tilt_selector.update(selector_message);
        }
        Message::ColorLegend(legend_message) => {
            state.color_legend.update(legend_message);
        }
        Message::CacheManager(cache_message) => {
            state.cache_manager.update(cache_message);
        }
        Message::OfflineIndicator(indicator_message) => {
            state.offline_indicator.update(indicator_message);
        }
        Message::Timeline(timeline_message) => {
            state.timeline.update(timeline_message);
            // Sync prefetcher after timeline updates
            state.sync_prefetcher_with_timeline();
            let prediction = state.prefetcher.predict();
            if !prediction.keys.is_empty() {
                // Keys are available for prefetching
                // In a full implementation, this would trigger async fetches
                // For now, just have the message available for testing
            }
        }
        Message::ToggleSettings => {
            state.show_settings = !state.show_settings;
        }
        Message::SettingsCacheSizeChanged(size) => {
            state.config.cache_size_mb = size;
            if let Err(e) = state.config.save() {
                eprintln!("Failed to save config: {}", e);
            }
        }
        Message::SettingsDefaultMomentChanged(moment) => {
            state.config.default_moment = moment;
            if let Err(e) = state.config.save() {
                eprintln!("Failed to save config: {}", e);
            }
        }
        Message::SettingsPollingIntervalChanged(interval) => {
            state.config.polling_interval_seconds = interval;
            if let Err(e) = state.config.save() {
                eprintln!("Failed to save config: {}", e);
            }
        }
        Message::PrefetcherUpdated(keys) => {
            // Handle prefetcher updated - keys are available for prefetching
            // This would trigger async fetches in a full implementation
            let _ = keys;
        }
        Message::PlayPause => {
            state.timeline.update(TimelineMessage::PlayPauseToggled);
            state.sync_prefetcher_with_timeline();
            let prediction = state.prefetcher.predict();
            if !prediction.keys.is_empty() {
                // Prefetch keys available
            }
        }
        Message::StepBackward => {
            state.timeline.update(TimelineMessage::StepBackward);
            state.sync_prefetcher_with_timeline();
            let prediction = state.prefetcher.predict();
            if !prediction.keys.is_empty() {
                // Prefetch keys available
            }
        }
        Message::StepForward => {
            state.timeline.update(TimelineMessage::StepForward);
            state.sync_prefetcher_with_timeline();
            let prediction = state.prefetcher.predict();
            if !prediction.keys.is_empty() {
                // Prefetch keys available
            }
        }
        Message::ZoomIn => {
            if state.zoom_level < 5 {
                state.zoom_level += 1;
                println!("Zoom in: level {}", state.zoom_level);
            }
        }
        Message::ZoomOut => {
            if state.zoom_level > -3 {
                state.zoom_level -= 1;
                println!("Zoom out: level {}", state.zoom_level);
            }
        }
        Message::Pan(direction) => {
            let (dx, dy) = match direction {
                PanDirection::Up => (0, -10),
                PanDirection::Down => (0, 10),
                PanDirection::Left => (-10, 0),
                PanDirection::Right => (10, 0),
            };
            state.pan_offset = (state.pan_offset.0 + dx, state.pan_offset.1 + dy);
            println!(
                "Pan: offset ({}, {})",
                state.pan_offset.0, state.pan_offset.1
            );
        }
        Message::Keyboard(key) => {
            // Handle keyboard shortcuts
            if let iced::keyboard::Key::Character(c) = key {
                if c.as_str() == " " {
                    // Toggle play/pause via Timeline
                    state.timeline.update(TimelineMessage::PlayPauseToggled);
                }
            }
        }
    }
    Task::none()
}

/// Get the application theme
fn theme(_state: &State) -> Theme {
    Theme::Dark
}

/// Initialize the application state with async cache setup
fn initialize() -> (State, Task<Message>) {
    // Load configuration
    let config = config::AppConfig::load_or_default();

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
