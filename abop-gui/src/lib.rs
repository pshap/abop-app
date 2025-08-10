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

// Constants used throughout the application
pub mod constants;

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

// Testing utilities and validation
#[cfg(test)]
pub mod testing;

/// Test utilities module for shared test fixtures and helpers
///
/// This module provides reusable test components, mock objects, and helper functions
/// for testing the ABOP GUI. It includes:
/// - Mock application state builders
/// - Test event generators
/// - Common assertion utilities
/// - Test renderers and simulators
/// - Example data factories
///
/// # Examples
/// ```no_run
/// use abop_gui::test_utils::{create_test_app, simulate_click};
/// # let _ = create_test_app();
/// ```
#[cfg(test)]
pub mod test_utils;

/// Re-exports of commonly used types and traits
///
/// This module provides convenient access to frequently used types from Iced
/// and application-specific modules. It's recommended to import this prelude
/// in your application code for better ergonomics.
///
/// # Example
/// ```no_run
/// use abop_gui::prelude::*;
/// ```
/// Prelude module for convenient imports
///
/// This module re-exports commonly used types and traits from Iced and
/// application-specific modules to reduce import boilerplate.
///
/// # Usage
/// ```no_run
/// use abop_gui::prelude::*;
///
/// // Now you have access to common Iced types and app-specific types
/// fn example() -> Element<'static, Message> {
///     Column::new()
///         .push(Text::new("Hello, ABOP!"))
///         .into()
/// }
/// ```
pub mod prelude {
    // Core Iced types
    pub use iced::{
        Alignment, Element, Length, Padding, Settings, Task, Theme,
        widget::{Button, Checkbox, Column, Container, PickList, Row, Text, TextInput},
    };

    // Application-specific types
    pub use crate::app::App;
    pub use crate::messages::Message;

    // Core domain types
    pub use crate::state::AppState;

    // Common traits
    pub use iced::Application;
    pub use iced::executor::Default;
}
