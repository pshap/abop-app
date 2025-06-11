//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.

mod lib;
pub mod strategy;

// Re-export core types from lib
pub use lib::{
    SelectionColors, SelectionSize, SelectionState, SelectionStyling, 
    SelectionStyleBuilder, SelectionStyleError, SelectionVariant,
    BaseSelectionState, InteractionState,
    checkbox_style, radio_style, chip_style, switch_style,
};

// Re-export strategy types
pub use strategy::{
    SelectionStyleStrategy, SelectionStyleContext,
    CheckboxStrategy, RadioStrategy, ChipStrategy, SwitchStrategy,
    create_strategy,
};
