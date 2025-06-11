//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.

mod lib;
pub mod strategy;

// Re-export core types from lib
pub use lib::{
    BaseSelectionState, InteractionState, SelectionColors, SelectionSize, SelectionState,
    SelectionStyleBuilder, SelectionStyleError, SelectionStyling, SelectionVariant, checkbox_style,
    chip_style, radio_style, switch_style,
};

// Re-export strategy types
pub use strategy::{
    CheckboxStrategy, ChipStrategy, RadioStrategy, SelectionStyleContext, SelectionStyleStrategy,
    SwitchStrategy, create_strategy,
};
