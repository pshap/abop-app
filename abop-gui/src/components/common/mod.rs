//! Common UI builders for reusable Material Design 3 components
//!
//! This module provides high-level builder functions for creating Material Design 3
//! compliant UI components, primarily buttons and FABs. It uses a consistent builder
//! pattern interface over the lower-level MaterialButton widget implementation.
//!
//! # Design Philosophy
//! - Use a builder pattern for clear, flexible component configuration
//! - Maintain Material Design 3 specifications for sizing and behavior
//! - Offer both simple semantic functions and advanced configuration options
//! - Ensure consistent styling across the application
//!
//! # Usage Examples
//! ```no_run
//! use abop_gui::components::common::*;
//! use abop_gui::styling::material::MaterialTokens;
//! use abop_gui::styling::material::components::widgets::IconPosition;
//! use abop_gui::styling::material::{MaterialButtonVariant, ButtonSize};
//! use iced::Length;
//!
//! #[derive(Debug, Clone)]
//! enum AppMessage {
//!     Save,
//!     Export,
//!     Custom,
//! }
//!
//! let tokens = MaterialTokens::default();
//!
//! // Simple primary action using semantic function
//! let save_btn = primary_button_semantic("Save", AppMessage::Save, &tokens);
//!
//! // Button with icon using semantic function
//! let icon_btn = primary_button_with_icon_semantic(
//!     "Export",
//!     "download",
//!     IconPosition::Leading,
//!     AppMessage::Export,
//!     &tokens
//! );
//!
//! // Advanced configuration using the builder pattern
//! let custom_btn = button_builder(&tokens)
//!     .label("Custom")
//!     .icon("settings", IconPosition::Leading)
//!     .variant(MaterialButtonVariant::Outlined)
//!     .size(ButtonSize::Large)
//!     .width(Length::Fixed(200.0))
//!     .on_press(AppMessage::Custom)
//!     .build();
//! ```

// Re-export submodules
pub mod progress;
pub mod sizing;

// Re-export commonly used items for convenience
pub use self::progress::*;
pub use self::sizing::*;
