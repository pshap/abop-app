//! Modular data models for ABOP
//!
//! This module provides a well-organized collection of data models split by domain:
//! - Core business models (audiobooks, libraries, progress)
//! - UI-specific models (application state, view types)
//! - Configuration models (user preferences, themes)

pub mod audiobook;
pub mod core;
pub mod library;
pub mod progress;
pub mod search;
pub mod ui;

// Re-export commonly used types for convenience
pub use audiobook::Audiobook;
pub use core::Chapter;
pub use library::Library;
pub use progress::Progress;
pub use search::{SearchQuery, SearchResult};
pub use ui::{
    AppData, AppState, PlaybackConfig, ThemeConfig, UserPreferences, ViewType, WindowConfig,
};
