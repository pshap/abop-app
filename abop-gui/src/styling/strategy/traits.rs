//! Core traits for the Color Strategy System
//!
//! This module defines the fundamental traits and interfaces that make up
//! the strategy pattern implementation for component styling.

use crate::styling::material::MaterialTokens;

/// Core trait for component styling strategies
///
/// This trait defines the interface that all component styling strategies must implement.
/// It ensures consistent styling across different component variants and states.
pub trait ComponentStyleStrategy {
    /// Get the styling for a component in a specific state
    ///
    /// # Parameters
    /// - `state`: The current interaction state of the component
    /// - `tokens`: Material Design tokens for consistent theming
    ///
    /// # Returns
    /// Complete styling information for the component
    fn get_styling(
        &self,
        state: ComponentState,
        tokens: &MaterialTokens,
    ) -> crate::styling::strategy::ComponentStyling;
}

/// Represents the current state of a component
///
/// This enum captures all possible interaction states that affect
/// component appearance and behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    /// Default state - no user interaction
    Default,
    /// Hovered state - cursor is over the component
    Hovered,
    /// Pressed state - component is being pressed/clicked
    Pressed,
    /// Focused state - component has keyboard focus
    Focused,
    /// Disabled state - component is not interactive
    Disabled,
    /// Loading state - component is processing an action
    Loading,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self::Default
    }
}

impl ComponentState {
    /// Check if this state is interactive (can receive user input)
    #[must_use]
    pub const fn is_interactive(&self) -> bool {
        !matches!(self, Self::Disabled | Self::Loading)
    }

    /// Check if this state should show visual feedback
    #[must_use]
    pub const fn shows_feedback(&self) -> bool {
        matches!(self, Self::Hovered | Self::Pressed | Self::Focused)
    }

    /// Get the opacity modifier for this state
    #[must_use]
    pub const fn opacity_modifier(&self) -> f32 {
        match self {
            Self::Default => 1.0,
            Self::Hovered => 0.92,
            Self::Pressed => 0.88,
            Self::Focused => 1.0,
            Self::Disabled => 0.38,
            Self::Loading => 0.7,
        }
    }
}
