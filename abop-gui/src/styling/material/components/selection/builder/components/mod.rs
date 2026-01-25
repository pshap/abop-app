//! Component Struct Definitions
//!
//! This module contains actual component structs that are built by builders.
//! These represent final constructed Material Design 3 selection components.
//!
//! ## Components
//! - [`Checkbox`] - Material Design 3 checkbox component
//! - [`Radio`] - Material Design 3 radio button component
//! - [`Switch`] - Material Design 3 switch component
//! - [`Chip`] - Material Design 3 chip component

// Re-export all component modules
pub mod checkbox;
pub mod chip;
pub mod radio;
pub mod switch;
#[cfg(test)]
pub mod tests;
pub mod tokens;

// Re-export all components for convenient use
pub use checkbox::Checkbox;
pub use chip::Chip;
pub use radio::Radio;
pub use switch::Switch;
pub use tokens::{DARK_TOKENS, LIGHT_TOKENS};

// Import common traits and types for the components
use crate::styling::material::components::selection::common::prelude::*;
use crate::styling::material::components::selection_style::{
    SelectionSize as LegacySelectionSize, SelectionStyleBuilder, SelectionVariant,
};
use crate::styling::material::{tokens::core::MaterialTokens, MaterialColors};
use iced::{theme::Theme, Element, Renderer};
