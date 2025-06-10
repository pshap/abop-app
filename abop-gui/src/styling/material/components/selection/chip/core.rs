//! Core chip implementation with state management and validation
//!
//! This module contains the fundamental Chip type and its core functionality,
//! including state transitions, validation, and trait implementations.

use super::super::builder::{Chip, ChipBuilder};
use super::super::common::*;

use std::time::Duration;

// ============================================================================
// Constants
// ============================================================================

/// Maximum allowed length for chip labels
pub const MAX_CHIP_LABEL_LENGTH: usize = 100;

/// Default animation duration for chip state transitions
pub const DEFAULT_ANIMATION_DURATION: Duration = Duration::from_millis(150);

/// Default chip label for placeholder implementations
const DEFAULT_CHIP_LABEL: &str = "Default";

// ============================================================================
// Core Chip Implementation Extensions
// ============================================================================

impl Chip {
    /// Create a new chip with the specified label and variant
    #[must_use]
    pub fn new<S: Into<String>>(label: S, variant: ChipVariant) -> ChipBuilder {
        ChipBuilder::new(label, variant)
    }

    /// Create a filter chip
    #[must_use]
    pub fn filter<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::filter(label)
    }

    /// Create an assist chip
    #[must_use]
    pub fn assist<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::assist(label)
    }

    /// Create an input chip
    #[must_use]
    pub fn input<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::input(label)
    }

    /// Create a suggestion chip
    #[must_use]
    pub fn suggestion<S: Into<String>>(label: S) -> ChipBuilder {
        ChipBuilder::suggestion(label)
    }

    /// Get the chip label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the current chip state
    #[must_use]
    pub const fn state(&self) -> ChipState {
        self.state
    }

    /// Get the chip variant
    #[must_use]
    pub const fn variant(&self) -> ChipVariant {
        self.variant
    }

    /// Get the component properties
    #[must_use]
    pub const fn props(&self) -> &ComponentProps {
        &self.props
    }

    /// Get the animation configuration
    #[must_use]
    pub const fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    /// Update the chip state with validation
    pub fn update_state(&mut self, new_state: ChipState) -> Result<(), SelectionError> {
        validate_chip_state(new_state, self.variant, &self.props)?;
        self.state = new_state;
        Ok(())
    }

    /// Toggle the chip selection state
    pub fn toggle(&mut self) -> Result<ChipState, SelectionError> {
        let new_state = self.state.toggle();
        self.update_state(new_state)?;
        Ok(new_state)
    }

    /// Set chip as selected
    pub fn select(&mut self) -> Result<(), SelectionError> {
        self.update_state(ChipState::Selected)
    }

    /// Set chip as unselected
    pub fn unselect(&mut self) -> Result<(), SelectionError> {
        self.update_state(ChipState::Unselected)
    }

    /// Check if chip is selected
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        self.state.is_selected()
    }

    /// Check if chip is unselected
    #[must_use]
    pub const fn is_unselected(&self) -> bool {
        matches!(self.state, ChipState::Unselected)
    }

    /// Check if chip is being pressed
    #[must_use]
    pub const fn is_pressed(&self) -> bool {
        matches!(self.state, ChipState::Pressed)
    }
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl SelectionWidget<ChipState> for Chip {
    type Message = ChipState;
    type Builder = ChipBuilder;

    fn new(state: ChipState) -> Self::Builder {
        // Note: Chips require label and variant, so this is a placeholder
        ChipBuilder::filter(DEFAULT_CHIP_LABEL).with_state(state)
    }

    fn validate(&self) -> Result<(), SelectionError> {
        validate_chip_state(self.state, self.variant, &self.props)
    }

    fn state(&self) -> ChipState {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }
}

impl StatefulWidget<ChipState> for Chip {
    fn update_state(&mut self, new_state: ChipState) -> Result<(), SelectionError> {
        self.update_state(new_state)
    }

    fn transition_to(&mut self, new_state: ChipState) -> Result<ChipState, SelectionError> {
        self.update_state(new_state)?;
        Ok(self.state)
    }
}

impl AnimatedWidget for Chip {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::components::selection::ComponentBuilder;

    #[test]
    fn test_chip_creation() {
        let chip = Chip::filter("Test Chip")
            .selected(true)
            .size(ComponentSize::Large)
            .build()
            .expect("Should create valid chip");

        assert_eq!(chip.label(), "Test Chip");
        assert_eq!(chip.state(), ChipState::Selected);
        assert_eq!(chip.variant(), ChipVariant::Filter);
        assert_eq!(chip.props().size, ComponentSize::Large);
        assert!(chip.is_selected());
    }

    #[test]
    fn test_chip_state_transitions() {
        let mut chip = Chip::filter("Test")
            .build()
            .expect("Should create valid chip");

        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(chip.is_unselected());

        // Select the chip
        chip.select().expect("Should select successfully");
        assert_eq!(chip.state(), ChipState::Selected);
        assert!(chip.is_selected());

        // Toggle to unselected
        let new_state = chip.toggle().expect("Should toggle successfully");
        assert_eq!(new_state, ChipState::Unselected);
        assert!(chip.is_unselected());
    }

    #[test]
    fn test_chip_variants() {
        let assist = Chip::assist("Help").build().unwrap();
        let filter = Chip::filter("Category").build().unwrap();
        let input = Chip::input("Tag").build().unwrap();
        let suggestion = Chip::suggestion("Action").build().unwrap();

        assert_eq!(assist.variant(), ChipVariant::Assist);
        assert_eq!(filter.variant(), ChipVariant::Filter);
        assert_eq!(input.variant(), ChipVariant::Input);
        assert_eq!(suggestion.variant(), ChipVariant::Suggestion);
    }

    #[test]
    fn test_chip_validation() {
        // Valid chip
        let valid_chip = Chip::filter("Valid").build();
        assert!(valid_chip.is_ok());

        // Invalid chip - empty label
        let invalid_chip = Chip::filter("").build();
        assert!(invalid_chip.is_err());

        // Invalid chip - label too long
        let long_label = "x".repeat(MAX_CHIP_LABEL_LENGTH + 1);
        let invalid_chip = Chip::filter(long_label).build();
        assert!(invalid_chip.is_err());
    }

    #[test]
    fn test_chip_traits() {
        let chip = Chip::filter("Test")
            .build()
            .expect("Should create valid chip");

        // Test SelectionWidget trait
        assert_eq!(chip.state(), ChipState::Unselected);
        assert!(chip.validate().is_ok());

        // Test animation support
        assert!(chip.animation_config().enabled);
    }
}
