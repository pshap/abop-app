//! Application state management for the GUI
//!
//! This module provides a clean, domain-separated state management system:
//!
//! ## State Architecture
//!
//! - **`AppState`**: Main application state container with domain separation
//! - **`UiState`**: Pure UI concerns (theme, dialogs, rendering flags)
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

/// Backward compatibility alias for the old monolithic state structure.
///
/// This alias provides compatibility for existing code during the migration from
/// the monolithic `UiState` to the new domain-separated `AppState` architecture.
///
/// # Migration Path
///
/// Replace `UiState` usage with `AppState` and update field access patterns:
/// ```ignore
/// // Old monolithic access
/// let theme = state.theme_mode;
/// let books = state.audiobooks;
///
/// // New domain-separated access  
/// let theme = state.ui.theme_mode;
/// let books = state.library.audiobooks;
/// ```
///
/// This alias will be removed in version 0.2.0.
#[deprecated(
    since = "0.1.0",
    note = "Use AppState instead. This alias will be removed in v0.2.0. See migration documentation for field access changes."
)]
pub use AppStateContainer as UiState;
