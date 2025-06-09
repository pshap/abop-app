//! UI and window configuration settings
//!
//! This module contains configuration for user interface elements,
//! window behavior, and display preferences.

use crate::config::validation::{ConfigValidation, ValidationResult, validate_range};
use serde::{Deserialize, Serialize};

// Default value functions for serde
fn default_initial_width() -> u32 { 1200 }
fn default_initial_height() -> u32 { 800 }
fn default_remember_position() -> bool { true }
fn default_remember_size() -> bool { true }
fn default_show_decorations() -> bool { true }
fn default_resizable() -> bool { true }
fn default_opacity() -> f32 { 1.0 }

fn default_scale_factor() -> f32 { 1.0 }
fn default_animation_speed() -> f32 { 1.0 }
fn default_show_tooltips() -> bool { true }
fn default_tooltip_delay() -> u32 { 500 }
fn default_use_native_dialogs() -> bool { true }
fn default_confirm_destructive_actions() -> bool { true }
fn default_items_per_page() -> usize { 50 }
fn default_show_progress_bars() -> bool { true }

/// Window appearance and behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Minimum window width in pixels
    pub min_width: u32,
    /// Minimum window height in pixels
    pub min_height: u32,
    /// Initial window width in pixels (0 for system default)
    #[serde(default = "default_initial_width")]
    pub initial_width: u32,
    /// Initial window height in pixels (0 for system default)
    #[serde(default = "default_initial_height")]
    pub initial_height: u32,
    /// Whether to remember window position between sessions
    #[serde(default = "default_remember_position")]
    pub remember_position: bool,
    /// Whether to remember window size between sessions
    #[serde(default = "default_remember_size")]
    pub remember_size: bool,
    /// Whether to start maximized
    #[serde(default)]
    pub start_maximized: bool,
    /// Whether to show window decorations
    #[serde(default = "default_show_decorations")]
    pub show_decorations: bool,
    /// Whether to allow window resizing
    #[serde(default = "default_resizable")]
    pub resizable: bool,
    /// Window transparency (0.0 = transparent, 1.0 = opaque)
    #[serde(default = "default_opacity")]
    pub opacity: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            min_width: 800,
            min_height: 600,
            initial_width: 1200,
            initial_height: 800,
            remember_position: true,
            remember_size: true,
            start_maximized: false,
            show_decorations: true,
            resizable: true,
            opacity: 1.0,
        }
    }
}

/// UI behavior and appearance preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// UI scale factor (1.0 = normal, 2.0 = double size)
    #[serde(default = "default_scale_factor")]
    pub scale_factor: f32,
    /// Animation speed multiplier (1.0 = normal, 0.0 = no animations)
    #[serde(default = "default_animation_speed")]
    pub animation_speed: f32,
    /// Whether to show tooltips
    #[serde(default = "default_show_tooltips")]
    pub show_tooltips: bool,
    /// Tooltip delay in milliseconds
    #[serde(default = "default_tooltip_delay")]
    pub tooltip_delay: u32,
    /// Whether to use native file dialogs
    #[serde(default = "default_use_native_dialogs")]
    pub use_native_dialogs: bool,
    /// Whether to confirm destructive actions
    #[serde(default = "default_confirm_destructive_actions")]
    pub confirm_destructive_actions: bool,
    /// Number of items to show per page in lists
    #[serde(default = "default_items_per_page")]
    pub items_per_page: usize,
    /// Whether to show progress bars for long operations
    #[serde(default = "default_show_progress_bars")]
    pub show_progress_bars: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            animation_speed: 1.0,
            show_tooltips: true,
            tooltip_delay: 500,
            use_native_dialogs: true,
            confirm_destructive_actions: true,
            items_per_page: 50,
            show_progress_bars: true,
        }
    }
}

impl ConfigValidation for WindowConfig {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate minimum dimensions
        if let Err(e) = validate_range(self.min_width, 400, 4000, "min_width") {
            result.add_error("min_width", &e.to_string(), Some("Use a value between 400 and 4000"));
        }

        if let Err(e) = validate_range(self.min_height, 300, 3000, "min_height") {
            result.add_error("min_height", &e.to_string(), Some("Use a value between 300 and 3000"));
        }

        // Validate initial dimensions
        if self.initial_width > 0 && self.initial_width < self.min_width {
            result.add_error(
                "initial_width",
                "Initial width cannot be smaller than minimum width",
                Some("Set initial_width to 0 for system default or >= min_width"),
            );
        }

        if self.initial_height > 0 && self.initial_height < self.min_height {
            result.add_error(
                "initial_height",
                "Initial height cannot be smaller than minimum height",
                Some("Set initial_height to 0 for system default or >= min_height"),
            );
        }

        // Validate opacity
        if let Err(e) = validate_range(self.opacity, 0.1, 1.0, "opacity") {
            result.add_error("opacity", &e.to_string(), Some("Use a value between 0.1 and 1.0"));
        }

        result
    }

    fn validate_and_fix(&mut self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Auto-fix minimum dimensions
        if self.min_width < 400 {
            self.min_width = 400;
            result.add_warning("min_width", "Automatically increased min_width to 400", None);
        }
        if self.min_height < 300 {
            self.min_height = 300;
            result.add_warning("min_height", "Automatically increased min_height to 300", None);
        }

        // Auto-fix initial dimensions
        if self.initial_width > 0 && self.initial_width < self.min_width {
            self.initial_width = self.min_width;
            result.add_warning(
                "initial_width",
                "Automatically adjusted initial_width to match min_width",
                None,
            );
        }

        if self.initial_height > 0 && self.initial_height < self.min_height {
            self.initial_height = self.min_height;
            result.add_warning(
                "initial_height",
                "Automatically adjusted initial_height to match min_height",
                None,
            );
        }

        // Auto-fix opacity
        if self.opacity < 0.1 {
            self.opacity = 0.1;
            result.add_warning("opacity", "Automatically increased opacity to 0.1", None);
        } else if self.opacity > 1.0 {
            self.opacity = 1.0;
            result.add_warning("opacity", "Automatically decreased opacity to 1.0", None);
        }

        result
    }
}

impl ConfigValidation for UiConfig {
    fn validate(&self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Validate scale factor
        if let Err(e) = validate_range(self.scale_factor, 0.5, 4.0, "scale_factor") {
            result.add_error("scale_factor", &e.to_string(), Some("Use a value between 0.5 and 4.0"));
        }

        // Validate animation speed
        if let Err(e) = validate_range(self.animation_speed, 0.0, 3.0, "animation_speed") {
            result.add_error("animation_speed", &e.to_string(), Some("Use a value between 0.0 and 3.0"));
        }

        // Validate tooltip delay
        if self.tooltip_delay > 5000 {
            result.add_warning(
                "tooltip_delay",
                "Very long tooltip delay may impact user experience",
                Some("Consider using a value under 2000ms"),
            );
        }

        // Validate items per page
        if self.items_per_page > 500 {
            result.add_warning(
                "items_per_page",
                "Very large page size may impact performance",
                Some("Consider using a value under 200"),
            );
        }

        result
    }

    fn validate_and_fix(&mut self) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Auto-fix scale factor
        if self.scale_factor < 0.5 {
            self.scale_factor = 0.5;
            result.add_warning("scale_factor", "Automatically increased scale_factor to 0.5", None);
        } else if self.scale_factor > 4.0 {
            self.scale_factor = 4.0;
            result.add_warning("scale_factor", "Automatically decreased scale_factor to 4.0", None);
        }

        // Auto-fix animation speed
        if self.animation_speed < 0.0 {
            self.animation_speed = 0.0;
            result.add_warning("animation_speed", "Automatically increased animation_speed to 0.0", None);
        } else if self.animation_speed > 3.0 {
            self.animation_speed = 3.0;
            result.add_warning("animation_speed", "Automatically decreased animation_speed to 3.0", None);
        }

        // Auto-fix tooltip delay (if extremely high)
        if self.tooltip_delay > 10000 {
            self.tooltip_delay = 2000;
            result.add_warning("tooltip_delay", "Automatically reduced tooltip_delay to 2000ms", None);
        }

        // Auto-fix items per page (if extremely high)
        if self.items_per_page > 1000 {
            self.items_per_page = 100;
            result.add_warning("items_per_page", "Automatically reduced items_per_page to 100", None);
        } else if self.items_per_page == 0 {
            self.items_per_page = 50;
            result.add_warning("items_per_page", "Automatically set items_per_page to 50", None);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.min_width, 800);
        assert_eq!(config.min_height, 600);
        assert!(config.resizable);
        assert_eq!(config.opacity, 1.0);
    }

    #[test]
    fn test_window_config_validation() {
        let mut config = WindowConfig::default();
        config.min_width = 200; // Too small
        config.opacity = 2.0; // Invalid

        let result = config.validate();
        assert!(result.has_errors());
        assert_eq!(result.errors.len(), 2);
    }

    #[test]
    fn test_ui_config_default() {
        let config = UiConfig::default();
        assert_eq!(config.scale_factor, 1.0);
        assert_eq!(config.animation_speed, 1.0);
        assert!(config.show_tooltips);
    }

    #[test]
    fn test_ui_config_validation() {
        let mut config = UiConfig::default();
        config.scale_factor = 5.0; // Too large
        config.tooltip_delay = 10000; // Very long

        let result = config.validate();
        assert!(result.has_errors() || result.has_warnings());
    }

    #[test]
    fn test_window_config_validate_and_fix() {
        let mut config = WindowConfig::default();
        config.min_width = 200; // Too small
        config.opacity = 2.0; // Too high

        let result = config.validate_and_fix();
        assert!(result.has_warnings());
        assert_eq!(config.min_width, 400); // Should be auto-fixed
        assert_eq!(config.opacity, 1.0); // Should be auto-fixed
    }

    #[test]
    fn test_ui_config_validate_and_fix() {
        let mut config = UiConfig::default();
        config.scale_factor = 5.0; // Too large
        config.animation_speed = -1.0; // Too small
        config.items_per_page = 0; // Invalid

        let result = config.validate_and_fix();
        assert!(result.has_warnings());
        assert_eq!(config.scale_factor, 4.0); // Should be auto-fixed
        assert_eq!(config.animation_speed, 0.0); // Should be auto-fixed
        assert_eq!(config.items_per_page, 50); // Should be auto-fixed
    }
}
