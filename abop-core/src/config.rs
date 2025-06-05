//! Centralized configuration management for ABOP

use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Window-specific configuration options
    pub window: WindowConfig,
    /// Theme and appearance settings
    pub theme: ThemeConfig,
    /// Directory for application data storage
    pub data_dir: PathBuf,
}

impl Config {
    /// Load configuration from file system
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to determine the configuration file path
    /// - Failed to read the configuration file
    /// - Failed to parse the TOML configuration
    /// - Failed to save the default configuration (if file doesn't exist)
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let contents = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&contents)?)
        } else {
            let default = Self::default();
            default.save()?;
            Ok(default)
        }
    }

    /// Save current configuration to file system
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to determine the configuration file path
    /// - Failed to create the parent directory
    /// - Failed to serialize the configuration to TOML
    /// - Failed to write the configuration file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, contents)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| AppError::Config("Could not find config directory".to_string()))?;
        path.push("abop-iced");
        path.push("config.toml");
        Ok(path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            data_dir: dirs::data_dir().unwrap_or_else(|| PathBuf::from("./data")),
        }
    }
}

/// Window appearance and behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Minimum window width in pixels
    pub min_width: u32,
    /// Minimum window height in pixels
    pub min_height: u32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            min_width: 800,
            min_height: 600,
        }
    }
}

use crate::models::ThemeConfig;
