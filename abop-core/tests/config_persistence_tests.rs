// Tests for config file persistence and recovery.

use abop_core::config::{Config, WindowConfig};
use abop_core::models::ui::ThemeConfig;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_save_and_load_config() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let config = Config {
        window: WindowConfig {
            min_width: 1024,
            min_height: 768,
        },
        theme: ThemeConfig::Dark,
        data_dir: PathBuf::from("/tmp/abop_data"),
    };

    // Save config to file using TOML format (as the actual Config uses)
    let config_str = toml::to_string_pretty(&config).unwrap();
    fs::write(&config_path, config_str).unwrap();

    // Load config from file
    let loaded: Config = toml::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();

    assert_eq!(loaded.window.min_width, config.window.min_width);
    assert_eq!(loaded.window.min_height, config.window.min_height);
    assert_eq!(loaded.theme, config.theme);
    assert_eq!(loaded.data_dir, config.data_dir);
}

#[test]
fn test_load_default_when_config_missing() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // Do not create the file
    let result = fs::read_to_string(&config_path);
    assert!(result.is_err());

    // Should fallback to default in real app logic
    let default = Config::default();
    assert_eq!(default.window.min_width, 800);
    assert_eq!(default.window.min_height, 600);
    assert_eq!(default.theme, ThemeConfig::System);
}

#[test]
fn test_load_corrupted_config_file() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    fs::write(&config_path, b"not valid toml").unwrap();
    let result = toml::from_str::<Config>(&fs::read_to_string(&config_path).unwrap());
    assert!(result.is_err());
}

#[test]
fn test_config_default_values() {
    let config = Config::default();

    // Verify default window configuration
    assert_eq!(config.window.min_width, 800);
    assert_eq!(config.window.min_height, 600);

    // Verify default theme
    assert_eq!(config.theme, ThemeConfig::System);

    // Verify data directory is set
    assert!(!config.data_dir.as_os_str().is_empty());
}

#[test]
fn test_different_theme_configurations() {
    let themes = [ThemeConfig::System, ThemeConfig::Light, ThemeConfig::Dark];

    for theme in &themes {
        let config = Config {
            window: WindowConfig::default(),
            theme: theme.clone(),
            data_dir: PathBuf::from("/tmp/test"),
        };

        // Serialize and deserialize to test all theme variants
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.theme, theme.clone());
        assert_eq!(deserialized.theme.display_name(), theme.display_name());
    }
}
