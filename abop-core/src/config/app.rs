//! Application-level configuration settings
//!
//! This module contains general application settings that don't fit into
//! more specific categories like audio, database, or UI configuration.

use crate::config::validation::{ConfigValidation, ValidationResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Default value functions for serde
fn default_app_name() -> String {
    "ABOP Iced".to_string()
}
fn default_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
fn default_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("./data"))
        .join("abop-iced")
}
fn default_debug_mode() -> bool {
    cfg!(debug_assertions)
}
fn default_max_recent_files() -> usize {
    10
}
fn default_auto_save_interval() -> u64 {
    300
}
fn default_crash_reporting() -> bool {
    true
}

/// Application-level configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application name for display purposes
    #[serde(default = "default_app_name")]
    pub app_name: String,
    /// Application version
    #[serde(default = "default_version")]
    pub version: String,
    /// Directory for application data storage
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    /// Enable debug logging
    #[serde(default = "default_debug_mode")]
    pub debug_mode: bool,
    /// Maximum number of recent files to remember
    #[serde(default = "default_max_recent_files")]
    pub max_recent_files: usize,
    /// Automatic save interval in seconds (0 to disable)
    #[serde(default = "default_auto_save_interval")]
    pub auto_save_interval: u64,
    /// Enable crash reporting
    #[serde(default = "default_crash_reporting")]
    pub crash_reporting: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "ABOP Iced".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            data_dir: dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("./data"))
                .join("abop-iced"),
            debug_mode: cfg!(debug_assertions),
            max_recent_files: 10,
            auto_save_interval: 300, // 5 minutes
            crash_reporting: true,
        }
    }
}

impl ConfigValidation for AppConfig {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate data directory
        if let Err(e) =
            crate::config::validation::validate_or_create_directory(&self.data_dir, "data_dir")
        {
            result.add_error(
                "data_dir",
                &e.to_string(),
                Some("Ensure the path is writable"),
            );
        }

        // Validate max_recent_files range
        if self.max_recent_files > 100 {
            result.add_warning(
                "max_recent_files",
                "Very large number of recent files may impact performance",
                Some("Consider reducing to 50 or fewer"),
            );
        }

        // Validate auto_save_interval
        if self.auto_save_interval > 0 && self.auto_save_interval < 60 {
            result.add_warning(
                "auto_save_interval",
                "Very frequent auto-save may impact performance",
                Some("Consider setting to at least 60 seconds"),
            );
        }

        result
    }

    fn validate_and_fix(&mut self) -> ValidationResult {
        let mut result = self.validate();

        // Auto-fix common issues
        if self.max_recent_files > 100 {
            self.max_recent_files = 50;
            result.add_warning(
                "max_recent_files",
                "Automatically reduced max_recent_files to 50",
                None,
            );
        }

        if self.auto_save_interval > 0 && self.auto_save_interval < 60 {
            self.auto_save_interval = 60;
            result.add_warning(
                "auto_save_interval",
                "Automatically increased auto_save_interval to 60 seconds",
                None,
            );
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.app_name, "ABOP Iced");
        assert!(config.max_recent_files <= 100);
        assert!(config.auto_save_interval >= 60 || config.auto_save_interval == 0);
    }

    #[test]
    fn test_app_config_validation() {
        let config = AppConfig {
            max_recent_files: 150,
            auto_save_interval: 30,
            ..Default::default()
        };

        let result = config.validate();
        assert!(result.has_warnings());
        assert_eq!(result.warnings.len(), 2);
    }

    #[test]
    fn test_app_config_validate_and_fix() {
        let mut config = AppConfig {
            max_recent_files: 150,
            auto_save_interval: 30,
            ..Default::default()
        };

        let result = config.validate_and_fix();
        assert_eq!(config.max_recent_files, 50);
        assert_eq!(config.auto_save_interval, 60);
        assert!(result.has_warnings());
    }
}
