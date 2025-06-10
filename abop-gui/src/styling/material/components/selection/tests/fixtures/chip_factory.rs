//! Factory functions for creating test chips with various configurations
//!
//! This module provides comprehensive factory functions for creating chips
//! in different states and configurations for testing purposes.

use crate::styling::material::components::selection::{
    Chip, ChipBuilder, ChipVariant, ChipState, ComponentSize, SelectionError,
};
use crate::styling::material::components::selection::builder::ComponentBuilder;
use super::test_data::*;

// ============================================================================
// Basic Chip Factories
// ============================================================================

/// Create a basic test chip with default settings
pub fn test_chip(label: &str, variant: ChipVariant) -> Chip {
    ChipBuilder::new(label, variant)
        .build()
        .expect("Failed to build test chip")
}

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

/// Create a chip with specific size
pub fn sized_chip(label: &str, variant: ChipVariant, size: ComponentSize) -> Chip {
    ChipBuilder::new(label, variant)
        .size(size)
        .build()
        .expect("Failed to build sized chip")
}

// ============================================================================
// Variant-Specific Factories
// ============================================================================

/// Create a filter chip with standard configuration
pub fn filter_chip(label: &str) -> Chip {
    test_chip(label, ChipVariant::Filter)
}

/// Create a selected filter chip
pub fn selected_filter_chip(label: &str) -> Chip {
    selected_chip(label, ChipVariant::Filter)
}

/// Create an assist chip with standard configuration
pub fn assist_chip(label: &str) -> Chip {
    test_chip(label, ChipVariant::Assist)
}

/// Create an input chip with standard configuration
pub fn input_chip(label: &str) -> Chip {
    test_chip(label, ChipVariant::Input)
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
    test_chip(label, ChipVariant::Suggestion)
}

// ============================================================================
// Enhanced UI Chip Factories
// ============================================================================

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

// ============================================================================
// Error Case Factories
// ============================================================================

/// Attempt to create chip with empty label (should fail)
pub fn empty_label_chip(variant: ChipVariant) -> Result<Chip, SelectionError> {
    ChipBuilder::new("", variant).build()
}

/// Attempt to create chip with oversized label (should fail)
pub fn oversized_label_chip(variant: ChipVariant) -> Result<Chip, SelectionError> {
    ChipBuilder::new(oversized_label(), variant).build()
}

/// Attempt to create chip with max length label (should succeed)
pub fn max_length_label_chip(variant: ChipVariant) -> Result<Chip, SelectionError> {
    ChipBuilder::new(max_length_label(), variant).build()
}

// ============================================================================
// Batch Factories
// ============================================================================

/// Create a set of chips with all variants
pub fn all_variant_chips(label: &str) -> Vec<Chip> {
    ALL_CHIP_VARIANTS
        .iter()
        .map(|&variant| test_chip(label, variant))
        .collect()
}

/// Create a set of chips with all sizes
pub fn all_size_chips(label: &str, variant: ChipVariant) -> Vec<Chip> {
    ALL_COMPONENT_SIZES
        .iter()
        .map(|&size| sized_chip(label, variant, size))
        .collect()
}

/// Create a set of chips with all states
pub fn all_state_chips(label: &str, variant: ChipVariant) -> Vec<Chip> {
    ALL_CHIP_STATES
        .iter()
        .map(|&state| {
            ChipBuilder::new(label, variant)
                .with_state(state)
                .build()
                .expect("Failed to build state chip")
        })
        .collect()
}

/// Create chips for performance testing
pub fn performance_chips(variant: ChipVariant, count: usize) -> Vec<Chip> {
    (0..count)
        .map(|i| test_chip(&format!("Chip {}", i), variant))
        .collect()
}

/// Create chips with varying label lengths
pub fn variable_length_chips(variant: ChipVariant) -> Vec<Chip> {
    performance_test_labels()
        .into_iter()
        .map(|label| test_chip(&label, variant))
        .collect()
}

// ============================================================================
// Real-World Scenario Factories
// ============================================================================

/// Create filter chips for search interface testing
pub fn search_filter_chips() -> Vec<Chip> {
    FILTER_CHIP_LABELS
        .iter()
        .map(|&label| filter_chip(label))
        .collect()
}

/// Create input chips for tag interface testing
pub fn tag_input_chips() -> Vec<Chip> {
    INPUT_CHIP_LABELS
        .iter()
        .map(|&label| input_chip(label))
        .collect()
}

/// Create assist chips for help interface testing
pub fn help_assist_chips() -> Vec<Chip> {
    ASSIST_CHIP_LABELS
        .iter()
        .map(|&label| assist_chip(label))
        .collect()
}

/// Create suggestion chips for action interface testing
pub fn action_suggestion_chips() -> Vec<Chip> {
    SUGGESTION_CHIP_LABELS
        .iter()
        .map(|&label| suggestion_chip(label))
        .collect()
}

// ============================================================================
// Property-Based Testing Factories
// ============================================================================

/// Create a random valid chip for property-based testing
pub fn random_chip() -> Chip {
    test_chip(&random_valid_label(), random_chip_variant())
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

// ============================================================================
// Stress Testing Factories
// ============================================================================

/// Create chips for memory usage testing
pub fn memory_test_chips() -> Vec<Chip> {
    // Create chips with varying amounts of data to test memory efficiency
    let mut chips = Vec::new();
    
    // Simple chips
    chips.extend(performance_chips(ChipVariant::Filter, 100));
    
    // Complex chips with metadata
    for i in 0..100 {
        let chip = ChipBuilder::filter(&format!("Complex Chip {}", i))
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
            test_chip(
                &format!("Thread Safe Chip {}", i),
                ALL_CHIP_VARIANTS[i % ALL_CHIP_VARIANTS.len()]
            )
        })
        .collect()
}
