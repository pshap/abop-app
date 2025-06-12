//! Centralized configuration management for ABOP
//!
//! This module provides a modular configuration system that maintains backward
//! compatibility while offering enhanced validation and organization.

use crate::error::{AppError, Result};
use crate::platform::env_utils;
use log::warn;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod app;
pub mod ui;
pub mod validation;

// Re-export for convenience
pub use app::AppConfig;
pub use ui::{UiConfig, WindowConfig};
pub use validation::{ConfigValidation, ValidationError, ValidationResult};

/// Main application configuration settings
///
/// This is the primary configuration struct that combines all configuration
/// modules while maintaining compatibility with existing TOML files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Window-specific configuration options
    pub window: WindowConfig,
    /// Theme and appearance settings
    pub theme: ThemeConfig,
    /// Directory for application data storage
    pub data_dir: PathBuf,
    /// Application-level settings (new modular structure)
    #[serde(default)]
    pub app: AppConfig,
    /// UI behavior and preferences (new modular structure)
    #[serde(default)]
    pub ui: UiConfig,
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
            let mut config: Config = toml::from_str(&contents)?;

            // Validate and auto-fix configuration
            let validation_result = config.validate_and_fix();
            if validation_result.has_errors() {
                log::warn!(
                    "Configuration validation errors: {:?}",
                    validation_result.errors
                );
            }
            if validation_result.has_warnings() {
                log::info!(
                    "Configuration validation warnings: {:?}",
                    validation_result.warnings
                );
            }

            Ok(config)
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
        // First try to get the config path from environment variable
        if let Ok(custom_config) = std::env::var("ABOP_CONFIG") {
            match env_utils::expand_path_env_vars(PathBuf::from(custom_config).as_path()) {
                Ok(expanded_path) => {
                    if expanded_path.exists() {
                        return Ok(expanded_path);
                    }
                    warn!("Configured config file does not exist: {expanded_path:?}");
                }
                Err(e) => warn!("Failed to expand config path from ABOP_CONFIG: {e}"),
            }
        }

        // Fall back to standard locations
        let base_dirs =
            directories::ProjectDirs::from("com", "abop", "abop-iced").ok_or_else(|| {
                AppError::Config("Could not determine project directories".to_string())
            })?;

        let config_dir = base_dirs.config_dir();
        std::fs::create_dir_all(config_dir)
            .map_err(|e| AppError::Config(format!("Could not create config directory: {e}")))?;

        Ok(config_dir.join("config.toml"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            data_dir: dirs::data_dir().unwrap_or_else(|| PathBuf::from("./data")),
            app: AppConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl ConfigValidation for Config {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate sub-configurations
        let window_result = self.window.validate();
        result.errors.extend(window_result.errors);
        result.warnings.extend(window_result.warnings);

        let app_result = self.app.validate();
        result.errors.extend(app_result.errors);
        result.warnings.extend(app_result.warnings);

        let ui_result = self.ui.validate();
        result.errors.extend(ui_result.errors);
        result.warnings.extend(ui_result.warnings);

        // Validate data_dir
        if let Err(e) = validation::validate_or_create_directory(&self.data_dir, "data_dir") {
            result.add_error(
                "data_dir",
                &e.to_string(),
                Some("Ensure the path is writable"),
            );
        }

        // Update overall validity
        result.is_valid = result.errors.is_empty();
        result
    }

    fn validate_and_fix(&mut self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate and fix sub-configurations
        let window_result = self.window.validate_and_fix();
        result.errors.extend(window_result.errors);
        result.warnings.extend(window_result.warnings);

        let app_result = self.app.validate_and_fix();
        result.errors.extend(app_result.errors);
        result.warnings.extend(app_result.warnings);

        let ui_result = self.ui.validate_and_fix();
        result.errors.extend(ui_result.errors);
        result.warnings.extend(ui_result.warnings);

        // Update overall validity
        result.is_valid = result.errors.is_empty();
        result
    }
}

use crate::models::ThemeConfig;
