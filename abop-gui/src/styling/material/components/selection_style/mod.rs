//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.

mod lib;
pub mod strategy;
pub mod variants;
pub mod constants;
pub mod state;
pub mod colors;
pub mod builder;
pub mod functions;

// Re-export core types from lib
pub use lib::{
    BaseSelectionState, InteractionState, SelectionColors, SelectionSize, SelectionState,
    SelectionStyleBuilder, SelectionStyleError, SelectionStyling, SelectionVariant, checkbox_style,
    chip_style, radio_style, switch_style,
};

// Re-export strategy types
pub use strategy::{
    SelectionStyleContext, SelectionStyleStrategy, create_strategy,
};

// Re-export variant implementations
pub use variants::{
    CheckboxStrategy, ChipStrategy, RadioStrategy, SwitchStrategy,
};
