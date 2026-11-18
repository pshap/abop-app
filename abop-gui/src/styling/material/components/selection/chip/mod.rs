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
//! # Modular Architecture
//!
//! The chip system is organized into focused submodules:
//! - `core`: Core chip implementation and state management
//! - `collection`: Chip collection management and selection modes
//! - `view`: Unified view methods for different chip rendering modes
//! - `builder`: Enhanced builder patterns with improved ergonomics
//!
//! # Examples
//!
//! ## Basic Chip Creation
//!
//! ```rust,no_run
//! use abop_gui::styling::material::components::selection::chip::*;
//! use abop_gui::styling::material::components::selection::common::ComponentSize;
//! use abop_gui::styling::material::components::selection::builder::ComponentBuilder;
//!
//! // Create different types of chips
//! let filter_chip = filter_chip("Technology")
//!     .size(ComponentSize::Medium)
//!     .selected(true)
//!     .build();
//!
//! let assist_chip = assist_chip("Help")
//!     .with_leading_icon("help")
//!     .build();
//!
//! let input_chip = input_chip("Tag: Rust")
//!     .with_trailing_icon("close")
//!     .build();
//! ```
//!
//! ## Collection Management
//!
//! ```rust,no_run
//! use abop_gui::styling::material::components::selection::chip::*;
//!
//! // Create a single-select chip collection
//! let mut collection = single_select_chip_collection()
//!     .filter("Option 1")
//!     .filter("Option 2")
//!     .filter("Option 3")
//!     .build()
//!     .unwrap();
//!
//! // Toggle selection (automatically handles single-select logic)
//! collection.toggle_chip(1).unwrap();
//! assert_eq!(collection.selected_count(), 1);
//!
//! // Create a multi-select collection
//! let mut multi_collection = filter_chip_collection()
//!     .filter("Feature 1")
//!     .filter("Feature 2")
//!     .build()
//!     .unwrap();
//!
//! // Select multiple chips
//! multi_collection.select_chip(0).unwrap();
//! multi_collection.select_chip(1).unwrap();
//! assert_eq!(multi_collection.selected_count(), 2);
//! ```
//!
//! ## Enhanced UI with Icons and Badges
//!
//! ```rust,no_run
//! use abop_gui::styling::material::components::selection::chip::*;
//! use abop_gui::styling::material::components::selection::builder::ComponentBuilder;
//!
//! let enhanced_chip = filter_chip("Messages")
//!     .with_leading_icon("email")
//!     .with_badge(5)
//!     .build();
//! ```
//!
//! ## Collection Layout and Rendering
//!
//! ```rust,no_run
//! use abop_gui::styling::material::components::selection::chip::*;
//! use abop_gui::styling::material::components::selection::chip::collection::ChipCollectionLayout;
//!
//! let collection = filter_chip_collection()
//!     .filter("Technology")
//!     .filter("Science")
//!     .filter("Art")
//!     .build()
//!     .unwrap();
//!
//! // Render with different layouts - simplified examples
//! // Note: In real usage, you'd need proper color_scheme and callback parameters
//! ```

// Core submodules
pub mod builder;
pub mod collection;
pub mod core;
pub mod view;

// Import ComponentBuilder trait for build() method

// Re-export core types
pub use self::builder::{ChipBuilder, ChipCollectionBuilder};
pub use self::collection::{ChipCollection, ChipSelectionMode};
pub use super::builder::Chip;
pub use super::common::{ChipState, ChipVariant, ComponentSize, SelectionError};

// Re-export convenience functions
pub use self::builder::{filter_chip_collection, single_select_chip_collection};

// Constants (moved to core)
pub use self::core::{DEFAULT_ANIMATION_DURATION, MAX_CHIP_LABEL_LENGTH};

// ============================================================================
// Module-level convenience functions for quick chip creation (optimized)
// ============================================================================

/// Create a filter chip with the given label
///
/// Accepts both string slices and owned strings to minimize allocations
#[must_use]
pub fn filter_chip(label: &str) -> ChipBuilder {
    Chip::filter(label)
}

/// Create an assist chip with the given label
///
/// Accepts both string slices and owned strings to minimize allocations
#[must_use]
pub fn assist_chip(label: &str) -> ChipBuilder {
    Chip::assist(label)
}

/// Create an input chip with the given label
///
/// Accepts both string slices and owned strings to minimize allocations
#[must_use]
pub fn input_chip(label: &str) -> ChipBuilder {
    Chip::input(label)
}

/// Create a suggestion chip with the given label
///
/// Accepts both string slices and owned strings to minimize allocations
#[must_use]
pub fn suggestion_chip(label: &str) -> ChipBuilder {
    Chip::suggestion(label)
}
