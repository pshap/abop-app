//! Material Design 3 Widget Implementations
//!
//! This module contains actual Iced widget implementations of Material Design 3 components.
//! Each widget is a proper Iced Widget that can be used directly in Iced applications.

pub mod material_button;

// Re-export MaterialButton widget components and shared types
pub use material_button::{ButtonSize, IconPosition, MaterialButton, MaterialButtonVariant};
