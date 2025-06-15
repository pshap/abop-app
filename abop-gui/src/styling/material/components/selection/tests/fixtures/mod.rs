//! Test fixtures for comprehensive chip testing
//!
//! This module provides organized test utilities, factories, and assertion helpers
//! for creating and validating chip components and collections.

pub mod assertion_helpers;
pub mod chip_factory;
pub mod collection_factory;
pub mod test_data;

// Re-export specific items to avoid naming conflicts
pub use chip_factory::{
    // Assertion functions
    assertions::*,

    // Basic chip creation
    basic::{chip, chip_with_badge, chip_with_leading_icon, chip_with_trailing_icon, sized_chip},

    disabled_chip,
    // Error variants
    errors::{empty_label_chip, max_length_label_chip, oversized_label_chip},

    pressed_chip,
    selected_chip,
    // Top-level convenience functions
    test_chip,
    // State variants
    variants::{
        assist_chip, deletable_input_chip, filter_chip, input_chip, selected_filter_chip,
        suggestion_chip,
    },
};

// Re-export collection functions that don't conflict
pub use collection_factory::{
    assist_only_collection, comfortable_collection, compact_collection, disabled_collection,
    display_only_collection, filter_collection, filter_only_collection, input_only_collection,
    large_performance_collection, max_selected_collection, medium_performance_collection,
    mixed_variant_collection, multiple_select_collection, pre_selected_collection,
    search_interface_collection, single_select_collection, small_performance_collection,
    stress_test_collection, suggestion_only_collection, test_collection, uniform_size_collection,
};

// Re-export test data constants and functions
pub use test_data::*;
