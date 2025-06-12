// Tests for config file persistence and recovery.

use abop_core::config::{AppConfig, Config, UiConfig, WindowConfig};
use abop_core::models::ui::ThemeConfig;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_save_and_load_config() {
    let dir = tempdir().expect("Should create temporary directory");
    let config_path = dir.path().join("config.toml");

    let config = Config {
        window: WindowConfig {
            min_width: 1024,
            min_height: 768,
            initial_width: 1024,
            initial_height: 768,
            remember_position: true,
            remember_size: true,
            start_maximized: false,
            show_decorations: true,
            resizable: true,
            opacity: 1.0,
        },
        theme: ThemeConfig::Dark,
        data_dir: PathBuf::from("/tmp/abop_data"),
        app: AppConfig::default(),
        ui: UiConfig::default(),
    };

    // Save config to file using TOML format (as the actual Config uses)
    let config_str = toml::to_string_pretty(&config).expect("Should serialize config to TOML");
    fs::write(&config_path, config_str).expect("Should write config file");

    // Load config from file
    let loaded: Config =
        toml::from_str(&fs::read_to_string(&config_path).expect("Should read config file"))
            .expect("Should deserialize config from TOML");

    assert_eq!(loaded.window.min_width, config.window.min_width);
    assert_eq!(loaded.window.min_height, config.window.min_height);
    assert_eq!(loaded.theme, config.theme);
    assert_eq!(loaded.data_dir, config.data_dir);
}

#[test]
fn test_load_default_when_config_missing() {
    let dir = tempdir().expect("Should create temporary directory");
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
    let dir = tempdir().expect("Should create temporary directory");
    let config_path = dir.path().join("config.toml");

    fs::write(&config_path, b"not valid toml").expect("Should write corrupted file");
    let result = toml::from_str::<Config>(
        &fs::read_to_string(&config_path).expect("Should read corrupted file"),
    );
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
            app: AppConfig::default(),
            ui: UiConfig::default(),
        };

        // Serialize and deserialize to test all theme variants
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.theme, theme.clone());
        assert_eq!(deserialized.theme.display_name(), theme.display_name());
    }
}
