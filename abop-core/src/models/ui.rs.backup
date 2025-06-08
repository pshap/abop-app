//! UI state and configuration models

use crate::error::{AppError, Result};
use crate::models::{Audiobook, Library, Progress};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::mpsc;

/// Centralized application state for ABOP
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    /// Currently active application view
    pub current_view: ViewType,
    /// User's saved preferences and settings
    pub user_preferences: UserPreferences,
    /// Application runtime data and state
    pub data: AppData,
}

impl AppState {
    /// Creates a new default application state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Load application state from file system with validation and repair
    ///
    /// This method loads the state, validates it for integrity issues,
    /// and automatically repairs any problems found. If critical issues
    /// are detected that cannot be repaired, a default state is used.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to determine the state file path
    /// - Failed to save the default state (if file doesn't exist)
    /// - Failed to save the repaired state (if repairs were made)
    pub fn load() -> Result<Self> {
        let state_path = Self::state_path()?;
        if state_path.exists() {
            match Self::load_and_validate() {
                Ok(state) => Ok(state),
                Err(e) => {
                    log::warn!("Failed to load or validate state: {e}. Using default state.");
                    let default = Self::default();
                    default.save()?;
                    Ok(default)
                }
            }
        } else {
            let default = Self::default();
            default.save()?;
            Ok(default)
        }
    }

    /// Load and validate state from file system
    ///
    /// This is a separate method for better error handling and testing.
    fn load_and_validate() -> Result<Self> {
        use crate::validation::validate_and_repair_app_state;

        let state_path = Self::state_path()?;
        let contents = std::fs::read_to_string(&state_path)?;
        let mut state: Self = toml::from_str(&contents)?;

        // Validate and repair the loaded state
        let (validation_result, repair_actions) = validate_and_repair_app_state(&mut state);

        if !repair_actions.is_empty() {
            log::info!(
                "Applied {} repair actions to loaded state:",
                repair_actions.len()
            );
            for action in &repair_actions {
                if action.success {
                    log::info!("  ✓ {}: {}", action.action_type, action.description);
                    if let Some(details) = &action.details {
                        log::info!("    {details}");
                    }
                } else {
                    log::warn!("  ✗ {}: {}", action.action_type, action.description);
                }
            }

            // Save the repaired state back to disk
            state.save()?;
            log::info!("Saved repaired state to disk");
        }

        if validation_result.has_critical_issues() {
            return Err(AppError::Config(format!(
                "Critical validation issues found: {}",
                validation_result.critical_issues().len()
            )));
        }

        if !validation_result.is_valid() {
            log::warn!(
                "State validation found {} issues:",
                validation_result.issues.len()
            );
            for issue in &validation_result.issues {
                log::warn!(
                    "  {} [{}]: {}",
                    issue.severity,
                    issue.category,
                    issue.message
                );
            }
        } else {
            log::info!("State validation passed successfully");
        }

        Ok(state)
    }

    /// Save current application state to file system
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to determine the state file path
    /// - Failed to create the parent directory
    /// - Failed to serialize the state to TOML
    /// - Failed to write the state file
    pub fn save(&self) -> Result<()> {
        let state_path = Self::state_path()?;
        if let Some(parent) = state_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(&state_path, contents)?;
        Ok(())
    }

    /// Save current application state to file system asynchronously for Iced
    ///
    /// This is a blocking operation that should be used with `Task::perform`
    /// to run in the background without blocking the UI.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to determine the state file path
    /// - Failed to create the parent directory
    /// - Failed to serialize the state to TOML
    /// - Failed to write the state file
    pub fn save_blocking(self) -> Result<String> {
        let state_path = Self::state_path()?;
        log::info!("save_blocking: Saving to path: {}", state_path.display());

        if let Some(parent) = state_path.parent() {
            log::info!(
                "save_blocking: Creating parent directory: {}",
                parent.display()
            );
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!(
                    "save_blocking: Failed to create parent directory {}: {}",
                    parent.display(),
                    e
                );
                return Err(AppError::Io(e.to_string()));
            }
        }

        log::info!(
            "save_blocking: About to serialize AppState ({} audiobooks) to TOML.",
            self.data.audiobooks.len()
        );

        let contents = match toml::to_string_pretty(&self) {
            Ok(c) => {
                log::info!(
                    "save_blocking: TOML serialization successful. Content length: {}",
                    c.len()
                );
                c
            }
            Err(e) => {
                log::error!("save_blocking: TOML serialization FAILED: {e}");
                return Err(AppError::TomlSer(e.to_string()));
            }
        };

        log::info!(
            "save_blocking: About to write file at {}",
            state_path.display()
        );
        if let Err(e) = std::fs::write(&state_path, contents) {
            log::error!(
                "save_blocking: File write FAILED for {}: {}",
                state_path.display(),
                e
            );
            return Err(AppError::Io(e.to_string()));
        }

        log::info!(
            "save_blocking: File write successful to {}",
            state_path.display()
        );
        Ok(format!(
            "State saved successfully with {} audiobooks",
            self.data.audiobooks.len()
        ))
    }

    /// Save current application state with progress reporting
    ///
    /// This version sends progress updates through a channel and includes delays
    /// to make progress visible for large datasets.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Failed to determine the state file path
    /// - Failed to create the parent directory
    /// - Failed to serialize the state to TOML
    /// - Failed to write the state file
    pub fn save_blocking_with_progress(
        self,
        progress_sender: Option<mpsc::Sender<f32>>,
    ) -> Result<String> {
        let send_progress = |progress: f32| {
            if let Some(ref sender) = progress_sender {
                let _ = sender.send(progress);
                // Small delay to make progress visible
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        };

        let state_path = Self::state_path()?;
        log::info!(
            "save_blocking_with_progress: Saving to path: {}",
            state_path.display()
        );
        send_progress(0.1); // 10% - Started

        if let Some(parent) = state_path.parent() {
            log::info!(
                "save_blocking_with_progress: Creating parent directory: {}",
                parent.display()
            );
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!(
                    "save_blocking_with_progress: Failed to create parent directory {}: {}",
                    parent.display(),
                    e
                );
                return Err(AppError::Io(e.to_string()));
            }
        }
        send_progress(0.2); // 20% - Directory created

        log::info!(
            "save_blocking_with_progress: About to serialize AppState ({} audiobooks) to TOML.",
            self.data.audiobooks.len()
        );
        send_progress(0.3); // 30% - Starting serialization

        // For large datasets, serialization can take time, so we'll simulate progress
        let audiobook_count = self.data.audiobooks.len();
        let is_large_dataset = audiobook_count > 50;

        if is_large_dataset {
            log::info!(
                "save_blocking_with_progress: Large dataset detected ({audiobook_count} audiobooks), will show serialization progress"
            );
            send_progress(0.4); // 40%
            std::thread::sleep(std::time::Duration::from_millis(100));
            send_progress(0.5); // 50%
            std::thread::sleep(std::time::Duration::from_millis(100));
            send_progress(0.6); // 60%
        }

        let contents = match toml::to_string_pretty(&self) {
            Ok(c) => {
                log::info!(
                    "save_blocking_with_progress: TOML serialization successful. Content length: {}",
                    c.len()
                );
                c
            }
            Err(e) => {
                log::error!("save_blocking_with_progress: TOML serialization FAILED: {e}");
                return Err(AppError::TomlSer(e.to_string()));
            }
        };

        send_progress(0.8); // 80% - Serialization complete

        log::info!(
            "save_blocking_with_progress: About to write file at {}",
            state_path.display()
        );
        if let Err(e) = std::fs::write(&state_path, &contents) {
            log::error!(
                "save_blocking_with_progress: File write FAILED for {}: {}",
                state_path.display(),
                e
            );
            return Err(AppError::Io(e.to_string()));
        }

        send_progress(1.0); // 100% - Complete
        log::info!(
            "save_blocking_with_progress: File write successful to {}",
            state_path.display()
        );
        Ok(format!(
            "State saved successfully with {} audiobooks",
            self.data.audiobooks.len()
        ))
    }

    fn state_path() -> Result<PathBuf> {
        let mut path = dirs::data_dir()
            .ok_or_else(|| AppError::Config("Could not find data directory".to_string()))?;
        path.push("abop-iced");
        path.push("app_state.toml");
        Ok(path)
    }

    /// Switches to a different view
    pub const fn switch_view(&mut self, view: ViewType) {
        self.current_view = view;
    }

    /// Gets the current view type
    #[must_use]
    pub const fn current_view(&self) -> &ViewType {
        &self.current_view
    }

    /// Adds a library to the application data
    pub fn add_library(&mut self, library: Library) {
        self.data.libraries.push(library);
    }

    /// Removes a library by ID
    pub fn remove_library(&mut self, library_id: &str) -> bool {
        let initial_len = self.data.libraries.len();
        self.data.libraries.retain(|lib| lib.id != library_id);
        self.data.libraries.len() != initial_len
    }

    /// Gets a library by ID
    #[must_use]
    pub fn get_library(&self, library_id: &str) -> Option<&Library> {
        self.data.libraries.iter().find(|lib| lib.id == library_id)
    }

    /// Gets all libraries
    #[must_use]
    pub fn libraries(&self) -> &[Library] {
        &self.data.libraries
    }

    /// Adds an audiobook to the application data
    pub fn add_audiobook(&mut self, audiobook: Audiobook) {
        self.data.audiobooks.push(audiobook);
    }

    /// Gets audiobooks for a specific library
    #[must_use]
    pub fn audiobooks_for_library(&self, library_id: &str) -> Vec<&Audiobook> {
        self.data
            .audiobooks
            .iter()
            .filter(|book| book.library_id == library_id)
            .collect()
    }

    /// Gets all audiobooks
    #[must_use]
    pub fn audiobooks(&self) -> &[Audiobook] {
        &self.data.audiobooks
    }

    /// Updates or creates progress for an audiobook
    pub fn update_progress(&mut self, audiobook_id: &str, position_seconds: u64) {
        if let Some(progress) = self
            .data
            .progress
            .iter_mut()
            .find(|p| p.audiobook_id == audiobook_id)
        {
            progress.update_position(position_seconds);
        } else {
            let new_progress = Progress::new(audiobook_id, position_seconds);
            self.data.progress.push(new_progress);
        }
    }

    /// Gets progress for a specific audiobook
    #[must_use]
    pub fn get_progress(&self, audiobook_id: &str) -> Option<&Progress> {
        self.data
            .progress
            .iter()
            .find(|p| p.audiobook_id == audiobook_id)
    }

    /// Gets recently played audiobooks (based on progress)
    #[must_use]
    pub fn recently_played_audiobooks(&self) -> Vec<&Audiobook> {
        let recent_audiobook_ids: Vec<&str> = self
            .data
            .progress
            .iter()
            .filter(|p| p.is_recently_played())
            .map(|p| p.audiobook_id.as_str())
            .collect();

        self.data
            .audiobooks
            .iter()
            .filter(|book| recent_audiobook_ids.contains(&book.id.as_str()))
            .collect()
    }
}

/// User configuration preferences for the application
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPreferences {
    /// Current theme configuration
    pub theme: ThemeConfig,
    /// Recently accessed directory paths
    pub recent_directories: Vec<PathBuf>,
    /// Window size and position preferences
    pub window_config: WindowConfig,
    /// Audio playback preferences
    pub playback_config: PlaybackConfig,
}

impl UserPreferences {
    /// Adds a directory to recent directories (avoiding duplicates)
    pub fn add_recent_directory(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_directories.retain(|p| p != &path);
        // Add to front
        self.recent_directories.insert(0, path);
        // Keep only last 10
        self.recent_directories.truncate(10);
    }

    /// Sets the theme
    pub const fn set_theme(&mut self, theme: ThemeConfig) {
        self.theme = theme;
    }

    /// Gets the most recent directory, or None if empty
    #[must_use]
    pub fn most_recent_directory(&self) -> Option<&PathBuf> {
        self.recent_directories.first()
    }
}

/// Runtime application data including libraries and bookmarks
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    /// Available audiobook libraries
    pub libraries: Vec<Library>,
    /// Loaded audiobook metadata
    pub audiobooks: Vec<Audiobook>,
    /// Playback progress tracking data
    pub progress: Vec<Progress>,
}

/// Available application view modes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ViewType {
    /// Library browsing and management view
    #[default]
    Library,
    /// Application settings and configuration view
    Settings,
    /// Audio processing and conversion view
    AudioProcessing,
    /// Application information and credits view
    About,
}

impl ViewType {
    /// Gets a human-readable name for the view
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Library => "Library",
            Self::Settings => "Settings",
            Self::AudioProcessing => "Audio Processing",
            Self::About => "About",
        }
    }

    /// Gets all available view types
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Library,
            Self::Settings,
            Self::AudioProcessing,
            Self::About,
        ]
    }
}

/// Theme configuration options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ThemeConfig {
    /// Use system theme preference
    #[default]
    System,
    /// Use light theme colors
    Light,
    /// Use dark theme colors
    Dark,
}

impl ThemeConfig {
    /// Gets a human-readable name for the theme
    #[must_use]
    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::System => "System",
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    /// Gets all available theme options
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![Self::System, Self::Light, Self::Dark]
    }
}

/// Window configuration preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    /// Window width in pixels
    pub width: u32,
    /// Window height in pixels
    pub height: u32,
    /// Whether window is maximized
    pub maximized: bool,
    /// Whether to remember window position
    pub remember_position: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 1200,
            height: 800,
            maximized: false,
            remember_position: true,
        }
    }
}

/// Audio playback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackConfig {
    /// Default playback volume (0.0 to 1.0)
    pub volume: f32,
    /// Default playback speed multiplier
    pub speed: f32,
    /// Whether to resume from last position automatically
    pub auto_resume: bool,
    /// Skip forward/backward amount in seconds
    pub skip_amount: u64,
    /// Whether to automatically bookmark when stopping playback
    pub auto_bookmark: bool,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            volume: 0.8,
            speed: 1.0,
            auto_resume: true,
            skip_amount: 15,
            auto_bookmark: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_app_state_creation() {
        let state = AppState::new();
        assert_eq!(state.current_view, ViewType::Library);
        assert!(state.data.libraries.is_empty());
        assert!(state.data.audiobooks.is_empty());
    }

    #[test]
    fn test_view_switching() {
        let mut state = AppState::new();
        assert_eq!(state.current_view(), &ViewType::Library);

        state.switch_view(ViewType::Settings);
        assert_eq!(state.current_view(), &ViewType::Settings);
    }

    #[test]
    fn test_library_management() {
        let mut state = AppState::new();
        let library = Library::new("Test Library", "/test/path");
        let library_id = library.id.clone();

        state.add_library(library);
        assert_eq!(state.libraries().len(), 1);
        assert!(state.get_library(&library_id).is_some());

        assert!(state.remove_library(&library_id));
        assert_eq!(state.libraries().len(), 0);
        assert!(state.get_library(&library_id).is_none());
    }

    #[test]
    fn test_recent_directories() {
        let mut prefs = UserPreferences::default();

        prefs.add_recent_directory(Path::new("/path1").to_path_buf());
        prefs.add_recent_directory(Path::new("/path2").to_path_buf());
        prefs.add_recent_directory(Path::new("/path1").to_path_buf()); // Duplicate

        assert_eq!(prefs.recent_directories.len(), 2);
        assert_eq!(
            prefs.most_recent_directory(),
            Some(&Path::new("/path1").to_path_buf())
        );
    }

    #[test]
    fn test_view_type_display() {
        assert_eq!(ViewType::Library.display_name(), "Library");
        assert_eq!(ViewType::Settings.display_name(), "Settings");
        assert_eq!(ViewType::AudioProcessing.display_name(), "Audio Processing");
        assert_eq!(ViewType::About.display_name(), "About");
    }

    #[test]
    fn test_theme_config_display() {
        assert_eq!(ThemeConfig::System.display_name(), "System");
        assert_eq!(ThemeConfig::Light.display_name(), "Light");
        assert_eq!(ThemeConfig::Dark.display_name(), "Dark");
    }
}
