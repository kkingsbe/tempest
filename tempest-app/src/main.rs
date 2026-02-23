//! Tempest - NEXRAD Weather Radar Application
//!
//! Main application entry point using iced UI framework.

mod station_selector;
mod moment_switcher;

use iced::{Sandbox, Element, Settings, Color, Size};
use station_selector::{StationSelector, StationSelectorMessage};
use moment_switcher::{MomentSwitcher, MomentSwitcherMessage};

/// Application state
#[derive(Debug, Default)]
pub struct App {
    /// Station selector component
    station_selector: StationSelector,
    /// Moment switcher component
    moment_switcher: MomentSwitcher,
}

/// Messages that can be sent to the application
#[derive(Debug, Clone)]
pub enum Message {
    /// Station selector internal messages
    StationSelector(StationSelectorMessage),
    /// Moment switcher internal messages
    MomentSwitcher(MomentSwitcherMessage),
    /// Toggle settings panel
    ToggleSettings,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Tempest - NEXRAD Weather Radar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::StationSelector(selector_message) => {
                self.station_selector.update(selector_message);
            }
            Message::MomentSwitcher(switcher_message) => {
                self.moment_switcher.update(switcher_message);
            }
            Message::ToggleSettings => {
                // Placeholder for settings toggle
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::{column, container, text};

        // Dark themed content
        let content = column![
            text("Tempest - NEXRAD Weather Radar")
                .size(28)
                .style(iced::theme::Text::Color(Color::from_rgb(0.2, 0.6, 1.0))),
            text("").size(10),
            // Moment switcher component
            self.moment_switcher.view().map(Message::MomentSwitcher),
            text("").size(15),
            // Station selector component
            self.station_selector.view().map(Message::StationSelector),
        ]
        .spacing(10)
        .padding(20);

        container(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
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
