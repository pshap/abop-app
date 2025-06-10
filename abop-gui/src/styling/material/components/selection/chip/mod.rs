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
//!
//! # Examples
//!
//! ## Basic Chip Creation
//!
//! ```rust
//! use crate::styling::material::components::selection::chip::*;
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
//!     .removable(true)
//!     .build();
//! ```
//!
//! ## Collection Management
//!
//! ```rust
//! use crate::styling::material::components::selection::chip::*;
//!
//! // Create a single-select chip collection
//! let mut collection = single_select_chip_collection(vec![
//!     filter_chip("Option 1").build(),
//!     filter_chip("Option 2").build(),
//!     filter_chip("Option 3").build(),
//! ]);
//!
//! // Toggle selection (automatically handles single-select logic)
//! collection.toggle_chip(1).unwrap();
//! assert_eq!(collection.selected_count(), 1);
//!
//! // Create a multi-select collection
//! let mut multi_collection = filter_chip_collection(vec![
//!     filter_chip("Feature 1").build(),
//!     filter_chip("Feature 2").build(),
//! ]);
//!
//! // Select multiple chips
//! multi_collection.select_chip(0).unwrap();
//! multi_collection.select_chip(1).unwrap();
//! assert_eq!(multi_collection.selected_count(), 2);
//! ```
//!
//! ## Enhanced UI with Icons and Badges
//!
//! ```rust
//! use crate::styling::material::components::selection::chip::*;
//!
//! let enhanced_chip = filter_chip("Messages")
//!     .with_leading_icon("email")
//!     .with_badge("5")
//!     .with_badge_color("primary")
//!     .build();
//!
//! // Render with enhanced configuration
//! let config = ChipViewConfig {
//!     show_icons: true,
//!     show_badges: true,
//!     badge_color: Some(MaterialColors::primary()),
//!     icon_size: 16.0,
//!     badge_size: 12.0,
//! };
//!
//! let element = enhanced_chip.view_enhanced(&config, &color_scheme);
//! ```
//!
//! ## Collection Layout and Rendering
//!
//! ```rust
//! use crate::styling::material::components::selection::chip::*;
//!
//! let collection = filter_chip_collection(vec![
//!     filter_chip("Technology").build(),
//!     filter_chip("Science").build(),
//!     filter_chip("Art").build(),
//! ]);
//!
//! // Render with different layouts
//! let row_view = collection.view_with_layout(
//!     ChipCollectionLayout::Row,
//!     &color_scheme
//! );
//!
//! let wrap_view = collection.view_with_layout(
//!     ChipCollectionLayout::Wrap,
//!     &color_scheme
//! );
//!
//! let grid_view = collection.view_with_layout(
//!     ChipCollectionLayout::Grid,
//!     &color_scheme
//! );
//! ```

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
