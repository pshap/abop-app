//! Core UI state management types and enums

use crate::models::{Audiobook, Library, Progress};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::constants::*;

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
            width: DEFAULT_WINDOW_WIDTH,
            height: DEFAULT_WINDOW_HEIGHT,
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
            volume: DEFAULT_VOLUME,
            speed: DEFAULT_PLAYBACK_SPEED,
            auto_resume: true,
            skip_amount: DEFAULT_SKIP_AMOUNT,
            auto_bookmark: false,
        }
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
        // Keep only last MAX_RECENT_DIRECTORIES
        self.recent_directories.truncate(MAX_RECENT_DIRECTORIES);
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
