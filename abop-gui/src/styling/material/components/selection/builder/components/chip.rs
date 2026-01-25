//! Material Design 3 Chip component
//!
//! This module contains the Chip component implementation with all its
//! trait implementations and methods for state management.

use crate::styling::material::components::selection::common::prelude::*;

/// Material Design 3 Chip component (modern implementation)
#[derive(Debug, Clone, Default)]
pub struct Chip {
    pub(crate) label: String,
    pub(crate) state: ChipState,
    pub(crate) variant: ChipVariant,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

impl Chip {
    /// Get the chip label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Get the chip variant
    #[must_use]
    pub const fn variant(&self) -> ChipVariant {
        self.variant
    }

    /// Select the chip
    pub fn select(&mut self) -> Result<ChipState, SelectionError> {
        let previous_state = self.state;
        self.state = ChipState::Selected;
        Ok(previous_state)
    }

    /// Unselect the chip
    pub fn unselect(&mut self) -> Result<ChipState, SelectionError> {
        let previous_state = self.state;
        self.state = ChipState::Unselected;
        Ok(previous_state)
    }

    /// Toggle the chip selection state
    pub fn toggle(&mut self) -> Result<(ChipState, ChipState), SelectionError> {
        let previous_state = self.state;
        self.state = self.state.toggle();
        Ok((previous_state, self.state))
    }

    /// Check if the chip is selected
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        matches!(self.state, ChipState::Selected)
    }

    /// Check if the chip is unselected
    #[must_use]
    pub const fn is_unselected(&self) -> bool {
        matches!(self.state, ChipState::Unselected)
    }

    /// Set the chip label
    pub fn set_label<S: Into<String>>(&mut self, label: S) {
        self.label = label.into();
    }
}

impl PartialEq for Chip {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
            && self.variant == other.variant
            && self.state == other.state
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Chip {}

// ============================================================================
// Core Trait Implementations for Chip
// ============================================================================

impl SelectionComponent for Chip {
    type State = ChipState;
    type Message = ChipState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        // Chip validation would go here
        Ok(())
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl StatefulComponent for Chip {
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl AnimatedComponent for Chip {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_creation() {
        let chip = Chip::default();
        assert_eq!(chip.label(), "");
        assert!(chip.is_unselected());
    }

    #[test]
    fn test_chip_selection() {
        let mut chip = Chip::default();
        chip.select().unwrap();
        assert!(chip.is_selected());

        chip.unselect().unwrap();
        assert!(chip.is_unselected());
    }

    #[test]
    fn test_chip_toggle() {
        let mut chip = Chip::default();
        let (prev, new) = chip.toggle().unwrap();
        assert!(matches!(prev, ChipState::Unselected));
        assert!(matches!(new, ChipState::Selected));
    }
}
