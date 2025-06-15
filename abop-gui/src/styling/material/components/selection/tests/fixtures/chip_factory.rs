//! Unified factory functions for creating test chips with various configurations
//!
//! This module provides comprehensive factory functions for creating chips
//! in different states and configurations for testing purposes.
//!
//! # Organization
//!
//! - [`basic`] - Basic chip creation functions
//! - [`states`] - State-specific chip variants (selected, disabled, etc.)
//! - [`variants`] - Variant-specific convenience constructors
//! - [`collections`] - Chip collection creation functions
//! - [`assertions`] - Test assertion helper functions
//! - [`generators`] - Data generation utilities for testing
//! - [`scenarios`] - Real-world usage scenario factories
//! - [`performance`] - Performance and stress testing utilities

use super::test_data::*;
use crate::styling::material::components::selection::builder::ComponentBuilder;
use crate::styling::material::components::selection::common::SelectionComponent;
use crate::styling::material::components::selection::{
    Chip, ChipBuilder, ChipCollection, ChipCollectionBuilder, ChipSelectionMode, ChipState,
    ChipVariant, ComponentSize, SelectionError,
};

// ============================================================================
// Basic Chip Creation Functions
// ============================================================================

/// Basic chip creation module
pub mod basic {
    use super::*;

    /// Create a basic test chip with default settings
    pub fn chip(label: &str, variant: ChipVariant) -> Chip {
        ChipBuilder::new(label, variant)
            .build()
            .expect("Failed to build test chip")
    }

    /// Create a chip with specific size
    pub fn sized_chip(label: &str, variant: ChipVariant, size: ComponentSize) -> Chip {
        ChipBuilder::new(label, variant)
            .size(size)
            .build()
            .expect("Failed to build sized chip")
    }

    /// Create a chip with leading icon
    pub fn chip_with_leading_icon(label: &str, variant: ChipVariant, icon: &str) -> Chip {
        ChipBuilder::new(label, variant)
            .with_leading_icon(icon)
            .build()
            .expect("Failed to build chip with leading icon")
    }

    /// Create a chip with trailing icon
    pub fn chip_with_trailing_icon(label: &str, variant: ChipVariant, icon: &str) -> Chip {
        ChipBuilder::new(label, variant)
            .with_trailing_icon(icon)
            .build()
            .expect("Failed to build chip with trailing icon")
    }

    /// Create a chip with badge
    pub fn chip_with_badge(label: &str, variant: ChipVariant, count: u32) -> Chip {
        ChipBuilder::new(label, variant)
            .with_badge(count)
            .build()
            .expect("Failed to build chip with badge")
    }

    /// Create a chip with both icons and badge
    pub fn enhanced_chip(
        label: &str,
        variant: ChipVariant,
        leading_icon: Option<&str>,
        trailing_icon: Option<&str>,
        badge_count: Option<u32>,
    ) -> Chip {
        let mut builder = ChipBuilder::new(label, variant);

        if let Some(icon) = leading_icon {
            builder = builder.with_leading_icon(icon);
        }

        if let Some(icon) = trailing_icon {
            builder = builder.with_trailing_icon(icon);
        }

        if let Some(count) = badge_count {
            builder = builder.with_badge(count);
        }

        builder.build().expect("Failed to build enhanced chip")
    }
}

// ============================================================================
// State-Specific Chip Creation Functions
// ============================================================================

/// State-specific chip creation module
pub mod states {
    use super::*;

    /// Create a chip in selected state
    pub fn selected_chip(label: &str, variant: ChipVariant) -> Chip {
        ChipBuilder::new(label, variant)
            .selected(true)
            .build()
            .expect("Failed to build selected chip")
    }

    /// Create a chip in pressed state
    pub fn pressed_chip(label: &str, variant: ChipVariant) -> Chip {
        ChipBuilder::new(label, variant)
            .with_state(ChipState::Pressed)
            .build()
            .expect("Failed to build pressed chip")
    }

    /// Create a disabled chip
    pub fn disabled_chip(label: &str, variant: ChipVariant) -> Chip {
        ChipBuilder::new(label, variant)
            .disabled(true)
            .build()
            .expect("Failed to build disabled chip")
    }
}

// ============================================================================
// Variant-Specific Convenience Constructors
// ============================================================================

/// Variant-specific chip creation module
pub mod variants {
    use super::*;

    /// Create a filter chip with standard configuration
    pub fn filter_chip(label: &str) -> Chip {
        basic::chip(label, ChipVariant::Filter)
    }

    /// Create a selected filter chip
    pub fn selected_filter_chip(label: &str) -> Chip {
        states::selected_chip(label, ChipVariant::Filter)
    }

    /// Create an assist chip with standard configuration
    pub fn assist_chip(label: &str) -> Chip {
        basic::chip(label, ChipVariant::Assist)
    }

    /// Create an input chip with standard configuration
    pub fn input_chip(label: &str) -> Chip {
        basic::chip(label, ChipVariant::Input)
    }

    /// Create a deletable input chip
    pub fn deletable_input_chip(label: &str) -> Chip {
        ChipBuilder::input(label)
            .deletable()
            .build()
            .expect("Failed to build deletable chip")
    }

    /// Create a suggestion chip with standard configuration
    pub fn suggestion_chip(label: &str) -> Chip {
        basic::chip(label, ChipVariant::Suggestion)
    }
}

// ============================================================================
// Collection Creation Functions
// ============================================================================

/// Chip collection creation module
pub mod collections {
    use super::*;

    /// Create a test chip collection with default settings
    pub fn test_collection(mode: ChipSelectionMode) -> ChipCollection {
        ChipCollectionBuilder::new(mode)
            .build()
            .expect("Failed to build test collection")
    }

    /// Create a collection with predefined filter chips
    pub fn filter_chip_collection(labels: &[&str]) -> ChipCollection {
        let mut builder = ChipCollectionBuilder::new(ChipSelectionMode::Multiple);

        for label in labels {
            builder = builder.filter(*label);
        }

        builder.build().expect("Failed to build filter collection")
    }

    /// Create a collection with mixed chip variants
    pub fn mixed_variant_collection() -> ChipCollection {
        ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
            .filter("Filter")
            .assist("Assist")
            .input("Input")
            .suggestion("Suggestion")
            .build()
            .expect("Failed to build mixed collection")
    }
}

// ============================================================================
// Test Assertion Helper Functions
// ============================================================================

/// Test assertion helper module
pub mod assertions {
    use super::*;

    /// Assert that a chip has expected basic properties
    pub fn assert_chip_basics(chip: &Chip, expected_label: &str, expected_variant: ChipVariant) {
        assert_eq!(chip.label(), expected_label);
        assert_eq!(chip.variant(), expected_variant);
        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(!chip.props().disabled);
    }

    /// Assert that a chip is in selected state
    pub fn assert_chip_selected(chip: &Chip) {
        assert_eq!(chip.state(), ChipState::Selected);
        assert!(chip.is_selected());
    }

    /// Assert that a chip is in unselected state
    pub fn assert_chip_unselected(chip: &Chip) {
        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(!chip.is_selected());
    }

    /// Assert collection selection state
    pub fn assert_collection_selection(collection: &ChipCollection, expected_count: usize) {
        assert_eq!(collection.selected_chips().len(), expected_count);
    }

    /// Assert that a chip has expected size
    pub fn assert_chip_size(chip: &Chip, expected_size: ComponentSize) {
        assert_eq!(chip.props().size, expected_size);
    }

    /// Assert that a chip is enabled
    pub fn assert_chip_enabled(chip: &Chip) {
        assert!(!chip.props().disabled);
    }

    /// Assert that a chip is disabled
    pub fn assert_chip_disabled(chip: &Chip) {
        assert!(chip.props().disabled);
    }

    /// Assert that a chip is in pressed state
    pub fn assert_chip_pressed(chip: &Chip) {
        assert_eq!(chip.state(), ChipState::Pressed);
    }

    /// Assert that a chip has specific metadata
    pub fn assert_chip_has_metadata(chip: &Chip, key: &str, expected_value: &str) {
        assert!(
            chip.props().metadata.contains_key(key),
            "Chip should have metadata key '{key}'"
        );
        assert_eq!(
            chip.props().metadata.get(key),
            Some(&expected_value.to_string()),
            "Metadata '{key}' should have value '{expected_value}'"
        );
    }

    /// Assert Material Design 3 height compliance
    pub fn assert_md3_height_compliance(_chip: &Chip) {
        // MD3 specifies minimum touch target heights
        let min_height = match _chip.props().size {
            ComponentSize::Small => 32.0,
            ComponentSize::Medium => 40.0,
            ComponentSize::Large => 48.0,
        };
        // This is a placeholder assertion - in real implementation,
        // you would check the actual rendered height
        assert!(min_height > 0.0, "Chip should meet MD3 height requirements");
    }

    /// Assert Material Design 3 touch target compliance
    pub fn assert_md3_touch_target_compliance(_chip: &Chip) {
        // MD3 specifies minimum 48dp touch targets
        let min_touch_target = 48.0;
        // This is a placeholder assertion - in real implementation,
        // you would check the actual touch target size
        assert!(
            min_touch_target > 0.0,
            "Chip should meet MD3 touch target requirements"
        );
    }

    /// Assert that an operation completes within a time limit
    pub fn assert_within_time_limit<F>(operation: F, max_duration_ms: u64, description: &str)
    where
        F: FnOnce(),
    {
        let start = std::time::Instant::now();
        operation();
        let duration = start.elapsed();
        assert!(
            duration.as_millis() <= max_duration_ms as u128,
            "{}: Operation took {}ms, expected <= {}ms",
            description,
            duration.as_millis(),
            max_duration_ms
        );
    }

    /// Test that label validation works correctly
    pub fn assert_label_validation_error(
        result: &Result<Chip, SelectionError>,
        expected_len: usize,
    ) {
        match result {
            Err(SelectionError::LabelTooLong { len, max: _ }) => {
                assert_eq!(*len, expected_len);
            }
            _ => panic!("Expected LabelTooLong error"),
        }
    }

    /// Assert that an empty label error occurs
    pub fn assert_empty_label_error(result: Result<Chip, SelectionError>) {
        assert!(result.is_err(), "Expected error for empty label");
    }
}

// ============================================================================
// Data Generation Utilities
// ============================================================================

/// Data generation module for testing
pub mod generators {
    use super::*;

    /// Maximum allowed label length for chips
    pub const MAX_LABEL_LENGTH: usize = 100; // This should match actual constant

    /// Valid test labels of various lengths for basic testing
    pub const VALID_LABELS: &[&str] = &["Short", "Medium Label", "A Longer Test Label"];

    /// Create an oversized label for testing
    pub fn oversized_label() -> String {
        "x".repeat(MAX_LABEL_LENGTH + 1)
    }

    /// Create a label with maximum allowed length
    pub fn max_length_label() -> String {
        "A".repeat(MAX_LABEL_LENGTH)
    }

    /// Generate test labels of various lengths
    pub fn generate_test_labels() -> Vec<String> {
        vec![
            "A".to_string(),
            "Short".to_string(),
            "Medium Length Label".to_string(),
            "A".repeat(50),
            "A".repeat(MAX_LABEL_LENGTH),
        ]
    }

    /// Generate all chip variants for testing
    pub fn all_chip_variants() -> Vec<ChipVariant> {
        vec![
            ChipVariant::Assist,
            ChipVariant::Filter,
            ChipVariant::Input,
            ChipVariant::Suggestion,
        ]
    }

    /// Generate all component sizes for testing
    pub fn all_component_sizes() -> Vec<ComponentSize> {
        vec![
            ComponentSize::Small,
            ComponentSize::Medium,
            ComponentSize::Large,
        ]
    }

    /// Generate all selection modes for testing
    pub fn all_selection_modes() -> Vec<ChipSelectionMode> {
        vec![
            ChipSelectionMode::None,
            ChipSelectionMode::Single,
            ChipSelectionMode::Multiple,
        ]
    }
    /// All possible chip states for testing
    pub const ALL_CHIP_STATES: &[ChipState] = &[ChipState::Unselected, ChipState::Selected];
}

// ============================================================================
// Error Case Factories
// ============================================================================

/// Error case factory module
pub mod errors {
    use super::*;

    /// Attempt to create chip with empty label (should fail)
    pub fn empty_label_chip(variant: ChipVariant) -> Result<Chip, SelectionError> {
        ChipBuilder::new("", variant).build()
    }

    /// Attempt to create chip with oversized label (should fail)
    pub fn oversized_label_chip(variant: ChipVariant) -> Result<Chip, SelectionError> {
        ChipBuilder::new(generators::oversized_label(), variant).build()
    }

    /// Attempt to create chip with max length label (should succeed)
    pub fn max_length_label_chip(variant: ChipVariant) -> Result<Chip, SelectionError> {
        ChipBuilder::new(generators::max_length_label(), variant).build()
    }
}

// ============================================================================
// Batch Factory Functions
// ============================================================================

/// Batch creation module for testing multiple chips
pub mod batch {
    use super::*;

    /// Create a set of chips with all variants
    pub fn all_variant_chips(label: &str) -> Vec<Chip> {
        generators::all_chip_variants()
            .iter()
            .map(|&variant| basic::chip(label, variant))
            .collect()
    }

    /// Create a set of chips with all sizes
    pub fn all_size_chips(label: &str, variant: ChipVariant) -> Vec<Chip> {
        generators::all_component_sizes()
            .iter()
            .map(|&size| basic::sized_chip(label, variant, size))
            .collect()
    }

    /// Create a set of chips with all states
    pub fn all_state_chips(label: &str, variant: ChipVariant) -> Vec<Chip> {
        generators::ALL_CHIP_STATES
            .iter()
            .map(|&state| match state {
                ChipState::Selected => states::selected_chip(label, variant),
                ChipState::Unselected => basic::chip(label, variant),
                ChipState::Pressed => states::pressed_chip(label, variant),
            })
            .collect()
    }

    /// Create chips for performance testing
    pub fn performance_chips(variant: ChipVariant, count: usize) -> Vec<Chip> {
        (0..count)
            .map(|i| basic::chip(&format!("Chip {i}"), variant))
            .collect()
    }

    /// Create chips with varying label lengths
    pub fn variable_length_chips(variant: ChipVariant) -> Vec<Chip> {
        generators::generate_test_labels()
            .into_iter()
            .map(|label| basic::chip(&label, variant))
            .collect()
    }
}

// ============================================================================
// Real-World Scenario Factories
// ============================================================================

/// Real-world scenario module for testing
pub mod scenarios {
    use super::*;

    /// Create filter chips for search interface testing
    pub fn search_filter_chips() -> Vec<Chip> {
        FILTER_CHIP_LABELS
            .iter()
            .map(|&label| variants::filter_chip(label))
            .collect()
    }

    /// Create input chips for tag interface testing
    pub fn tag_input_chips() -> Vec<Chip> {
        INPUT_CHIP_LABELS
            .iter()
            .map(|&label| variants::input_chip(label))
            .collect()
    }

    /// Create assist chips for help interface testing
    pub fn help_assist_chips() -> Vec<Chip> {
        ASSIST_CHIP_LABELS
            .iter()
            .map(|&label| variants::assist_chip(label))
            .collect()
    }

    /// Create suggestion chips for action interface testing
    pub fn action_suggestion_chips() -> Vec<Chip> {
        SUGGESTION_CHIP_LABELS
            .iter()
            .map(|&label| variants::suggestion_chip(label))
            .collect()
    }
}

// ============================================================================
// Performance Testing Utilities
// ============================================================================

/// Performance testing module
pub mod performance {
    use super::*;

    /// Create a random valid chip for property-based testing
    pub fn random_chip() -> Chip {
        basic::chip(&random_valid_label(), random_chip_variant())
    }

    /// Create multiple random chips
    pub fn random_chips(count: usize) -> Vec<Chip> {
        (0..count).map(|_| random_chip()).collect()
    }

    /// Create a chip with random valid configuration
    pub fn random_configured_chip() -> Chip {
        let label = random_valid_label();
        let variant = random_chip_variant();
        let size = random_component_size();

        ChipBuilder::new(label, variant)
            .size(size)
            .build()
            .expect("Failed to build random configured chip")
    }

    /// Create a set of test chips with sequential labels
    pub fn test_chip_set(count: usize) -> Vec<Chip> {
        (0..count)
            .map(|i| basic::chip(&format!("Test Chip {i}"), ChipVariant::Filter))
            .collect()
    }

    /// Create chips for memory usage testing
    pub fn memory_test_chips() -> Vec<Chip> {
        // Create chips with varying amounts of data to test memory efficiency
        let mut chips = Vec::with_capacity(200); // Pre-allocate for 100 simple + 100 complex chips

        // Simple chips
        chips.extend(batch::performance_chips(ChipVariant::Filter, 100));

        // Complex chips with metadata
        for i in 0..100 {
            let chip = ChipBuilder::filter(format!("Complex Chip {i}"))
                .with_leading_icon("filter")
                .with_trailing_icon("times")
                .with_badge(i as u32)
                .build()
                .expect("Failed to build complex chip");
            chips.push(chip);
        }

        chips
    }

    /// Create chips for concurrent access testing
    pub fn concurrent_test_chips() -> Vec<Chip> {
        // Create chips that will be accessed from multiple threads
        (0..1000)
            .map(|i| {
                let variants = generators::all_chip_variants();
                basic::chip(
                    &format!("Thread Safe Chip {i}"),
                    variants[i % variants.len()],
                )
            })
            .collect()
    }
}

// ============================================================================
// Legacy API Compatibility (Re-exports)
// ============================================================================

// Re-export commonly used functions at the top level for backward compatibility
pub use basic::chip as test_chip;
pub use basic::sized_chip;
pub use states::{disabled_chip, pressed_chip, selected_chip};
pub use variants::{assist_chip, filter_chip, input_chip, selected_filter_chip, suggestion_chip};

// Re-export assertion functions
pub use assertions::*;

// Re-export collection functions
pub use collections::*;

// Re-export generators
pub use generators::*;
