//! Configuration management for Tempest application.
//!
//! Handles loading and saving configuration from/to ~/.tempest/config.toml

use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

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
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            cache_size_mb: 1024,
            default_station: None,
            default_moment: "REF".to_string(),
            polling_interval_seconds: 60,
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
    /// If the file doesn't exist, creates a default configuration
    pub fn load() -> Self {
        if let Some(config_path) = Self::config_path() {
            if config_path.exists() {
                if let Ok(contents) = fs::read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str(&contents) {
                        return config;
                    }
                }
            }
        }

        // Return default config if loading fails
        Self::default()
    }

    /// Save configuration to ~/.tempest/config.toml
    #[allow(dead_code)]
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
