//! UI state and configuration models module
//!
//! This module provides refactored UI state management with better separation of concerns:
//! - Core state management (AppState)
//! - State persistence operations (StatePersistence)
//! - Data repository management (DataRepository)
//! - Configuration and preferences (UserPreferences, ViewType, etc.)

mod constants;
mod data_repository;
mod persistence;
mod state;
mod types;

// Re-export main types for backward compatibility
pub use state::AppState;
pub use types::{
    AppData, PlaybackConfig, ThemeConfig, UserPreferences, ViewType, WindowConfig,
};

// Re-export new interfaces
pub use data_repository::DataRepository;
pub use persistence::{SaveOptions, StatePersistence};
pub use constants::*;
