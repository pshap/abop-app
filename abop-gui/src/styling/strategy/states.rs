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

/// Form control interaction states for precise styling control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionState {
    /// Default state - no interaction
    Default,
    /// Mouse is hovering over the component
    Hovered,
    /// Component is being pressed/clicked
    Pressed,
    /// Component has keyboard focus
    Focused,
}

impl Default for InteractionState {
    fn default() -> Self {
        Self::Default
    }
}

/// Checkbox-specific state management
///
/// Provides comprehensive state tracking for Material Design 3 checkboxes
/// including selection, interaction, and error states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CheckboxState {
    /// Whether the checkbox is currently selected/checked
    selected: bool,
    /// Whether the checkbox is in an indeterminate state
    indeterminate: bool,
    /// Current interaction state (hover, press, focus, etc.)
    interaction: InteractionState,
    /// Whether the checkbox is disabled
    disabled: bool,
    /// Whether the checkbox is in an error state
    error: bool,
}

impl Default for CheckboxState {
    fn default() -> Self {
        Self {
            selected: false,
            indeterminate: false,
            interaction: InteractionState::Default,
            disabled: false,
            error: false,
        }
    }
}

impl CheckboxState {
    /// Create a new checkbox state
    #[must_use]
    pub const fn new() -> Self {
        Self {
            selected: false,
            indeterminate: false,
            interaction: InteractionState::Default,
            disabled: false,
            error: false,
        }
    }

    /// Set the selected state
    #[must_use]
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set the indeterminate state
    #[must_use]
    pub const fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Set the interaction state
    #[must_use]
    pub const fn interaction(mut self, interaction: InteractionState) -> Self {
        self.interaction = interaction;
        self
    }

    /// Set the disabled state
    #[must_use]
    pub const fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the error state
    #[must_use]
    pub const fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Check if the checkbox is selected
    #[must_use]
    pub const fn is_selected(&self) -> bool {
        self.selected
    }

    /// Check if the checkbox is indeterminate
    #[must_use]
    pub const fn is_indeterminate(&self) -> bool {
        self.indeterminate
    }    /// Get the current interaction state
    #[must_use]
    pub const fn get_interaction(&self) -> InteractionState {
        self.interaction
    }

    /// Check if the checkbox is disabled
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Check if the checkbox is in error state
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.error
    }

    /// Check if the checkbox is interactive (not disabled)
    #[must_use]
    pub const fn is_interactive(&self) -> bool {
        !self.disabled
    }

    /// Check if the checkbox is focused
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        matches!(self.interaction, InteractionState::Focused)
    }

    /// Check if the checkbox is hovered
    #[must_use]
    pub const fn is_hovered(&self) -> bool {
        matches!(self.interaction, InteractionState::Hovered)
    }

    /// Check if the checkbox is pressed
    #[must_use]
    pub const fn is_pressed(&self) -> bool {
        matches!(self.interaction, InteractionState::Pressed)
    }    /// Get the state layer opacity for Material Design state layers
    #[must_use]
    pub const fn state_layer_opacity(&self) -> f32 {
        if self.disabled {
            return 0.0;
        }

        match self.interaction {
            InteractionState::Default => 0.0,
            InteractionState::Hovered => 0.08,
            InteractionState::Pressed => 0.12,
            InteractionState::Focused => 0.12,
        }
    }
}
