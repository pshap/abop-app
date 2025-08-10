//! Refactored state management with domain separation
//!
//! This module provides a clean separation of concerns for application state:
//! - UiState: Pure UI concerns (dialogs, theme, rendering)
//! - LibraryState: Library management (audiobooks, directories, scanning)
//! - PlayerState: Audio playback state
//! - TaskState: Background task management  
//! - ProgressCache: Performance optimization for progress display

pub mod ui_state;
pub mod library_state;
pub mod player_state;
pub mod task_state;
pub mod progress_cache;

// Re-export the main state types
pub use ui_state::UiState;
pub use library_state::{LibraryState, DirectoryInfo, TableState};
pub use player_state::PlayerState;
pub use task_state::{TaskState, TaskInfo, TaskType};
pub use progress_cache::ProgressCache;

use abop_core::models::AppState;

/// Consolidated application state with domain separation
#[derive(Clone)]
pub struct AppStateContainer {
    /// Core application state from abop-core
    pub core_state: AppState,
    /// UI-specific state (theme, dialogs, rendering)
    pub ui: UiState,
    /// Library management state (audiobooks, directories, scanning)
    pub library: LibraryState,
    /// Audio player state
    pub player: PlayerState,
    /// Background task management
    pub tasks: TaskState,
    /// Progress text caching for performance
    pub progress_cache: ProgressCache,
}

impl AppStateContainer {
    /// Create new state container from core state
    #[must_use]
    pub fn from_core_state(core_state: AppState) -> Self {
        let ui = UiState::default();
        let library = LibraryState::from_core_state(&core_state);
        let player = PlayerState::default();
        let tasks = TaskState::default();
        let progress_cache = ProgressCache::default();

        Self {
            core_state,
            ui,
            library,
            player,
            tasks,
            progress_cache,
        }
    }

    /// Create state container with synchronized metadata
    #[must_use]
    pub fn from_core_state_synced(core_state: AppState) -> Self {
        let mut container = Self::from_core_state(core_state);
        container.library.sync_directory_metadata();
        container
    }

    /// Update theme across all relevant state domains
    pub fn set_theme_mode(&mut self, theme_mode: crate::theme::ThemeMode) {
        self.ui.set_theme_mode(theme_mode);
    }

    /// Check if any state domain needs a UI redraw
    pub fn needs_redraw(&self) -> bool {
        self.ui.needs_redraw()
            || self.library.needs_redraw()
            || self.player.needs_redraw()
            || self.tasks.needs_redraw()
    }

    /// Clear all redraw flags
    pub fn clear_redraw_flags(&mut self) {
        self.ui.clear_redraw_flag();
        self.library.clear_redraw_flag();
        self.player.clear_redraw_flag();
        self.tasks.clear_redraw_flag();
    }
}

impl Default for AppStateContainer {
    fn default() -> Self {
        Self::from_core_state_synced(AppState::default())
    }
}

impl std::fmt::Debug for AppStateContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppStateContainer")
            .field("core_state", &self.core_state)
            .field("ui", &self.ui)
            .field("library", &self.library)
            .field("player", &self.player)
            .field("tasks", &self.tasks)
            .field("progress_cache", &"<ProgressCache>")
            .finish()
    }
}