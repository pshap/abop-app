//! Material Design 3 Selection Component Styling System
//!
//! This module provides a sophisticated styling system for all selection components,
//! implementing the strategy pattern used throughout the Material Design system.
//! Eliminates code duplication across MaterialCheckbox, MaterialRadio, MaterialSwitch, and MaterialChip.
//!
//! ## Phase 1 Architectural Improvements
//! - Strategy pattern implementation for consistent architecture
//! - Complete Material Design 3 state system integration
//! - Enhanced error handling and validation
//! - Proper integration with MaterialTokens
//! - Type-safe styling with comprehensive state management
//! - Builder pattern support with fluent construction
//! - Full theme integration with Material Design 3 specifications
//!
//! ## Integration with Material Strategy System
//! This implementation follows the same architectural patterns as the button styling
//! system, ensuring consistency across all Material components.

// Re-export sibling modules
pub use super::constants;
pub use super::state;
pub use super::colors;
pub use super::builder;
pub use super::functions;

// Re-export commonly used types
pub use state::{
    BaseSelectionState, InteractionState, SelectionSize, SelectionState,
    SelectionStyleError, SelectionStyling, SelectionVariant,
};
pub use colors::SelectionColors;
pub use builder::SelectionStyleBuilder;
pub use functions::{checkbox_style, chip_style, radio_style, switch_style};
