//! Common UI components for reusable Material Design 3 elements
//!
//! This module provides shared components that are used across multiple views
//! and contexts within the application. Currently contains:
//!
//! - **progress**: Progress indicators including circular progress bars, linear progress bars,
//!   and loading spinners for providing visual feedback during asynchronous operations
//!   and background tasks
//!
//! # Design Philosophy
//! - Follow Material Design 3 specifications for consistency
//! - Provide reusable components to avoid code duplication
//! - Ensure proper theming and accessibility support
//!
//! # Usage Examples
//! ```no_run
//! use abop_gui::components::common::create_progress_indicator;
//! use abop_gui::styling::material::MaterialTokens;
//! use abop_gui::theme::ThemeMode;
//!
//! let tokens = MaterialTokens::default();
//! let progress = create_progress_indicator(
//!     Some(0.5),
//!     "Loading...",
//!     ThemeMode::Light,
//!     &tokens
//! );
//! ```
//!
//! Note: Button-related functionality has been moved to the dedicated `buttons` module
//! for better organization and more comprehensive Material Design 3 support.

// Re-export submodules
pub mod progress;

// Re-export commonly used items for convenience
pub use self::progress::*;
