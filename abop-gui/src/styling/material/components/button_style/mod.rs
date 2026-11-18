//! Button Style Module
//!
//! Modular structure for button styling that maintains the existing public API
//! while providing improved organization and maintainability.

pub mod colors;
pub mod constants;
pub mod functions;
pub mod sizing;
pub mod strategy;
pub mod variants;

// Re-export the main button style types to maintain API compatibility
pub use colors::ButtonColors;
pub use constants::*;
pub use functions::{
    ButtonStyling, create_button_icon, create_button_style, get_button_size_properties,
    get_button_styling, get_icon_size_for_button,
};
pub use variants::{ButtonSizeVariant, ButtonStyleVariant};
