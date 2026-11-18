//! Persisted GUI preferences
//!
//! Stored as TOML in the platform config directory (e.g.
//! `~/.config/abop-iced/gui.toml`). Saving is driven by [`crate::app::App`]:
//! after every update it compares the current preferences with the last saved
//! copy and writes only when something changed.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::theme::ThemeMode;

/// How many recent library folders to remember
const MAX_RECENT_DIRECTORIES: usize = 10;

/// User preferences that survive restarts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct GuiPreferences {
    /// Library folder that was open when the app last ran
    pub last_library: Option<PathBuf>,
    /// Recently opened library folders, most recent first
    pub recent_directories: Vec<PathBuf>,
    /// Use the dark theme
    pub dark_theme: bool,
    /// Include subfolders when scanning
    pub scan_subdirectories: bool,
    /// Save library changes automatically
    pub auto_save_library: bool,
}

impl Default for GuiPreferences {
    fn default() -> Self {
        Self {
            last_library: None,
            recent_directories: Vec::new(),
            dark_theme: true,
            scan_subdirectories: true,
            auto_save_library: true,
        }
    }
}

impl GuiPreferences {
    /// Default location of the preferences file
    #[must_use]
    pub fn default_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| {
            dir.join(abop_core::models::ui::APP_DATA_DIR)
                .join("gui.toml")
        })
    }

    /// Load preferences, falling back to defaults if the file is missing or invalid
    #[must_use]
    pub fn load_from(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_else(|error| {
                log::warn!(
                    "Ignoring invalid preferences file {}: {error}",
                    path.display()
                );
                Self::default()
            }),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Self::default(),
            Err(error) => {
                log::warn!("Could not read preferences {}: {error}", path.display());
                Self::default()
            }
        }
    }

    /// Write preferences atomically (temporary file, then rename)
    ///
    /// # Errors
    ///
    /// Returns an error if the directory or file cannot be written.
    pub fn save_to(&self, path: &Path) -> std::io::Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let contents = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
        let temp = path.with_extension("toml.tmp");
        std::fs::write(&temp, contents)?;
        std::fs::rename(&temp, path)
    }

    /// Capture the preferences from the current application state
    #[must_use]
    pub fn from_state(state: &AppState) -> Self {
        let library = &state.library;
        let last_library =
            (!library.library_path.as_os_str().is_empty()).then(|| library.library_path.clone());

        // Most recent first, without duplicates
        let mut recent_directories: Vec<PathBuf> = Vec::new();
        for path in library.recent_directories.iter().rev().map(|dir| &dir.path) {
            if !recent_directories.contains(path) {
                recent_directories.push(path.clone());
            }
        }
        recent_directories.truncate(MAX_RECENT_DIRECTORIES);

        Self {
            last_library,
            recent_directories,
            dark_theme: !matches!(
                state.ui.theme_mode,
                ThemeMode::Light | ThemeMode::MaterialLight | ThemeMode::Slate
            ),
            scan_subdirectories: library.scan_subdirectories,
            auto_save_library: library.auto_save_library,
        }
    }

    /// Apply the preferences to a freshly created application state
    pub fn apply_to(&self, state: &mut AppState) {
        state.ui.set_theme_mode(if self.dark_theme {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        });
        state.library.scan_subdirectories = self.scan_subdirectories;
        state.library.auto_save_library = self.auto_save_library;
        // Stored most recent first; the state keeps the most recent last
        state.library.recent_directories = self
            .recent_directories
            .iter()
            .rev()
            .map(|path| crate::state::DirectoryInfo {
                path: path.clone(),
                last_scan: std::time::SystemTime::UNIX_EPOCH,
                book_count: 0,
                scan_duration: std::time::Duration::ZERO,
            })
            .collect();
        if let Some(library) = &self.last_library {
            state.library.library_path = library.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/gui.toml");
        let prefs = GuiPreferences {
            last_library: Some("/books".into()),
            recent_directories: vec!["/books".into(), "/old".into()],
            dark_theme: false,
            scan_subdirectories: false,
            auto_save_library: true,
        };
        prefs.save_to(&path).unwrap();
        assert_eq!(GuiPreferences::load_from(&path), prefs);
        assert!(!path.with_extension("toml.tmp").exists());
    }

    #[test]
    fn missing_or_invalid_files_give_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.toml");
        assert_eq!(
            GuiPreferences::load_from(&missing),
            GuiPreferences::default()
        );

        let invalid = dir.path().join("invalid.toml");
        std::fs::write(&invalid, "dark_theme = \"not a bool\"").unwrap();
        assert_eq!(
            GuiPreferences::load_from(&invalid),
            GuiPreferences::default()
        );

        // Unknown and missing fields are tolerated
        let partial = dir.path().join("partial.toml");
        std::fs::write(&partial, "dark_theme = false\nfuture_option = 1\n").unwrap();
        let loaded = GuiPreferences::load_from(&partial);
        assert!(!loaded.dark_theme);
        assert!(loaded.scan_subdirectories);
    }

    #[test]
    fn state_round_trip_preserves_settings_and_recent_order() {
        let prefs = GuiPreferences {
            last_library: Some("/books".into()),
            recent_directories: vec!["/books".into(), "/older".into()],
            dark_theme: false,
            scan_subdirectories: false,
            auto_save_library: false,
        };
        let mut state = AppState::default();
        prefs.apply_to(&mut state);

        assert_eq!(state.ui.theme_mode, ThemeMode::Light);
        assert_eq!(state.library.library_path, PathBuf::from("/books"));
        assert_eq!(GuiPreferences::from_state(&state), prefs);
    }
}
