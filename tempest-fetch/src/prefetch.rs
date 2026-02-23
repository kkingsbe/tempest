//! Prefetch types for predictive data loading.
//!
//! This module provides types for tracking playback state and predicting
//! which scans should be prefetched for optimal performance.

use chrono::{DateTime, Utc};

/// Direction of playback for predictive prefetching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackDirection {
    /// Playing forward through time (next scans)
    #[default]
    Forward,
    /// Playing backward through time (previous scans)
    Backward,
    /// Not playing, no prefetch needed
    Paused,
}

/// Configuration for the prefetcher
#[derive(Debug, Clone)]
pub struct PrefetchConfig {
    /// Number of scans to prefetch ahead/behind (3-5 recommended)
    pub prefetch_count: usize,
    /// Whether prefetching is enabled
    pub enabled: bool,
}

impl Default for PrefetchConfig {
    fn default() -> Self {
        Self {
            prefetch_count: 5,
            enabled: true,
        }
    }
}

/// State for tracking playback and making predictions
#[derive(Debug, Clone)]
pub struct PlaybackState {
    /// Current scan index in the timeline
    pub current_index: usize,
    /// Total number of available scans
    pub total_scans: usize,
    /// Current playback direction
    pub direction: PlaybackDirection,
    /// Current playback speed (scans per second, 0 = paused)
    pub speed: f64,
    /// Timestamp of last update (for speed calculation)
    pub last_update: DateTime<Utc>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            current_index: 0,
            total_scans: 0,
            direction: PlaybackDirection::Paused,
            speed: 0.0,
            last_update: Utc::now(),
        }
    }
}

/// A prediction of which scans should be prefetched
#[derive(Debug, Clone)]
pub struct PrefetchPrediction {
    /// Keys that should be prefetched
    pub keys: Vec<String>,
    /// Direction the prediction is for
    pub direction: PlaybackDirection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_direction_default() {
        let direction = PlaybackDirection::default();
        assert_eq!(direction, PlaybackDirection::Paused);
    }

    #[test]
    fn test_prefetch_config_default() {
        let config = PrefetchConfig::default();
        assert_eq!(config.prefetch_count, 5);
        assert!(config.enabled);
    }

    #[test]
    fn test_playback_state_default() {
        let state = PlaybackState::default();
        assert_eq!(state.current_index, 0);
        assert_eq!(state.total_scans, 0);
        assert_eq!(state.direction, PlaybackDirection::Paused);
        assert_eq!(state.speed, 0.0);
    }

    #[test]
    fn test_prefetch_prediction() {
        let prediction = PrefetchPrediction {
            keys: vec!["key1".to_string(), "key2".to_string()],
            direction: PlaybackDirection::Forward,
        };
        assert_eq!(prediction.keys.len(), 2);
        assert_eq!(prediction.direction, PlaybackDirection::Forward);
    }
}
