//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.

pub mod builder;
pub mod colors;
pub mod constants;
pub mod functions;
mod lib;
pub mod state;
pub mod strategy;
pub mod variants;

// Re-export core types from lib
pub use lib::{
    BaseSelectionState, InteractionState, SelectionColors, SelectionSize, SelectionState,
    SelectionStyleBuilder, SelectionStyleError, SelectionStyling, SelectionVariant, checkbox_style,
    chip_style, radio_style, switch_style,
};

// Re-export strategy types
pub use strategy::{SelectionStyleContext, SelectionStyleStrategy, create_strategy};

// Export variant implementations
pub use variants::{CheckboxStrategy, ChipStrategy, RadioStrategy, SwitchStrategy};
