//! Material Design 3 Checkbox component
//!
//! This module contains the Checkbox component implementation with all its
//! trait implementations and methods for state management.

use crate::styling::material::components::selection::common::prelude::*;

/// Material Design 3 Checkbox component (modern implementation)
#[derive(Debug, Clone, Default)]
pub struct Checkbox {
    pub(crate) state: CheckboxState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

// ============================================================================
// Core Trait Implementations for Checkbox
// ============================================================================

impl SelectionComponent for Checkbox {
    type State = CheckboxState;
    type Message = CheckboxState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        validate_checkbox_state(self.state, &self.props)
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl StatefulComponent for Checkbox {
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl AnimatedComponent for Checkbox {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

impl Checkbox {
    /// Create a new checkbox with the specified state
    #[must_use]
    pub fn new(state: CheckboxState) -> Self {
        Self {
            state,
            props: ComponentProps::default(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        }
    }

    /// Check the checkbox
    pub fn check(&mut self) -> Result<CheckboxState, SelectionError> {
        let previous_state = self.state;
        self.state = CheckboxState::Checked;
        Ok(previous_state)
    }

    /// Uncheck the checkbox
    pub fn uncheck(&mut self) -> Result<CheckboxState, SelectionError> {
        let previous_state = self.state;
        self.state = CheckboxState::Unchecked;
        Ok(previous_state)
    }

    /// Set checkbox to indeterminate state
    pub fn set_indeterminate(&mut self) -> Result<CheckboxState, SelectionError> {
        let previous_state = self.state;
        self.state = CheckboxState::Indeterminate;
        Ok(previous_state)
    }

    /// Toggle checkbox state
    pub fn toggle(&mut self) -> Result<(CheckboxState, CheckboxState), SelectionError> {
        let previous_state = self.state;
        self.state = self.state.toggle();
        Ok((previous_state, self.state))
    }

    /// Check if checkbox is selected (checked or indeterminate)
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        matches!(
            self.state,
            CheckboxState::Checked | CheckboxState::Indeterminate
        )
    }

    /// Check if checkbox is checked
    #[must_use]
    pub const fn is_checked(&self) -> bool {
        matches!(self.state, CheckboxState::Checked)
    }

    /// Check if checkbox is unchecked
    #[must_use]
    pub const fn is_unchecked(&self) -> bool {
        matches!(self.state, CheckboxState::Unchecked)
    }

    /// Check if checkbox is in indeterminate state
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        matches!(self.state, CheckboxState::Indeterminate)
    }

    /// Convert checkbox state to boolean (checked = true, others = false)
    #[must_use]
    pub const fn to_bool(&self) -> bool {
        matches!(self.state, CheckboxState::Checked)
    }
}

impl PartialEq for Checkbox {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
            && self.props == other.props
            && self.error_state == other.error_state
        // Note: animation_config is excluded from equality comparison
    }
}

impl Eq for Checkbox {}
