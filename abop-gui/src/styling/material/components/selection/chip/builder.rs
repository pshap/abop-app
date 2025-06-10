//! Enhanced builder patterns for chips and chip collections
//!
//! This module provides improved builder patterns with better ergonomics,
//! method chaining, and consistent APIs across chip and collection builders.

use super::core::{MAX_CHIP_LABEL_LENGTH};
use super::collection::{ChipCollection, ChipSelectionMode};
use super::super::builder::{Chip, ChipBuilder as CoreChipBuilder, ComponentBuilder};
use super::super::common::*;

// ============================================================================
// Re-export builders from core module
// ============================================================================

/// Re-export the core chip builder
pub type ChipBuilder = CoreChipBuilder;

// ============================================================================
// Enhanced Collection Builder
// ============================================================================

/// Enhanced builder for creating chip collections with improved ergonomics
#[derive(Debug, Clone)]
pub struct ChipCollectionBuilder {
    chips: Vec<Chip>,
    selection_mode: ChipSelectionMode,
    props: ComponentProps,
    validation_config: ValidationConfig,
}

impl ChipCollectionBuilder {
    /// Create a new chip collection builder
    #[must_use]
    pub const fn new(selection_mode: ChipSelectionMode) -> Self {
        Self {
            chips: Vec::new(),
            selection_mode,
            props: ComponentProps::new(),
            validation_config: ValidationConfig {
                max_label_length: MAX_CHIP_LABEL_LENGTH,
                allow_empty_label: false,
                custom_rules: Vec::new(),
            },
        }
    }

    /// Add a pre-built chip to the collection
    #[must_use]
    pub fn chip(mut self, chip: Chip) -> Self {
        self.chips.push(chip);
        self
    }

    /// Add a chip with label and variant (internal helper)
    #[must_use]
    pub fn add<S: Into<String>>(mut self, label: S, variant: ChipVariant) -> Self {
        let chip = ChipBuilder::new(label, variant)
            .size(self.props.size)
            .disabled(self.props.disabled)
            .build_unchecked();
        self.chips.push(chip);
        self
    }

    /// Add a filter chip with the given label
    #[must_use]
    pub fn filter<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Filter)
    }

    /// Add an assist chip with the given label
    #[must_use]
    pub fn assist<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Assist)
    }

    /// Add an input chip with the given label
    #[must_use]
    pub fn input<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Input)
    }

    /// Add a suggestion chip with the given label
    #[must_use]
    pub fn suggestion<S: Into<String>>(self, label: S) -> Self {
        self.add(label, ChipVariant::Suggestion)
    }

    /// Set collection properties (applies to all chips)
    #[must_use]
    pub fn props(mut self, props: ComponentProps) -> Self {
        self.props = props;
        self
    }

    /// Set collection size (applies to all chips)
    #[must_use]
    pub const fn size(mut self, size: ComponentSize) -> Self {
        self.props.size = size;
        self
    }

    /// Set collection disabled state (applies to all chips)
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.props.disabled = disabled;
        self
    }

    /// Set validation configuration
    #[must_use]
    pub fn validation(mut self, config: ValidationConfig) -> Self {
        self.validation_config = config;
        self
    }    /// Build the chip collection with validation
    pub fn build(self) -> Result<ChipCollection, SelectionError> {
        let collection = ChipCollection::from_builder_parts(
            self.chips,
            self.selection_mode,
            self.props,
            self.validation_config,
        );

        collection.validate()?;
        Ok(collection)
    }

    /// Build the chip collection without validation
    #[must_use]
    pub fn build_unchecked(self) -> ChipCollection {
        ChipCollection::from_builder_parts(
            self.chips,
            self.selection_mode,
            self.props,
            self.validation_config,
        )
    }
}

// ============================================================================
// Convenience Functions for Quick Creation
// ============================================================================

/// Create a filter chip collection (multiple selection)
#[must_use]
pub const fn filter_chip_collection() -> ChipCollectionBuilder {
    ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
}

/// Create a single-select chip collection
#[must_use]
pub const fn single_select_chip_collection() -> ChipCollectionBuilder {
    ChipCollectionBuilder::new(ChipSelectionMode::Single)
}

/// Create an input chip collection (no selection)
#[must_use]
pub const fn input_chip_collection() -> ChipCollectionBuilder {
    ChipCollectionBuilder::new(ChipSelectionMode::None)
}

// ============================================================================
// Batch Builder Support
// ============================================================================

impl ChipCollectionBuilder {
    /// Add multiple chips at once
    #[must_use]
    pub fn chips(mut self, chips: Vec<Chip>) -> Self {
        self.chips.extend(chips);
        self
    }

    /// Add multiple filter chips from labels
    #[must_use]
    pub fn filters<S, I>(mut self, labels: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for label in labels {
            self = self.filter(label);
        }
        self
    }

    /// Add multiple assist chips from labels
    #[must_use]
    pub fn assists<S, I>(mut self, labels: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for label in labels {
            self = self.assist(label);
        }
        self
    }

    /// Add multiple input chips from labels
    #[must_use]
    pub fn inputs<S, I>(mut self, labels: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for label in labels {
            self = self.input(label);
        }
        self
    }

    /// Add multiple suggestion chips from labels
    #[must_use]
    pub fn suggestions<S, I>(mut self, labels: I) -> Self
    where
        S: Into<String>,
        I: IntoIterator<Item = S>,
    {
        for label in labels {
            self = self.suggestion(label);
        }
        self
    }
}

// ============================================================================
// Conditional Building Support
// ============================================================================

impl ChipCollectionBuilder {
    /// Add a chip conditionally
    #[must_use]
    pub fn chip_if(self, condition: bool, chip: Chip) -> Self {
        if condition {
            self.chip(chip)
        } else {
            self
        }
    }

    /// Add a filter chip conditionally
    #[must_use]
    pub fn filter_if<S: Into<String>>(self, condition: bool, label: S) -> Self {
        if condition {
            self.filter(label)
        } else {
            self
        }
    }

    /// Add an assist chip conditionally
    #[must_use]
    pub fn assist_if<S: Into<String>>(self, condition: bool, label: S) -> Self {
        if condition {
            self.assist(label)
        } else {
            self
        }
    }

    /// Add an input chip conditionally
    #[must_use]
    pub fn input_if<S: Into<String>>(self, condition: bool, label: S) -> Self {
        if condition {
            self.input(label)
        } else {
            self
        }
    }

    /// Add a suggestion chip conditionally
    #[must_use]
    pub fn suggestion_if<S: Into<String>>(self, condition: bool, label: S) -> Self {
        if condition {
            self.suggestion(label)
        } else {
            self
        }
    }
}

// ============================================================================
// Builder Utility Methods
// ============================================================================

impl ChipCollectionBuilder {
    /// Get the current number of chips in the builder
    #[must_use]
    pub fn count(&self) -> usize {
        self.chips.len()
    }

    /// Check if the builder is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chips.is_empty()
    }

    /// Get the selection mode
    #[must_use]
    pub const fn selection_mode(&self) -> ChipSelectionMode {
        self.selection_mode
    }

    /// Clear all chips from the builder
    #[must_use]
    pub fn clear(mut self) -> Self {
        self.chips.clear();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_collection_builder() {
        let collection = ChipCollectionBuilder::new(ChipSelectionMode::Multiple)
            .filter("Category")
            .assist("Help")
            .input("Tag")
            .suggestion("Action")
            .size(ComponentSize::Large)
            .build()
            .unwrap();

        assert_eq!(collection.len(), 4);
        assert_eq!(collection.selection_mode(), ChipSelectionMode::Multiple);
    }

    #[test]
    fn test_batch_builder_methods() {
        let collection = filter_chip_collection()
            .filters(vec!["Option 1", "Option 2", "Option 3"])
            .build()
            .unwrap();

        assert_eq!(collection.len(), 3);
        assert_eq!(collection.selection_mode(), ChipSelectionMode::Multiple);
    }

    #[test]
    fn test_conditional_building() {
        let collection = single_select_chip_collection()
            .filter_if(true, "Included")
            .filter_if(false, "Excluded")
            .build()
            .unwrap();

        assert_eq!(collection.len(), 1);
        assert_eq!(collection.chips()[0].label(), "Included");
    }

    #[test]
    fn test_builder_utilities() {
        let builder = ChipCollectionBuilder::new(ChipSelectionMode::Single)
            .filter("Test 1")
            .filter("Test 2");

        assert_eq!(builder.count(), 2);
        assert!(!builder.is_empty());
        assert_eq!(builder.selection_mode(), ChipSelectionMode::Single);

        let cleared_builder = builder.clear();
        assert_eq!(cleared_builder.count(), 0);
        assert!(cleared_builder.is_empty());
    }

    #[test]
    fn test_convenience_functions() {
        let filter_collection = filter_chip_collection().build().unwrap();
        assert_eq!(filter_collection.selection_mode(), ChipSelectionMode::Multiple);

        let single_collection = single_select_chip_collection().build().unwrap();
        assert_eq!(single_collection.selection_mode(), ChipSelectionMode::Single);

        let input_collection = input_chip_collection().build().unwrap();
        assert_eq!(input_collection.selection_mode(), ChipSelectionMode::None);
    }
}
