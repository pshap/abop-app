//! ABOP GUI - Iced-based GUI for the Audiobook Organizer & Processor
//!
//! This crate provides the graphical user interface for the ABOP application.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

// Main application module
pub mod app;

// Application router for view navigation
pub mod router;

// Asset management (including fonts)
pub mod assets;

// Message and command definitions
pub mod messages;

/// Command handling modules for async operations
pub mod commands;

/// Message handling modules for UI state management
pub mod handlers;

/// Application state management
pub mod state;

/// Update logic and state transitions
pub mod update;

// View modules
pub mod views;

// Audio processing and playback
pub mod audio;

// Library scanning and management
pub mod library;

// Utility functions
pub mod utils;

// Component modules
/// UI component modules for the ABOP graphical interface
///
/// This module contains all reusable UI components organized by functionality. Each submodule
/// provides a set of widgets or view functions for a specific part of the application, such as
/// navigation, dialogs, status displays, and toolbars. Components are designed for theme and state
/// integration, and follow the Iced widget pattern for composability.
///
/// # Modules
/// - `about` - About dialog and application information
/// - `audio_controls` - Playback controls and audio manipulation widgets
/// - `common` - Shared components used across multiple views
/// - `dialogs` - Modal dialogs and settings interfaces
/// - `navigation` - Navigation bar and view switching components
/// - `status` - Status display and information widgets
/// - `table` - Table components for audiobook library display
/// - `toolbar` - Toolbar components with action buttons
pub mod components;

// Professional theme system
pub mod theme;

// Professional styling components
pub mod styling;

// Re-exports (simplified for new implementation)

/// Prelude module for convenient imports
pub mod prelude {
    pub use iced::{
        Alignment, Element, Length, Padding, Settings, Theme,
        widget::{Button, Checkbox, Column, Container, PickList, Row, Text, TextInput},
    };

    // Re-export the new message and app types
    pub use crate::app::App;
    pub use crate::messages::Message;
    pub use abop_core::AppState;
}
