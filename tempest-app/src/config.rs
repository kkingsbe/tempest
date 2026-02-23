//! Configuration management for Tempest application.
//!
//! Handles loading and saving configuration from/to ~/.tempest/config.toml

use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),
    #[error("Config directory not available")]
    ConfigDirUnavailable,
}

/// Application configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppConfig {
    /// Cache size in megabytes (default: 1024)
    pub cache_size_mb: u64,
    /// Default radar station to load on startup
    pub default_station: Option<String>,
    /// Default radar moment to display (default: "REF")
    pub default_moment: String,
    /// Polling interval in seconds for fetching new data (default: 60)
    pub polling_interval_seconds: u64,
    /// Window width in pixels (default: 1200.0)
    pub window_width: f32,
    /// Window height in pixels (default: 800.0)
    pub window_height: f32,
    /// Map tile source provider (default: "OpenStreetMap")
    /// Valid values: "OpenStreetMap", "Stadia", "MapTiler"
    pub map_tile_source: String,
    /// Playback speed multiplier (default: 1.0)
    /// Valid values: 1.0, 2.0, 5.0, 10.0
    pub playback_speed: f32,
    /// Radar overlay opacity (default: 0.5)
    /// Range: 0.0 to 1.0
    pub radar_overlay_opacity: f32,
    /// Velocity units for display (default: "kts")
    /// Valid values: "kts", "mps" (meters per second)
    pub velocity_units: String,
    /// Window X position (default: None)
    pub window_x: Option<f32>,
    /// Window Y position (default: None)
    pub window_y: Option<f32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            cache_size_mb: 1024,
            default_station: None,
            default_moment: "REF".to_string(),
            polling_interval_seconds: 60,
            window_width: 1200.0,
            window_height: 800.0,
            map_tile_source: "OpenStreetMap".to_string(),
            playback_speed: 1.0,
            radar_overlay_opacity: 0.5,
            velocity_units: "kts".to_string(),
            window_x: None,
            window_y: None,
        }
    }
}

impl AppConfig {
    /// Get the configuration directory path (~/.tempest)
    fn config_dir() -> Option<PathBuf> {
        ProjectDirs::from("com", "tempest", "Tempest").map(|dirs| dirs.config_dir().to_path_buf())
    }

    /// Get the configuration file path (~/.tempest/config.toml)
    fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|dir| dir.join("config.toml"))
    }

    /// Load configuration from ~/.tempest/config.toml
    /// Returns an error if the config directory is unavailable, the file cannot be read,
    /// or the TOML cannot be parsed.
    pub fn load() -> Result<Self, ConfigError> {
        let config_dir = Self::config_dir().ok_or(ConfigError::ConfigDirUnavailable)?;

        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_path = Self::config_path().ok_or(ConfigError::ConfigDirUnavailable)?;

        let contents = fs::read_to_string(&config_path)?;
        let config = toml::from_str(&contents)?;

        Ok(config)
    }

    /// Load configuration from ~/.tempest/config.toml
    /// If loading fails (missing file, parse error, etc.), returns default configuration
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    /// Save configuration to ~/.tempest/config.toml
    pub fn save(&self) -> Result<(), String> {
        let config_dir =
            Self::config_dir().ok_or_else(|| "Could not determine config directory".to_string())?;

        // Create directory if it doesn't exist
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let config_path = Self::config_path()
            .ok_or_else(|| "Could not determine config file path".to_string())?;

        let toml_string = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(&config_path, toml_string)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.cache_size_mb, 1024);
        assert_eq!(config.default_moment, "REF");
        assert_eq!(config.polling_interval_seconds, 60);
    }

    #[test]
    fn test_config_dir_exists() {
        let dir = AppConfig::config_dir();
        assert!(dir.is_some());
    }

    #[test]
    fn test_config_path_exists() {
        let path = AppConfig::config_path();
        assert!(path.is_some());
    }
}
