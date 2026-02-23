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

/// Speed threshold constants for adaptive prefetching.
///
/// - 0-1 scans/sec: Use `prefetch_count` as-is
/// - 1-2 scans/sec: Multiply by 1.5x (round up)
/// - >2 scans/sec: Multiply by 2x (round up)
const SPEED_THRESHOLD_LOW: f64 = 1.0;
const SPEED_THRESHOLD_MEDIUM: f64 = 2.0;

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

/// Manages predictive prefetching of radar scans based on playback state.
///
/// The `Prefetcher` analyzes the current playback state and predicts which
/// scans should be prefetched to ensure smooth playback. It supports both
/// forward and backward playback directions.
///
/// # Example
///
/// ```rust
/// use tempest_fetch::prefetch::{Prefetcher, PrefetchConfig, PlaybackState, PlaybackDirection};
///
/// // Create a prefetcher with default config
/// let mut prefetcher = Prefetcher::with_default_config();
///
/// // Set available scan keys
/// prefetcher.set_available_scans(vec![
///     "scan_001".to_string(),
///     "scan_002".to_string(),
///     "scan_003".to_string(),
/// ]);
///
/// // Update playback state to start playing forward
/// prefetcher.update_playback_state(PlaybackState {
///     current_index: 0,
///     total_scans: 3,
///     direction: PlaybackDirection::Forward,
///     speed: 1.0,
///     last_update: chrono::Utc::now(),
/// });
///
/// // Get prediction for which scans to prefetch
/// let prediction = prefetcher.predict();
/// assert_eq!(prediction.keys.len(), 2); // prefetch_count default is 5, but only 2 available forward
/// ```
#[derive(Debug)]
pub struct Prefetcher {
    config: PrefetchConfig,
    state: PlaybackState,
    /// Cache keys for available scans (populated from scan metadata)
    available_keys: Vec<String>,
}

impl Prefetcher {
    /// Create a new Prefetcher with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The prefetch configuration
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_fetch::prefetch::{Prefetcher, PrefetchConfig};
    ///
    /// let config = PrefetchConfig {
    ///     prefetch_count: 3,
    ///     enabled: true,
    /// };
    /// let prefetcher = Prefetcher::new(config);
    /// ```
    pub fn new(config: PrefetchConfig) -> Self {
        Self {
            config,
            state: PlaybackState::default(),
            available_keys: Vec::new(),
        }
    }

    /// Create a Prefetcher with default configuration.
    ///
    /// This is a convenience constructor that uses `PrefetchConfig::default()`,
    /// which enables prefetching with a default prefetch count of 5.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_fetch::prefetch::Prefetcher;
    ///
    /// let prefetcher = Prefetcher::with_default_config();
    /// let prediction = prefetcher.predict();
    /// assert!(prediction.keys.is_empty()); // No available scans set yet
    /// ```
    pub fn with_default_config() -> Self {
        Self::new(PrefetchConfig::default())
    }

    /// Update the list of available scan cache keys.
    ///
    /// This should be called when new scan metadata is available to inform
    /// the prefetcher which scans can be prefetched.
    ///
    /// # Arguments
    ///
    /// * `keys` - Vector of cache keys for available scans
    pub fn set_available_scans(&mut self, keys: Vec<String>) {
        self.available_keys = keys;
        self.state.total_scans = self.available_keys.len();
    }

    /// Update playback state.
    ///
    /// Called by the UI when the user navigates through the timeline.
    /// This updates the internal state used for making predictions.
    ///
    /// # Arguments
    ///
    /// * `state` - The current playback state
    pub fn update_playback_state(&mut self, state: PlaybackState) {
        self.state = state;
    }

    /// Get current playback state.
    ///
    /// Returns a reference to the current playback state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_fetch::prefetch::Prefetcher;
    ///
    /// let prefetcher = Prefetcher::with_default_config();
    /// let state = prefetcher.get_playback_state();
    /// assert_eq!(state.current_index, 0);
    /// ```
    pub fn get_playback_state(&self) -> &PlaybackState {
        &self.state
    }

    /// Get a mutable reference to the current playback state.
    ///
    /// This allows direct modification of playback state for efficiency
    /// when only certain fields need to be updated.
    pub fn get_playback_state_mut(&mut self) -> &mut PlaybackState {
        &mut self.state
    }

    /// Generate a prediction of which scans should be prefetched.
    ///
    /// This is the core predictive logic. It analyzes the current playback
    /// state and returns a `PrefetchPrediction` containing the cache keys
    /// that should be prefetched.
    ///
    /// The prediction algorithm:
    /// - If prefetching is disabled, returns an empty prediction
    /// - If paused or speed is 0, returns an empty prediction
    /// - If playing forward, returns the next N scans (adjusted by speed)
    /// - If playing backward, returns the previous N scans (adjusted by speed)
    ///
    /// # Example
    ///
    /// ```rust
    /// use tempest_fetch::prefetch::{Prefetcher, PlaybackState, PlaybackDirection};
    ///
    /// let mut prefetcher = Prefetcher::with_default_config();
    /// prefetcher.set_available_scans(vec![
    ///     "scan_0".to_string(),
    ///     "scan_1".to_string(),
    ///     "scan_2".to_string(),
    ///     "scan_3".to_string(),
    ///     "scan_4".to_string(),
    /// ]);
    ///
    /// // When playing forward from index 1
    /// prefetcher.update_playback_state(PlaybackState {
    ///     current_index: 1,
    ///     total_scans: 5,
    ///     direction: PlaybackDirection::Forward,
    ///     speed: 1.0,
    ///     last_update: chrono::Utc::now(),
    /// });
    ///
    /// let prediction = prefetcher.predict();
    /// assert_eq!(prediction.keys, vec!["scan_2".to_string(), "scan_3".to_string(), "scan_4".to_string()]);
    /// ```
    pub fn predict(&self) -> PrefetchPrediction {
        // If disabled, return empty prediction
        if !self.config.enabled {
            return PrefetchPrediction {
                keys: Vec::new(),
                direction: PlaybackDirection::Paused,
            };
        }

        // If paused or speed is 0, return empty prediction
        if self.state.direction == PlaybackDirection::Paused || self.state.speed <= 0.0 {
            return PrefetchPrediction {
                keys: Vec::new(),
                direction: PlaybackDirection::Paused,
            };
        }

        match self.state.direction {
            PlaybackDirection::Paused => PrefetchPrediction {
                keys: Vec::new(),
                direction: PlaybackDirection::Paused,
            },
            PlaybackDirection::Forward => self.predict_forward(),
            PlaybackDirection::Backward => self.predict_backward(),
        }
    }

    /// Predict scans to prefetch when playing forward.
    ///
    /// Returns the next N scans starting from the current position + 1,
    /// where N is adjusted based on playback speed.
    /// Uses saturating arithmetic to avoid panics at boundaries.
    fn predict_forward(&self) -> PrefetchPrediction {
        let start = self.state.current_index.saturating_add(1);
        let count = self.calculate_adjusted_prefetch_count();

        let keys: Vec<String> = self
            .available_keys
            .iter()
            .skip(start)
            .take(count)
            .cloned()
            .collect();

        PrefetchPrediction {
            keys,
            direction: PlaybackDirection::Forward,
        }
    }

    /// Predict scans to prefetch when playing backward.
    ///
    /// Returns the previous N scans ending at current position - 1,
    /// where N is adjusted based on playback speed.
    /// Uses saturating arithmetic to avoid panics at boundaries.
    fn predict_backward(&self) -> PrefetchPrediction {
        // Calculate end index (exclusive), ensuring it doesn't go below 0
        let end = self.state.current_index.saturating_sub(1);
        let count = self.calculate_adjusted_prefetch_count();

        // Calculate start index
        let start = end.saturating_sub(count);

        // Get the keys in the range [start, end)
        let keys: Vec<String> = self
            .available_keys
            .iter()
            .skip(start)
            .take(end - start)
            .cloned()
            .collect();

        PrefetchPrediction {
            keys,
            direction: PlaybackDirection::Backward,
        }
    }

    /// Calculate the adjusted prefetch count based on playback speed.
    ///
    /// Speed thresholds:
    /// - 0-1 scans/sec: Use `prefetch_count` as-is (current behavior)
    /// - 1-2 scans/sec: Multiply `prefetch_count` by 1.5x (round up)
    /// - >2 scans/sec: Multiply `prefetch_count` by 2x (round up)
    ///
    /// Uses saturating arithmetic to avoid overflow.
    fn calculate_adjusted_prefetch_count(&self) -> usize {
        let base_count = self.config.prefetch_count;
        let speed = self.state.speed;

        if speed <= SPEED_THRESHOLD_LOW {
            base_count
        } else if speed <= SPEED_THRESHOLD_MEDIUM {
            // 1.5x multiplier, rounded up
            ((base_count as f64) * 1.5).ceil() as usize
        } else {
            // 2x multiplier
            base_count.saturating_mul(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playback_direction_default() {
        let direction = PlaybackDirection::default();
        assert_eq!(direction, PlaybackDirection::Forward);
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

    // ==================== Prefetcher Behavior Tests ====================

    /// Test: Forward prediction returns next N scans
    #[test]
    fn test_forward_prediction_returns_next_scans() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
            "scan_03".to_string(),
            "scan_04".to_string(),
            "scan_05".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 0,
            total_scans: 6,
            direction: PlaybackDirection::Forward,
            speed: 1.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert_eq!(
            prediction.direction,
            PlaybackDirection::Forward,
            "Prediction direction should be Forward"
        );
        assert_eq!(
            prediction.keys.len(),
            5,
            "Default prefetch_count is 5, so should return 5 keys"
        );
        assert_eq!(
            prediction.keys[0], "scan_01",
            "First prefetched key should be the next scan after current"
        );
        assert_eq!(
            prediction.keys[4], "scan_05",
            "Last prefetched key should be at index current + 5"
        );
    }

    /// Test: Backward prediction returns previous N scans
    #[test]
    fn test_backward_prediction_returns_previous_scans() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
            "scan_03".to_string(),
            "scan_04".to_string(),
            "scan_05".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 6,
            direction: PlaybackDirection::Backward,
            speed: 1.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert_eq!(
            prediction.direction,
            PlaybackDirection::Backward,
            "Prediction direction should be Backward"
        );
        // At index 5, there are only 4 previous scans available (indices 0-4)
        assert_eq!(
            prediction.keys.len(),
            4,
            "Should return available previous scans (4 before index 5)"
        );
        assert_eq!(
            prediction.keys[0], "scan_00",
            "First prefetched key should be at the start when playing backward from end"
        );
        assert_eq!(
            prediction.keys[3], "scan_03",
            "Last prefetched key should be at index current - 2"
        );
    }

    /// Test: Paused direction returns empty prediction
    #[test]
    fn test_paused_returns_empty_prediction() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 1,
            total_scans: 3,
            direction: PlaybackDirection::Paused,
            speed: 0.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert_eq!(
            prediction.direction,
            PlaybackDirection::Paused,
            "Prediction direction should be Paused"
        );
        assert!(
            prediction.keys.is_empty(),
            "Paused direction should return empty prediction"
        );
    }

    /// Test: Disabled config returns empty prediction
    #[test]
    fn test_disabled_config_returns_empty_prediction() {
        let config = PrefetchConfig {
            enabled: false,
            prefetch_count: 5,
        };
        let mut prefetcher = Prefetcher::new(config);
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 0,
            total_scans: 3,
            direction: PlaybackDirection::Forward,
            speed: 1.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert!(
            prediction.keys.is_empty(),
            "Disabled config should return empty prediction"
        );
    }

    /// Test: Edge case - at start of timeline backward
    #[test]
    fn test_backward_at_start_returns_empty() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
            "scan_03".to_string(),
            "scan_04".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        // Playing backward when at index 0 (start of timeline)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 0,
            total_scans: 5,
            direction: PlaybackDirection::Backward,
            speed: 1.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert!(
            prediction.keys.is_empty(),
            "Backward prediction at start of timeline should return empty"
        );
    }

    /// Test: Edge case - at end of timeline forward
    #[test]
    fn test_forward_at_end_returns_available() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
            "scan_03".to_string(),
            "scan_04".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        // Playing forward when at the last index
        prefetcher.update_playback_state(PlaybackState {
            current_index: 4, // Last index (total_scans - 1)
            total_scans: 5,
            direction: PlaybackDirection::Forward,
            speed: 1.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert!(
            prediction.keys.is_empty(),
            "Forward prediction at end of timeline should return empty"
        );
    }

    /// Test: Custom prefetch_count is respected
    #[test]
    fn test_custom_prefetch_count() {
        let config = PrefetchConfig {
            enabled: true,
            prefetch_count: 2,
        };
        let mut prefetcher = Prefetcher::new(config);
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
            "scan_03".to_string(),
            "scan_04".to_string(),
            "scan_05".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 0,
            total_scans: 6,
            direction: PlaybackDirection::Forward,
            speed: 1.0,
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert_eq!(
            prediction.keys.len(),
            2,
            "Custom prefetch_count of 2 should be respected"
        );
        assert_eq!(
            prediction.keys[0], "scan_01",
            "First prefetched key should be next after current"
        );
        assert_eq!(
            prediction.keys[1], "scan_02",
            "Second prefetched key should be current + 2"
        );
    }

    // ==================== Speed-Based Prefetch Tests ====================

    /// Test: Speed = 0 returns empty prediction (like paused)
    #[test]
    fn test_speed_zero_returns_empty() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys = vec![
            "scan_00".to_string(),
            "scan_01".to_string(),
            "scan_02".to_string(),
            "scan_03".to_string(),
            "scan_04".to_string(),
            "scan_05".to_string(),
        ];
        prefetcher.set_available_scans(keys);

        // Even with Forward direction, speed 0 means we don't prefetch
        prefetcher.update_playback_state(PlaybackState {
            current_index: 0,
            total_scans: 6,
            direction: PlaybackDirection::Forward,
            speed: 0.0, // Paused/zero speed
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        assert!(
            prediction.keys.is_empty(),
            "Speed 0 should return empty prediction"
        );
    }

    /// Test: Higher speed results in more prefetched scans (forward)
    #[test]
    fn test_higher_speed_results_in_more_prefetch_forward() {
        let mut prefetcher = Prefetcher::with_default_config();
        // Need plenty of scans to see the difference
        let keys: Vec<String> = (0..20).map(|i| format!("scan_{:02}", i)).collect();
        prefetcher.set_available_scans(keys);

        // Test speed 0-1: should use prefetch_count as-is (5)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 0.5,
            last_update: Utc::now(),
        });
        let prediction_low = prefetcher.predict();
        assert_eq!(
            prediction_low.keys.len(),
            5,
            "Speed 0.5 should prefetch 5 scans (base count)"
        );

        // Test speed 1-2: should use 1.5x (round up = 8)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 1.5,
            last_update: Utc::now(),
        });
        let prediction_medium = prefetcher.predict();
        assert_eq!(
            prediction_medium.keys.len(),
            8,
            "Speed 1.5 should prefetch 8 scans (5 * 1.5 = 7.5, rounded up to 8)"
        );

        // Test speed >2: should use 2x (10)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 3.0,
            last_update: Utc::now(),
        });
        let prediction_high = prefetcher.predict();
        assert_eq!(
            prediction_high.keys.len(),
            10,
            "Speed 3.0 should prefetch 10 scans (5 * 2)"
        );
    }

    /// Test: Higher speed results in more prefetched scans (backward)
    #[test]
    fn test_higher_speed_results_in_more_prefetch_backward() {
        let mut prefetcher = Prefetcher::with_default_config();
        // Need plenty of scans to see the difference
        let keys: Vec<String> = (0..20).map(|i| format!("scan_{:02}", i)).collect();
        prefetcher.set_available_scans(keys);

        // Test speed 0-1: should use prefetch_count as-is (5)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 15,
            total_scans: 20,
            direction: PlaybackDirection::Backward,
            speed: 0.5,
            last_update: Utc::now(),
        });
        let prediction_low = prefetcher.predict();
        assert_eq!(
            prediction_low.keys.len(),
            5,
            "Speed 0.5 should prefetch 5 scans (base count)"
        );

        // Test speed 1-2: should use 1.5x (round up = 8)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 15,
            total_scans: 20,
            direction: PlaybackDirection::Backward,
            speed: 1.5,
            last_update: Utc::now(),
        });
        let prediction_medium = prefetcher.predict();
        assert_eq!(
            prediction_medium.keys.len(),
            8,
            "Speed 1.5 should prefetch 8 scans (5 * 1.5 = 7.5, rounded up to 8)"
        );

        // Test speed >2: should use 2x (10)
        prefetcher.update_playback_state(PlaybackState {
            current_index: 15,
            total_scans: 20,
            direction: PlaybackDirection::Backward,
            speed: 3.0,
            last_update: Utc::now(),
        });
        let prediction_high = prefetcher.predict();
        assert_eq!(
            prediction_high.keys.len(),
            10,
            "Speed 3.0 should prefetch 10 scans (5 * 2)"
        );
    }

    /// Test: Boundary condition - speed exactly at threshold 1.0
    #[test]
    fn test_speed_at_threshold_one() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys: Vec<String> = (0..20).map(|i| format!("scan_{:02}", i)).collect();
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 1.0, // Exactly at threshold
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        // Speed 1.0 should use base count (<= 1.0)
        assert_eq!(
            prediction.keys.len(),
            5,
            "Speed 1.0 should prefetch 5 scans (base count, <= threshold)"
        );
    }

    /// Test: Boundary condition - speed exactly at threshold 2.0
    #[test]
    fn test_speed_at_threshold_two() {
        let mut prefetcher = Prefetcher::with_default_config();
        let keys: Vec<String> = (0..20).map(|i| format!("scan_{:02}", i)).collect();
        prefetcher.set_available_scans(keys);

        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 2.0, // Exactly at threshold
            last_update: Utc::now(),
        });

        let prediction = prefetcher.predict();
        // Speed 2.0 should use 1.5x (<= 2.0)
        assert_eq!(
            prediction.keys.len(),
            8,
            "Speed 2.0 should prefetch 8 scans (1.5x, <= threshold)"
        );
    }

    /// Test: Custom prefetch_count works with speed adjustment
    #[test]
    fn test_custom_prefetch_count_with_speed() {
        let config = PrefetchConfig {
            enabled: true,
            prefetch_count: 4, // Base count of 4
        };
        let mut prefetcher = Prefetcher::new(config);
        let keys: Vec<String> = (0..20).map(|i| format!("scan_{:02}", i)).collect();
        prefetcher.set_available_scans(keys);

        // Speed 0-1: should use 4
        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 0.5,
            last_update: Utc::now(),
        });
        let prediction_low = prefetcher.predict();
        assert_eq!(prediction_low.keys.len(), 4);

        // Speed 1-2: should use 4 * 1.5 = 6
        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 1.5,
            last_update: Utc::now(),
        });
        let prediction_medium = prefetcher.predict();
        assert_eq!(prediction_medium.keys.len(), 6);

        // Speed >2: should use 4 * 2 = 8
        prefetcher.update_playback_state(PlaybackState {
            current_index: 5,
            total_scans: 20,
            direction: PlaybackDirection::Forward,
            speed: 3.0,
            last_update: Utc::now(),
        });
        let prediction_high = prefetcher.predict();
        assert_eq!(prediction_high.keys.len(), 8);
    }
}
