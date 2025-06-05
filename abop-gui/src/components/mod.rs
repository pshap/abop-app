//! UI component modules for the ABOP audiobook player interface
//!
//! This module organizes all UI components into logical submodules, each focused
//! on specific functionality within the application. All components follow consistent
//! patterns for theme integration and state management.

/// About dialog and application information display components
pub mod about;
/// Audio playback controls and manipulation widgets
pub mod audio_controls;
/// Audio toolbar component for playback controls
pub mod audio_toolbar;
/// Shared components used across multiple views and contexts
///
/// Contains Material Design 3 components organized into submodules:
/// - buttons: Button components and variants
/// - progress: Progress indicators and status components
/// - sizing: Component dimension constants and converters
/// - builders: Advanced builder patterns for complex components
pub mod common;
/// Icon support utilities for buttons and widgets
pub mod icon_support;
/// Icon utilities and Font Awesome integration
pub mod icons;
/// Unified main toolbar combining navigation and actions
pub mod main_toolbar;
/// Status display and information presentation widgets
pub mod status;
/// Table core functionality - main table component
pub mod table_core;
/// Table header component with sorting functionality
pub mod table_header;
/// Table row component for data display
pub mod table_row;
