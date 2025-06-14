//! Component state definitions
//!
//! This module provides specific state types for different component categories,
//! building on the generic ComponentState trait.

use super::traits::ComponentState;

/// Button-specific interaction states
///
/// This enum extends the generic ComponentState with button-specific
/// state information while maintaining compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    /// Default button state
    Default,
    /// Mouse is hovering over the button
    Hovered,
    /// Button is being pressed
    Pressed,
    /// Button has keyboard focus
    Focused,
    /// Button is disabled and not interactive
    Disabled,
    /// Button is processing an action
    Loading,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Default
    }
}

impl From<ButtonState> for ComponentState {
    fn from(state: ButtonState) -> Self {
        match state {
            ButtonState::Default => ComponentState::Default,
            ButtonState::Hovered => ComponentState::Hovered,
            ButtonState::Pressed => ComponentState::Pressed,
            ButtonState::Focused => ComponentState::Focused,
            ButtonState::Disabled => ComponentState::Disabled,
            ButtonState::Loading => ComponentState::Loading,
        }
    }
}

impl ButtonState {
    /// Check if the button is interactive
    #[must_use]
    pub const fn is_interactive(&self) -> bool {
        !matches!(self, Self::Disabled | Self::Loading)
    }
    
    /// Get the state overlay opacity for Material Design state layers
    #[must_use]
    pub const fn state_layer_opacity(&self) -> f32 {
        match self {
            Self::Default => 0.0,
            Self::Hovered => 0.08,
            Self::Pressed => 0.12,
            Self::Focused => 0.12,
            Self::Disabled => 0.0,
            Self::Loading => 0.0,
        }
    }
}

/// Generic component interaction state
///
/// This can be used for components that don't need specialized state handling.
pub type ComponentInteractionState = ComponentState;
