//! Station Selector UI Component
//!
//! A widget for selecting NEXRAD radar stations with filtering and detail display.

use iced::widget::{button, column, container, scrollable, text, text_input, Column};
use iced::{Alignment, Element, Length};
use tempest_fetch::{list_all_stations, Station};

/// Maximum number of stations to show in the list
const MAX_VISIBLE_STATIONS: usize = 15;

/// Messages produced by the StationSelector component
#[derive(Debug, Clone)]
pub enum StationSelectorMessage {
    /// Filter text changed
    FilterChanged(String),
    /// Station was selected from the list
    StationSelected(Station),
}

/// A component for selecting NEXRAD radar stations
///
/// Allows users to search/filter stations by ID or name and displays
/// detailed information about the selected station.
#[derive(Debug)]
pub struct StationSelector {
    /// All available stations
    all_stations: Vec<Station>,
    /// Filtered stations based on search text
    filtered_stations: Vec<Station>,
    /// Current filter/search text
    filter_text: String,
    /// Currently selected station
    selected_station: Option<Station>,
}

impl StationSelector {
    /// Creates a new StationSelector with all available stations loaded
    pub fn new() -> Self {
        let all_stations = list_all_stations();
        let filtered_stations = all_stations.clone();

        Self {
            all_stations,
            filtered_stations,
            filter_text: String::new(),
            selected_station: None,
        }
    }

    /// Returns the currently selected station
    #[must_use]
    #[allow(dead_code)]
    pub fn selected_station(&self) -> Option<&Station> {
        self.selected_station.as_ref()
    }

    /// Updates the filter and recalculates visible stations
    fn update_filter(&mut self) {
        let query = self.filter_text.to_lowercase();

        if query.is_empty() {
            // Show all stations when no filter
            self.filtered_stations = self.all_stations.clone();
        } else {
            // Filter by ID or name (case-insensitive)
            self.filtered_stations = self
                .all_stations
                .iter()
                .filter(|s| {
                    s.id.to_lowercase().contains(&query) || s.name.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
        }

        // Limit the number of visible stations
        if self.filtered_stations.len() > MAX_VISIBLE_STATIONS {
            self.filtered_stations.truncate(MAX_VISIBLE_STATIONS);
        }
    }

    /// Handles incoming messages
    pub fn update(&mut self, message: StationSelectorMessage) {
        match message {
            StationSelectorMessage::FilterChanged(text) => {
                self.filter_text = text;
                self.update_filter();
                // Clear selection if current selection is not in filtered list
                if let Some(ref selected) = self.selected_station {
                    if !self.filtered_stations.iter().any(|s| s.id == selected.id) {
                        self.selected_station = None;
                    }
                }
            }
            StationSelectorMessage::StationSelected(station) => {
                self.selected_station = Some(station);
            }
        }
    }

    /// Returns the view for this component
    pub fn view(&self) -> Element<'_, StationSelectorMessage> {
        // Station label styling
        let label_style = iced::theme::Text::Color(iced::Color::from_rgb(0.7, 0.7, 0.7));
        let value_style = iced::theme::Text::Color(iced::Color::from_rgb(0.9, 0.9, 0.9));
        let heading_style = iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0));

        // Create the filter input
        let filter_input = text_input(
            "Filter by ID or name (e.g., KTLX, Oklahoma)",
            &self.filter_text,
        )
        .on_input(StationSelectorMessage::FilterChanged)
        .width(Length::Fill)
        .padding(8);

        // Build station list buttons
        let mut station_buttons = Column::new().spacing(4);
        for station in &self.filtered_stations {
            let is_selected = self
                .selected_station
                .as_ref()
                .is_some_and(|s| s.id == station.id);

            let btn = button(text(format!("{} - {}", station.id, station.name)).style(
                if is_selected {
                    iced::theme::Text::Color(iced::Color::from_rgb(0.2, 0.6, 1.0))
                } else {
                    iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.8, 0.8))
                },
            ))
            .on_press(StationSelectorMessage::StationSelected(station.clone()))
            .width(Length::Fill)
            .padding(8);

            station_buttons = station_buttons.push(btn);
        }

        let station_list = scrollable(station_buttons).height(Length::Fixed(300.0));

        // Station details section
        let details: Element<'_, StationSelectorMessage> =
            if let Some(ref station) = self.selected_station {
                let details_column = column![
                    text("Station Details").style(heading_style).size(16),
                    text("ID: ").style(label_style),
                    text(&station.id).style(value_style),
                    text("Name: ").style(label_style),
                    text(&station.name).style(value_style),
                    text("Location: ").style(label_style),
                    text(format!("{:.4}°, {:.4}°", station.lat, station.lon)).style(value_style),
                    text("Elevation: ").style(label_style),
                    text(format!("{:.0} m", station.elevation_m)).style(value_style),
                ]
                .spacing(4)
                .align_items(Alignment::Start);

                container(details_column).padding(15).into()
            } else {
                // No station selected - show placeholder
                text("Select a station to view details")
                    .style(label_style)
                    .into()
            };

        // Count of filtered stations
        let count_text = if self.filter_text.is_empty() {
            format!(
                "Showing all {} stations (scroll to see more)",
                self.all_stations.len()
            )
        } else {
            format!(
                "Found {} of {} stations",
                self.filtered_stations.len(),
                self.all_stations.len()
            )
        };

        // Build the main layout
        let content = column![
            text("NEXRAD Station Selection")
                .style(heading_style)
                .size(20),
            text("").size(10),
            text("Search").style(label_style).size(14),
            filter_input,
            text(&count_text)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(
                    0.5, 0.5, 0.5
                )))
                .size(12),
            text("").size(5),
            station_list,
            text("").size(15),
            details,
        ]
        .spacing(5)
        .align_items(Alignment::Start)
        .padding(20)
        .width(Length::FillPortion(1));

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Default for StationSelector {
    fn default() -> Self {
        Self::new()
    }
}
