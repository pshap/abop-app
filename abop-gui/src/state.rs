//! Application state management for the GUI
//!
//! This module provides a clean, domain-separated state management system:
//!
//! ## State Architecture
//!
//! - **`AppState`**: Main application state container with domain separation
//! - **`UiState`**: Pure UI concerns (theme, dialogs, rendering flags) - now part of AppState
//! - **`LibraryState`**: Library management (audiobooks, directories, scanning)
//! - **`PlayerState`**: Audio playback state
//! - **`TaskState`**: Background task management
//! - **`ProgressCache`**: Performance optimization for progress text formatting
//!
//! ## Usage
//!
//! ```ignore
//! use crate::state::AppState;
//!
//! // Access different state domains
//! let theme = &app_state.ui.theme_mode;
//! let audiobooks = &app_state.library.audiobooks;
//! let player_status = &app_state.player.player_state;
//! let active_task = app_state.tasks.active_task();
//! ```
//!
//! ## Benefits
//!
//! - **Maintainability**: State organized by domain instead of monolithic structure
//! - **Performance**: Domain-specific `needs_redraw` flags and optimized caching
//! - **Type Safety**: Clear separation prevents cross-domain state corruption
//! - **Scalability**: New features can be added to appropriate domains

// Import all the refactored state components
pub use self::state_components::*;

#[path = "state_refactored/mod.rs"]
mod state_components;

pub use AppStateContainer as AppState;

