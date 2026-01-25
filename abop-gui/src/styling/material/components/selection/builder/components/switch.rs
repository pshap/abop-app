//! Material Design 3 Switch component
//!
//! This module contains Switch component implementation with all its
//! trait implementations and methods for state management.

use crate::styling::material::components::selection::builder::common_builder::validate_switch_state;
use crate::styling::material::components::selection::common::prelude::*;

/// Material Design 3 Switch component (modern implementation)
#[derive(Debug, Clone, Default)]
pub struct Switch {
    pub(crate) state: SwitchState,
    pub(crate) props: ComponentProps,
    pub(crate) error_state: bool,
    pub(crate) animation_config: AnimationConfig,
}

// ============================================================================
// Core Trait Implementations for Switch
// ============================================================================

impl SelectionComponent for Switch {
    type State = SwitchState;
    type Message = SwitchState;

    fn state(&self) -> Self::State {
        self.state
    }

    fn props(&self) -> &ComponentProps {
        &self.props
    }

    fn validate(&self) -> Result<(), SelectionError> {
        validate_switch_state(self.state, &self.props)
    }

    fn has_error(&self) -> bool {
        self.error_state
    }
}

impl StatefulComponent for Switch {
    fn set_state(&mut self, new_state: Self::State) -> Result<(), SelectionError> {
        self.state = new_state;
        Ok(())
    }

    fn set_error(&mut self, error: bool) {
        self.error_state = error;
    }
}

impl AnimatedComponent for Switch {
    fn animation_config(&self) -> &AnimationConfig {
        &self.animation_config
    }

    fn set_animation_config(&mut self, config: AnimationConfig) {
        self.animation_config = config;
    }
}

impl Switch {
    /// Create a new switch with the specified state
    #[must_use]
    pub fn new(state: SwitchState) -> Self {
        Self {
            state,
            props: ComponentProps::default(),
            error_state: false,
            animation_config: AnimationConfig::default(),
        }
    }

    /// Toggle the switch state
    pub fn toggle(&mut self) -> Result<(SwitchState, SwitchState), SelectionError> {
        let previous_state = self.state;
        self.state = self.state.toggle();
        Ok((previous_state, self.state))
    }

    /// Check if the switch is on
    #[must_use]
    pub const fn is_on(&self) -> bool {
        matches!(self.state, SwitchState::On)
    }

    /// Check if the switch is off
    #[must_use]
    pub const fn is_off(&self) -> bool {
        matches!(self.state, SwitchState::Off)
    }

    /// Convert switch state to boolean (on = true, off = false)
    #[must_use]
    pub const fn to_bool(&self) -> bool {
        matches!(self.state, SwitchState::On)
    }
}
