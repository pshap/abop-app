//! Integration tests for the modular configuration system
//!
//! These tests ensure that the new modular configuration system maintains
//! backward compatibility while providing enhanced validation and functionality.

use abop_core::config::{AppConfig, Config, ConfigValidation, UiConfig, WindowConfig};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_config_backward_compatibility() {
    // Test that old TOML format still works
    let old_config_toml = r#"theme = "Light"
data_dir = "/tmp/test_data"

[window]
min_width = 1024
min_height = 768
"#;

    let config: Config = toml::from_str(old_config_toml).unwrap();
    assert_eq!(config.window.min_width, 1024);
    assert_eq!(config.window.min_height, 768);
    assert_eq!(config.data_dir.to_string_lossy(), "/tmp/test_data");

    // New fields should have defaults
    assert_eq!(config.app.app_name, "ABOP Iced");
    assert_eq!(config.ui.scale_factor, 1.0);
}

#[test]
fn test_config_new_format() {
    // Test the new modular format
    let new_config_toml = r#"theme = "Dark"
data_dir = "/tmp/test_data"

[window]
min_width = 1200
min_height = 800
initial_width = 1400
initial_height = 900

[app]
app_name = "Custom ABOP"
version = "0.1.0"
data_dir = "/tmp/app_data"
debug_mode = true
max_recent_files = 20

[ui]
scale_factor = 1.2
animation_speed = 0.8
show_tooltips = false
"#;

    let config: Config = toml::from_str(new_config_toml).unwrap();
    assert_eq!(config.window.min_width, 1200);
    assert_eq!(config.app.app_name, "Custom ABOP");
    assert_eq!(config.app.max_recent_files, 20);
    assert_eq!(config.ui.scale_factor, 1.2);
    assert_eq!(config.ui.animation_speed, 0.8);
    assert!(!config.ui.show_tooltips);
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();

    // Set some invalid values
    config.window.min_width = 200; // Too small
    config.app.max_recent_files = 150; // Too many
    config.ui.scale_factor = 5.0; // Too large

    let result = config.validate();
    assert!(result.has_errors());
    assert!(result.has_warnings());
    assert!(!result.is_valid);
}

#[test]
fn test_config_validate_and_fix() {
    let mut config = Config::default();

    // Set some fixable invalid values
    config.app.max_recent_files = 150; // Will be fixed to 50
    config.app.auto_save_interval = 30; // Will be fixed to 60

    let result = config.validate_and_fix();
    assert_eq!(config.app.max_recent_files, 50);
    assert_eq!(config.app.auto_save_interval, 60);
    assert!(result.has_warnings()); // Should have warnings about auto-fixes
}

#[test]
fn test_config_serialization_roundtrip() {
    let original = Config::default();

    // Serialize to TOML
    let toml_string = toml::to_string_pretty(&original).unwrap();

    // Deserialize back
    let deserialized: Config = toml::from_str(&toml_string).unwrap();

    // Should be equal
    assert_eq!(original.window.min_width, deserialized.window.min_width);
    assert_eq!(original.app.app_name, deserialized.app.app_name);
    assert_eq!(original.ui.scale_factor, deserialized.ui.scale_factor);
}

#[test]
fn test_config_load_save_integration() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create a config with custom values
    let mut original = Config::default();
    original.app.app_name = "Test App".to_string();
    original.ui.scale_factor = 1.5;

    // Save to file
    let contents = toml::to_string_pretty(&original).unwrap();
    fs::write(&config_path, contents).unwrap();

    // Load from file
    let loaded_contents = fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&loaded_contents).unwrap();

    assert_eq!(loaded.app.app_name, "Test App");
    assert_eq!(loaded.ui.scale_factor, 1.5);
}

#[test]
fn test_window_config_validation() {
    let window_config = WindowConfig {
        min_width: 200, // Too small
        min_height: 100, // Too small
        ..Default::default()
    };

    let result = window_config.validate();
    assert!(result.has_errors());
    assert_eq!(result.errors.len(), 2);
}

#[test]
fn test_app_config_validation() {
    let app_config = AppConfig {
        max_recent_files: 200, // Too many
        auto_save_interval: 10, // Too frequent
        ..Default::default()
    };

    let result = app_config.validate();
    assert!(result.has_warnings());
    assert_eq!(result.warnings.len(), 2);
}

#[test]
fn test_ui_config_validation() {
    let ui_config = UiConfig {
        scale_factor: 10.0, // Too large
        animation_speed: -1.0, // Invalid
        ..Default::default()
    };

    let result = ui_config.validate();
    assert!(result.has_errors());
    assert_eq!(result.errors.len(), 2);
}

#[test]
fn test_migration_from_old_config() {
    // Simulate an old config file that only has basic fields
    let old_config_toml = r#"theme = "System"
data_dir = "/old/data/path"

[window]
min_width = 800
min_height = 600
"#;

    let mut config: Config = toml::from_str(old_config_toml).unwrap();

    // Validate and fix should not break anything
    let result = config.validate_and_fix();
    assert!(result.is_valid || result.has_warnings()); // Should be valid or have only warnings

    // New fields should have sensible defaults
    assert!(!config.app.app_name.is_empty());
    assert!(config.ui.scale_factor > 0.0);
    assert!(config.app.max_recent_files > 0);
}

#[test]
fn test_config_error_handling() {
    // Test invalid TOML
    let invalid_toml = r#"
[window
min_width = "not a number"
"#;

    let result = toml::from_str::<Config>(invalid_toml);
    assert!(result.is_err());
}

#[test]
fn test_validation_error_details() {
    let mut config = Config::default();
    config.window.min_width = 100;
    config.window.min_height = 50;

    let result = config.validate();
    assert!(result.has_errors());

    // Check that error messages are helpful
    let width_error = result.errors.iter().find(|e| e.field == "min_width");
    assert!(width_error.is_some());
    assert!(width_error.unwrap().suggestion.is_some());
}

#[test]
fn test_config_default_values() {
    let config = Config::default();

    // Test that all defaults are reasonable
    assert!(config.window.min_width >= 400);
    assert!(config.window.min_height >= 300);
    assert!(!config.app.app_name.is_empty());
    assert!(config.ui.scale_factor > 0.0 && config.ui.scale_factor <= 4.0);
    assert!(config.app.max_recent_files > 0 && config.app.max_recent_files <= 100);
}
