//! Modular styling system for ABOP GUI
//!
//! This module provides a comprehensive styling system organized into logical components.
//! Each submodule focuses on a specific aspect of UI styling with consistent design token usage.

pub mod color_utils;
pub mod container;
pub mod design_tokens;
pub mod dynamic_themes;
pub mod input;
pub mod material;
pub mod plugins;
pub mod scrollable;
pub mod strategy;
pub mod testing;
pub mod traits;
pub mod utils;
pub mod validation;

// Re-export commonly used styling types for convenience
pub use color_utils::ColorUtils;
pub use material::MaterialTokens;
pub use strategy::{ButtonStyleVariant, ComponentState, ComponentStyleStrategy, ComponentStyling};
pub use traits::{ComponentStyle, StyleBuilder, StyleVariant, ThemeAware};
