//! Timeline UI Component
//!
//! A widget for navigating through radar scan times with playback controls.

use chrono::{DateTime, Utc};
use iced::widget::{button, container, row, text};
use iced::Element;

use crate::colors;
use crate::spacing;

/// Valid playback speeds
const VALID_SPEEDS: [u32; 4] = [1, 2, 5, 10];

/// Valid time range options in hours
const VALID_TIME_RANGES: [u32; 3] = [1, 6, 24];

/// Messages produced by the Timeline component
#[derive(Debug, Clone)]
pub enum TimelineMessage {
    /// New scan times loaded from data source
    ScanTimesUpdated(Vec<DateTime<Utc>>),
    /// User clicked on timeline at position (0.0-1.0)
    TimelineClicked(f32),
    /// User is dragging scrubber to position (0.0-1.0)
    TimelineDragged(f32),
    /// Play/pause button pressed
    PlayPauseToggled,
    /// Playback speed changed (1, 2, 5, 10)
    SpeedChanged(u32),
    /// Loop mode toggled
    LoopToggled,
    /// Time range preset changed (1, 6, 24 hours)
    TimeRangeChanged(u32),
    /// Step forward to next scan
    StepForward,
    /// Step backward to previous scan
    StepBackward,
}

/// A component for timeline navigation and playback control
///
/// Displays a timeline scrubber with play/pause controls for navigating
/// through available radar scan times.
#[derive(Debug, Clone)]
pub struct TimelineState {
    /// Available radar scan timestamps
    scan_times: Vec<DateTime<Utc>>,
    /// Currently selected scan index
    current_index: usize,
    /// Whether playback is active
    is_playing: bool,
    /// Playback speed multiplier (1, 2, 5, 10)
    playback_speed: u32,
    /// Whether to loop playback when reaching end
    is_looping: bool,
    /// Selected time range in hours (1, 6, 24)
    time_range_hours: u32,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self::new()
    }
}

impl TimelineState {
    /// Creates a new TimelineState with default values
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            scan_times: Vec::new(),
            current_index: 0,
            is_playing: false,
            playback_speed: 1,
            is_looping: false,
            time_range_hours: 1,
        }
    }

    /// Returns the currently selected scan timestamp
    #[must_use]
    #[allow(dead_code)]
    pub fn current_time(&self) -> Option<DateTime<Utc>> {
        self.scan_times.get(self.current_index).copied()
    }

    /// Returns the current scan index
    #[must_use]
    #[allow(dead_code)]
    pub fn current_index(&self) -> usize {
        self.current_index
    }

    /// Returns whether playback is active
    #[must_use]
    #[allow(dead_code)]
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Returns the playback speed
    #[must_use]
    #[allow(dead_code)]
    pub fn playback_speed(&self) -> u32 {
        self.playback_speed
    }

    /// Returns whether looping is enabled
    #[must_use]
    #[allow(dead_code)]
    pub fn is_looping(&self) -> bool {
        self.is_looping
    }

    /// Returns the time range in hours
    #[must_use]
    #[allow(dead_code)]
    pub fn time_range_hours(&self) -> u32 {
        self.time_range_hours
    }

    /// Returns the total number of available scans
    #[must_use]
    #[allow(dead_code)]
    pub fn scan_count(&self) -> usize {
        self.scan_times.len()
    }

    /// Returns a reference to all scan times
    #[must_use]
    #[allow(dead_code)]
    pub fn scan_times(&self) -> &[DateTime<Utc>] {
        &self.scan_times
    }

    /// Validates that a speed is valid (1, 2, 5, 10)
    fn is_valid_speed(speed: u32) -> bool {
        VALID_SPEEDS.contains(&speed)
    }

    /// Validates that a time range is valid (1, 6, 24)
    fn is_valid_time_range(hours: u32) -> bool {
        VALID_TIME_RANGES.contains(&hours)
    }

    /// Handles incoming messages
    pub fn update(&mut self, message: TimelineMessage) {
        match message {
            TimelineMessage::ScanTimesUpdated(times) => {
                self.scan_times = times;
                // Clamp current index to valid range
                if self.current_index >= self.scan_times.len() {
                    self.current_index = self.scan_times.len().saturating_sub(1);
                }
            }
            TimelineMessage::TimelineClicked(position) => {
                if !self.scan_times.is_empty() {
                    // Clamp position to valid range
                    let pos = position.clamp(0.0, 1.0);
                    let new_index = (pos * (self.scan_times.len() - 1) as f32).round() as usize;
                    self.current_index = new_index.min(self.scan_times.len() - 1);
                }
            }
            TimelineMessage::TimelineDragged(position) => {
                if !self.scan_times.is_empty() {
                    // Clamp position to valid range
                    let pos = position.clamp(0.0, 1.0);
                    let new_index = (pos * (self.scan_times.len() - 1) as f32).round() as usize;
                    self.current_index = new_index.min(self.scan_times.len() - 1);
                }
            }
            TimelineMessage::PlayPauseToggled => {
                self.is_playing = !self.is_playing;
            }
            TimelineMessage::SpeedChanged(speed) => {
                if Self::is_valid_speed(speed) {
                    self.playback_speed = speed;
                }
            }
            TimelineMessage::LoopToggled => {
                self.is_looping = !self.is_looping;
            }
            TimelineMessage::TimeRangeChanged(hours) => {
                if Self::is_valid_time_range(hours) {
                    self.time_range_hours = hours;
                }
            }
            TimelineMessage::StepForward => {
                if !self.scan_times.is_empty() {
                    if self.current_index < self.scan_times.len() - 1 {
                        self.current_index += 1;
                    } else if self.is_looping {
                        // Loop back to beginning
                        self.current_index = 0;
                    }
                }
            }
            TimelineMessage::StepBackward => {
                if !self.scan_times.is_empty() {
                    if self.current_index > 0 {
                        self.current_index -= 1;
                    } else if self.is_looping {
                        // Loop to end
                        self.current_index = self.scan_times.len() - 1;
                    }
                }
            }
        }
    }

    /// Returns the view for this component
    ///
    /// Renders a horizontal timeline bar with tick marks for each scan time,
    /// current position indicator, playback controls, and metadata display.
    pub fn view(&self) -> Element<'_, TimelineMessage> {
        // Styling constants - use semantic colors
        let accent_color = colors::accent::PRIMARY;
        let dark_bg_light = colors::surface::BG_PRIMARY;
        let text_gray = colors::text::SECONDARY;

        // Calculate position (0.0 to 1.0) for current scan
        let position = if self.scan_times.is_empty() {
            0.0
        } else {
            self.current_index as f32 / (self.scan_times.len() - 1).max(1) as f32
        };

        // Build metadata display
        let current_time_str = if let Some(time) = self.current_time() {
            time.format("%Y-%m-%d %H:%M UTC").to_string()
        } else {
            "No scan loaded".to_string()
        };

        let scan_count_str = if self.scan_times.is_empty() {
            "No scans".to_string()
        } else {
            format!(
                "{} of {} scans",
                self.current_index + 1,
                self.scan_times.len()
            )
        };

        // Build the timeline bar using a custom interactive widget
        let timeline_bar = self.build_timeline_bar(position, accent_color, dark_bg_light);

        // Build playback controls
        let playback_controls = self.build_playback_controls(accent_color, &text_gray);

        // Build time range selection buttons
        let time_range_controls = self.build_time_range_controls(accent_color, &text_gray);

        // Build the header with current time and metadata
        let header = row![
            text("RADAR SCANS")
                .size(14)
                .font(iced::font::Font::MONOSPACE),
            text("•").size(14),
            text(current_time_str)
                .size(16)
                .font(iced::font::Font::MONOSPACE),
        ]
        .spacing(spacing::MD)
        .align_y(iced::Alignment::Center);

        // Build footer with scan count and VCP info
        let footer = row![
            text(scan_count_str).size(12),
            text("•").size(12),
            text(format!("Range: {}h", self.time_range_hours)).size(12),
        ]
        .spacing(spacing::MD)
        .align_y(iced::Alignment::Center);

        // Assemble the full layout with controls row
        let controls_row = row![playback_controls, time_range_controls]
            .spacing(spacing::LG)
            .align_y(iced::Alignment::Center);

        let content = container(
            iced::widget::column![header, timeline_bar, controls_row, footer]
                .spacing(spacing::SM)
                .align_x(iced::Alignment::Start),
        )
        .padding(spacing::BASE)
        .width(iced::Length::Fill)
        .into();

        content
    }

    /// Builds playback control buttons (play/pause, speed, loop)
    fn build_playback_controls(
        &self,
        _accent_color: iced::Color,
        _text_color: &iced::Color,
    ) -> Element<'_, TimelineMessage> {
        // Play/Pause button with icon
        let play_pause_text = if self.is_playing {
            "⏸ Pause"
        } else {
            "▶ Play"
        };
        let play_pause_btn = button(
            text(play_pause_text)
                .size(14)
                .font(iced::font::Font::MONOSPACE),
        )
        .on_press(TimelineMessage::PlayPauseToggled)
        .padding(spacing::SM);

        // Step backward button
        let step_back_btn = button(text("⏮").size(14))
            .on_press(TimelineMessage::StepBackward)
            .padding(spacing::SM);

        // Step forward button
        let step_forward_btn = button(text("⏭").size(14))
            .on_press(TimelineMessage::StepForward)
            .padding(spacing::SM);

        // Speed label
        let speed_label = text("Speed:").size(12).font(iced::font::Font::MONOSPACE);

        // Build speed buttons row
        let mut speed_buttons_row = row![].spacing(spacing::XXS);
        for &speed in &VALID_SPEEDS {
            let _is_selected = self.playback_speed == speed;
            let btn = button(text(format!("{}x", speed)).size(11))
                .on_press(TimelineMessage::SpeedChanged(speed))
                .padding(spacing::XS);
            speed_buttons_row = speed_buttons_row.push(btn);
        }

        // Loop toggle button
        let loop_text = if self.is_looping {
            "🔁 Loop: On"
        } else {
            "🔁 Loop"
        };
        let loop_btn = button(text(loop_text).size(12))
            .on_press(TimelineMessage::LoopToggled)
            .padding(spacing::XS);

        // Combine all controls
        row![
            play_pause_btn,
            step_back_btn,
            step_forward_btn,
            speed_label,
            speed_buttons_row,
            loop_btn,
        ]
        .spacing(spacing::SM)
        .align_y(iced::Alignment::Center)
        .into()
    }

    /// Builds time range selection buttons (1h, 6h, 24h)
    fn build_time_range_controls(
        &self,
        _accent_color: iced::Color,
        _text_color: &iced::Color,
    ) -> Element<'_, TimelineMessage> {
        let range_label = text("Range:").size(12).font(iced::font::Font::MONOSPACE);

        // Build range buttons row
        let mut range_buttons_row = row![].spacing(spacing::XXS);
        for &hours in &VALID_TIME_RANGES {
            let is_selected = self.time_range_hours == hours;
            let btn_text = if is_selected {
                format!("{}h ✓", hours)
            } else {
                format!("{}h", hours)
            };
            let btn = button(text(btn_text).size(12))
                .on_press(TimelineMessage::TimeRangeChanged(hours))
                .padding(spacing::XS);
            range_buttons_row = range_buttons_row.push(btn);
        }

        row![range_label, range_buttons_row]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center)
            .into()
    }

    /// Builds the interactive timeline bar with tick marks
    fn build_timeline_bar(
        &self,
        _position: f32,
        accent_color: iced::Color,
        _track_color: iced::Color,
    ) -> Element<'_, TimelineMessage> {
        const TIMELINE_HEIGHT: f32 = 48.0;
        const TICK_HEIGHT: f32 = 20.0;
        const LABEL_HEIGHT: f32 = 18.0;
        const TOTAL_HEIGHT: f32 = TIMELINE_HEIGHT + TICK_HEIGHT + LABEL_HEIGHT;

        if self.scan_times.is_empty() {
            // Empty state
            return container(text("No scan times available").size(14))
                .width(iced::Length::Fill)
                .height(iced::Length::Fixed(TOTAL_HEIGHT))
                .center_x(iced::Length::Fill)
                .center_y(iced::Length::Fill)
                .into();
        }

        let scan_count = self.scan_times.len();

        // Build tick marks and labels
        let mut ticks_content = row![].spacing(0).align_y(iced::Alignment::End);

        // Determine label frequency based on number of scans
        let label_interval = if scan_count <= 6 {
            1
        } else if scan_count <= 12 {
            2
        } else if scan_count <= 24 {
            4
        } else {
            (scan_count / 6).max(1)
        };

        for (i, scan_time) in self.scan_times.iter().enumerate() {
            let is_current = i == self.current_index;
            let is_near_current = (i as isize - self.current_index as isize).abs() <= 1;

            // Tick color: accent for current, lighter for near, dim for others
            let _tick_color = if is_current {
                accent_color
            } else if is_near_current {
                colors::accent::HOVER
            } else {
                colors::text::MUTED
            };

            // Build tick mark - use a simple text-based tick
            let tick = if is_current {
                // Larger, highlighted tick for current position
                container(text("|").size((TICK_HEIGHT + 8.0) as u16))
                    .height(iced::Length::Fixed(TICK_HEIGHT + 8.0))
            } else {
                container(text("|").size(TICK_HEIGHT as u16))
                    .height(iced::Length::Fixed(TICK_HEIGHT))
            };

            // Add label below tick at intervals
            let tick_with_label = if i % label_interval == 0 || is_current {
                let label_text = scan_time.format("%H:%M").to_string();
                let label = text(label_text).size(10).font(iced::font::Font::MONOSPACE);

                container(
                    iced::widget::column![tick, label]
                        .spacing(spacing::XXS)
                        .align_x(iced::Alignment::Center),
                )
                .height(iced::Length::Fixed(TICK_HEIGHT + LABEL_HEIGHT + 10.0))
            } else {
                container(tick).height(iced::Length::Fixed(TICK_HEIGHT))
            };

            // Make each tick clickable
            let tick_position = if scan_count == 1 {
                0.0
            } else {
                i as f32 / (scan_count - 1) as f32
            };

            let tick_button = button(tick_with_label)
                .on_press(TimelineMessage::TimelineClicked(tick_position))
                .padding(0);

            ticks_content = ticks_content.push(tick_button);
        }

        // Return the ticks content - without the track variable since it's unused
        container(
            iced::widget::column![
                // Tick marks
                ticks_content,
            ]
            .spacing(spacing::XS),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fixed(TOTAL_HEIGHT))
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_default_state() {
        let timeline = TimelineState::new();
        assert!(timeline.scan_times().is_empty());
        assert_eq!(timeline.current_index(), 0);
        assert!(!timeline.is_playing());
        assert_eq!(timeline.playback_speed(), 1);
        assert!(!timeline.is_looping());
        assert_eq!(timeline.time_range_hours(), 1);
    }

    #[test]
    fn test_play_pause_toggle() {
        let mut timeline = TimelineState::new();
        assert!(!timeline.is_playing());

        timeline.update(TimelineMessage::PlayPauseToggled);
        assert!(timeline.is_playing());

        timeline.update(TimelineMessage::PlayPauseToggled);
        assert!(!timeline.is_playing());
    }

    #[test]
    fn test_speed_change_valid() {
        let mut timeline = TimelineState::new();
        assert_eq!(timeline.playback_speed(), 1);

        timeline.update(TimelineMessage::SpeedChanged(2));
        assert_eq!(timeline.playback_speed(), 2);

        timeline.update(TimelineMessage::SpeedChanged(5));
        assert_eq!(timeline.playback_speed(), 5);

        timeline.update(TimelineMessage::SpeedChanged(10));
        assert_eq!(timeline.playback_speed(), 10);
    }

    #[test]
    fn test_speed_change_invalid() {
        let mut timeline = TimelineState::new();
        assert_eq!(timeline.playback_speed(), 1);

        // Invalid speeds should be ignored
        timeline.update(TimelineMessage::SpeedChanged(3));
        assert_eq!(timeline.playback_speed(), 1);

        timeline.update(TimelineMessage::SpeedChanged(0));
        assert_eq!(timeline.playback_speed(), 1);
    }

    #[test]
    fn test_loop_toggle() {
        let mut timeline = TimelineState::new();
        assert!(!timeline.is_looping());

        timeline.update(TimelineMessage::LoopToggled);
        assert!(timeline.is_looping());

        timeline.update(TimelineMessage::LoopToggled);
        assert!(!timeline.is_looping());
    }

    #[test]
    fn test_time_range_change_valid() {
        let mut timeline = TimelineState::new();
        assert_eq!(timeline.time_range_hours(), 1);

        timeline.update(TimelineMessage::TimeRangeChanged(6));
        assert_eq!(timeline.time_range_hours(), 6);

        timeline.update(TimelineMessage::TimeRangeChanged(24));
        assert_eq!(timeline.time_range_hours(), 24);
    }

    #[test]
    fn test_time_range_change_invalid() {
        let mut timeline = TimelineState::new();
        assert_eq!(timeline.time_range_hours(), 1);

        // Invalid time ranges should be ignored
        timeline.update(TimelineMessage::TimeRangeChanged(3));
        assert_eq!(timeline.time_range_hours(), 1);

        timeline.update(TimelineMessage::TimeRangeChanged(12));
        assert_eq!(timeline.time_range_hours(), 1);
    }

    #[test]
    fn test_step_forward() {
        let mut timeline = TimelineState::new();
        let times = vec![Utc::now(), Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);

        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 1);

        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 2);

        // At end, should stay at end (no loop)
        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 2);
    }

    #[test]
    fn test_step_forward_with_loop() {
        let mut timeline = TimelineState::new();
        timeline.update(TimelineMessage::LoopToggled);
        let times = vec![Utc::now(), Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);

        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 1);

        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 2);

        // At end with loop, should wrap to beginning
        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 0);
    }

    #[test]
    fn test_step_backward() {
        let mut timeline = TimelineState::new();
        let times = vec![Utc::now(), Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        timeline.update(TimelineMessage::StepForward);
        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 2);

        timeline.update(TimelineMessage::StepBackward);
        assert_eq!(timeline.current_index(), 1);

        timeline.update(TimelineMessage::StepBackward);
        assert_eq!(timeline.current_index(), 0);

        // At beginning, should stay at beginning (no loop)
        timeline.update(TimelineMessage::StepBackward);
        assert_eq!(timeline.current_index(), 0);
    }

    #[test]
    fn test_step_backward_with_loop() {
        let mut timeline = TimelineState::new();
        timeline.update(TimelineMessage::LoopToggled);
        let times = vec![Utc::now(), Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);

        // At beginning with loop, should wrap to end
        timeline.update(TimelineMessage::StepBackward);
        assert_eq!(timeline.current_index(), 2);
    }

    #[test]
    fn test_timeline_click() {
        let mut timeline = TimelineState::new();
        let times = vec![Utc::now(), Utc::now(), Utc::now(), Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);

        // Click at 0.0 should go to index 0
        timeline.update(TimelineMessage::TimelineClicked(0.0));
        assert_eq!(timeline.current_index(), 0);

        // Click at 0.5 should go to index 2 (middle of 4 intervals)
        timeline.update(TimelineMessage::TimelineClicked(0.5));
        assert_eq!(timeline.current_index(), 2);

        // Click at 1.0 should go to last index
        timeline.update(TimelineMessage::TimelineClicked(1.0));
        assert_eq!(timeline.current_index(), 4);
    }

    #[test]
    fn test_timeline_click_clamped() {
        let mut timeline = TimelineState::new();
        let times = vec![Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));

        // Positions outside 0-1 should be clamped
        timeline.update(TimelineMessage::TimelineClicked(-0.5));
        assert_eq!(timeline.current_index(), 0);

        timeline.update(TimelineMessage::TimelineClicked(1.5));
        assert_eq!(timeline.current_index(), 1);
    }

    #[test]
    fn test_scan_times_updated_clamp_index() {
        let mut timeline = TimelineState::new();
        // Start with some scans
        let times = vec![Utc::now(), Utc::now(), Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        timeline.update(TimelineMessage::StepForward);
        assert_eq!(timeline.current_index(), 1);

        // Update with fewer scans - index should be clamped
        let times = vec![Utc::now()];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);

        // Update with empty scans
        let times: Vec<DateTime<Utc>> = vec![];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);
    }
}
