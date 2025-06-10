//! Modern Material Design 3 Chip Implementation
//!
//! This module provides a completely redesigned chip component system with:
//! - State-based design using ChipState enum
//! - Multiple chip variants (Assist, Filter, Input, Suggestion)
//! - Built-in validation and error handling
//! - Animation support for smooth transitions
//! - Modern builder pattern with fluent API
//! - Unified view methods for consistent rendering
//!
//! # Phase 2 Architectural Improvements
//!
//! The chip system has been split into focused submodules:
//! - `core`: Core chip implementation and state management
//! - `collection`: Chip collection management and selection modes
//! - `view`: Unified view methods for different chip rendering modes
//! - `builder`: Enhanced builder patterns with improved ergonomics

// Core submodules
pub mod builder;
pub mod collection;
pub mod core;
pub mod view;

// Re-export core types for backward compatibility
pub use self::builder::{ChipBuilder, ChipCollectionBuilder};
pub use self::collection::{ChipCollection, ChipSelectionMode};
pub use super::builder::Chip;
pub use super::common::{ChipState, ChipVariant, ComponentSize, SelectionError};

// Re-export convenience functions
pub use self::builder::{filter_chip_collection, single_select_chip_collection};

// Constants (moved to core)
pub use self::core::{DEFAULT_ANIMATION_DURATION, MAX_CHIP_LABEL_LENGTH};

// ============================================================================
// Module-level convenience functions for quick chip creation
// ============================================================================

/// Create a filter chip with the given label
#[must_use]
pub fn filter_chip<S: Into<String>>(label: S) -> ChipBuilder {
    Chip::filter(label)
}

/// Create an assist chip with the given label  
#[must_use]
pub fn assist_chip<S: Into<String>>(label: S) -> ChipBuilder {
    Chip::assist(label)
}

/// Create an input chip with the given label
#[must_use]
pub fn input_chip<S: Into<String>>(label: S) -> ChipBuilder {
    Chip::input(label)
}

/// Create a suggestion chip with the given label
#[must_use]
pub fn suggestion_chip<S: Into<String>>(label: S) -> ChipBuilder {
    Chip::suggestion(label)
}
