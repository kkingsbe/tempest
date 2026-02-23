//! Tempest - NEXRAD Weather Radar Application
//!
//! Main application entry point using iced UI framework.

mod cache_manager;
mod color_legend;
mod config;
mod elevation_tilt_selector;
mod moment_switcher;
mod offline_detection;
mod offline_indicator;
mod station_selector;
mod timeline;

use cache_manager::{CacheManager, CacheManagerMessage};
use color_legend::{ColorLegend, ColorLegendMessage};
use elevation_tilt_selector::{ElevationTiltSelector, ElevationTiltSelectorMessage};
use iced::{Color, Element, Sandbox, Settings, Size};
use moment_switcher::{MomentSwitcher, MomentSwitcherMessage};
use offline_indicator::{OfflineIndicator, OfflineIndicatorMessage};
use station_selector::{StationSelector, StationSelectorMessage};
use std::sync::Arc;
use tempest_fetch::cache_default;
use tempest_render_core::color::RadarMoment;
use timeline::{TimelineMessage, TimelineState};
use tokio::sync::RwLock;

/// Application state
#[derive(Debug)]
pub struct App {
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
    /// Counter for periodic connectivity checks
    connectivity_check_counter: u32,
    /// Application configuration
    #[allow(dead_code)]
    config: config::AppConfig,
    /// Playback state
    #[allow(dead_code)]
    is_playing: bool,
    /// Current zoom level (0 = default, positive = zoomed in, negative = zoomed out)
    zoom_level: i32,
    /// Pan offset for map (x, y)
    pan_offset: (i32, i32),
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
    /// Toggle settings panel
    ToggleSettings,
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

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        // Load configuration
        let config = config::AppConfig::load();

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

        Self {
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
            connectivity_check_counter: 0,
            config,
            is_playing: false,
            zoom_level: 0,
            pan_offset: (0, 0),
        }
    }

    fn title(&self) -> String {
        let mut title = String::from("Tempest");

        // Add station name if selected
        let station = self.station_selector.selected_station();
        if let Some(station) = station {
            title.push_str(&format!(" - {}", station.id));

            // Add moment if selected
            let moment = self.moment_switcher.selected_moment();
            let moment_str = match moment {
                moment_switcher::Moment::REF => "Reflectivity",
                moment_switcher::Moment::VEL => "Velocity",
                moment_switcher::Moment::SW => "Spectrum Width",
                moment_switcher::Moment::ZDR => "Differential Reflectivity",
                moment_switcher::Moment::CC => "Correlation Coefficient",
                moment_switcher::Moment::KDP => "Differential Phase",
            };
            title.push_str(&format!(" ({})", moment_str));
        }

        // Add offline indicator if offline
        if !self.offline_indicator.is_online() {
            title.push_str(" [OFFLINE]");
        }

        title
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StationSelector(selector_message) => {
                self.station_selector.update(selector_message);
            }
            Message::MomentSwitcher(switcher_message) => {
                self.moment_switcher.update(switcher_message);
                // Sync color legend with the selected moment
                let selected_moment = self.moment_switcher.selected_moment();
                let radar_moment = match selected_moment {
                    moment_switcher::Moment::REF => RadarMoment::Reflectivity,
                    moment_switcher::Moment::VEL => RadarMoment::Velocity,
                    moment_switcher::Moment::SW => RadarMoment::SpectrumWidth,
                    moment_switcher::Moment::ZDR => RadarMoment::Zdr,
                    moment_switcher::Moment::CC => RadarMoment::Cc,
                    moment_switcher::Moment::KDP => RadarMoment::Kdp,
                };
                self.color_legend.set_moment(radar_moment);
            }
            Message::ElevationTiltSelector(selector_message) => {
                self.elevation_tilt_selector.update(selector_message);
            }
            Message::ColorLegend(legend_message) => {
                self.color_legend.update(legend_message);
            }
            Message::CacheManager(cache_message) => {
                self.cache_manager.update(cache_message);
            }
            Message::OfflineIndicator(indicator_message) => {
                self.offline_indicator.update(indicator_message);
            }
            Message::Timeline(timeline_message) => {
                self.timeline.update(timeline_message);
            }
            Message::ToggleSettings => {
                // Placeholder for settings toggle
            }
            Message::PlayPause => {
                self.timeline.update(TimelineMessage::PlayPauseToggled);
            }
            Message::StepBackward => {
                self.timeline.update(TimelineMessage::StepBackward);
            }
            Message::StepForward => {
                self.timeline.update(TimelineMessage::StepForward);
            }
            Message::ZoomIn => {
                if self.zoom_level < 5 {
                    self.zoom_level += 1;
                    println!("Zoom in: level {}", self.zoom_level);
                }
            }
            Message::ZoomOut => {
                if self.zoom_level > -3 {
                    self.zoom_level -= 1;
                    println!("Zoom out: level {}", self.zoom_level);
                }
            }
            Message::Pan(direction) => {
                let (dx, dy) = match direction {
                    PanDirection::Up => (0, -10),
                    PanDirection::Down => (0, 10),
                    PanDirection::Left => (-10, 0),
                    PanDirection::Right => (10, 0),
                };
                self.pan_offset = (self.pan_offset.0 + dx, self.pan_offset.1 + dy);
                println!("Pan: offset ({}, {})", self.pan_offset.0, self.pan_offset.1);
            }
            Message::Keyboard(key) => {
                // Handle keyboard shortcuts
                if let iced::keyboard::Key::Character(c) = key {
                    match c.as_str() {
                        " " => {
                            // Toggle play/pause via Timeline
                            self.timeline.update(TimelineMessage::PlayPauseToggled);
                        }
                        "+" | "=" => {
                            if self.zoom_level < 5 {
                                self.zoom_level += 1;
                            }
                        }
                        "-" => {
                            if self.zoom_level > -3 {
                                self.zoom_level -= 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // Periodic connectivity check every few ticks (every ~5 seconds at 60fps)
        self.connectivity_check_counter += 1;
        if self.connectivity_check_counter >= 300 {
            self.connectivity_check_counter = 0;
            let is_online = offline_detection::is_online();
            self.offline_indicator.set_online(is_online);
        }
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::{column, container, row, text};

        // Dark themed content
        let controls = column![
            text("Tempest - NEXRAD Weather Radar")
                .size(28)
                .style(iced::theme::Text::Color(Color::from_rgb(0.2, 0.6, 1.0))),
            text("").size(10),
            // Moment switcher component
            self.moment_switcher.view().map(Message::MomentSwitcher),
            text("").size(15),
            // Elevation tilt selector component
            self.elevation_tilt_selector
                .view()
                .map(Message::ElevationTiltSelector),
            text("").size(15),
            // Station selector component
            self.station_selector.view().map(Message::StationSelector),
            text("").size(15),
            // Cache manager component
            self.cache_manager.view().map(Message::CacheManager),
            text("").size(10),
            // Keyboard shortcuts help
            text("Shortcuts: Space=Play/Pause, Arrows=Pan, +/-=Zoom")
                .size(12)
                .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5))),
        ]
        .spacing(10)
        .padding(20);

        // Main container with content
        let main_container = container(controls)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y();

        // Color legend positioned on the right side
        let legend = self.color_legend.view().map(Message::ColorLegend);

        // Offline indicator positioned at top-right
        let indicator = self.offline_indicator.view().map(Message::OfflineIndicator);

        // Timeline positioned at the bottom
        let timeline = self.timeline.view().map(Message::Timeline);

        // Combine main content, legend, indicator, and timeline using a column
        column![
            row![main_container, legend, indicator].align_items(iced::Alignment::Start),
            timeline
        ]
        .spacing(0)
        .into()
    }
}

/// Main entry point for the Tempest application
fn main() {
    // Configure window settings
    let settings = Settings {
        window: iced::window::Settings {
            size: Size::new(1200.0, 800.0),
            resizable: true,
            decorations: true,
            transparent: false,
            ..Default::default()
        },
        ..Default::default()
    };

    // Run the application
    if let Err(e) = App::run(settings) {
        eprintln!("Error running Tempest application: {}", e);
        std::process::exit(1);
    }
}
