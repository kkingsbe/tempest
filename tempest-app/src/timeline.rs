//! Timeline UI Component
//!
//! A widget for navigating through radar scan times with playback controls.

use chrono::{DateTime, Utc};
use iced::widget::{container, text};
use iced::Element;

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

    /// Returns the view for this component (stub implementation)
    ///
    /// Full rendering will be implemented in subtask 2.
    pub fn view(&self) -> Element<'_, TimelineMessage> {
        // Stub: just return a simple container with placeholder text
        // Full rendering will be implemented in subtask 2
        container(text("Timeline"))
            .padding(10)
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
        let times = vec![
            Utc::now(),
            Utc::now(),
            Utc::now(),
        ];
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
        let times = vec![
            Utc::now(),
            Utc::now(),
            Utc::now(),
        ];
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
        let times = vec![
            Utc::now(),
            Utc::now(),
            Utc::now(),
        ];
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
        let times = vec![
            Utc::now(),
            Utc::now(),
            Utc::now(),
        ];
        timeline.update(TimelineMessage::ScanTimesUpdated(times));
        assert_eq!(timeline.current_index(), 0);

        // At beginning with loop, should wrap to end
        timeline.update(TimelineMessage::StepBackward);
        assert_eq!(timeline.current_index(), 2);
    }

    #[test]
    fn test_timeline_click() {
        let mut timeline = TimelineState::new();
        let times = vec![
            Utc::now(),
            Utc::now(),
            Utc::now(),
            Utc::now(),
            Utc::now(),
        ];
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
