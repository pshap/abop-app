//! Component states for shape variations in the Material Design 3 Shape System
//!
//! This module defines different interactive states that may require
//! slight shape modifications for visual feedback.

/// Component states for shape variations
///
/// Defines different interactive states that may require
/// slight shape modifications for visual feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    /// Normal, inactive state
    Default,
    /// Mouse hover or touch hover state
    Hovered,
    /// Pressed or active state
    Pressed,
    /// Keyboard focus state
    Focused,
    /// Disabled, non-interactive state
    Disabled,
}

impl ComponentState {
    /// Get all component states
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Default,
            Self::Hovered,
            Self::Pressed,
            Self::Focused,
            Self::Disabled,
        ]
    }

    /// Get the state name as a string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Hovered => "hovered",
            Self::Pressed => "pressed",
            Self::Focused => "focused",
            Self::Disabled => "disabled",
        }
    }

    /// Get description of the component state
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Default => "Normal, inactive state",
            Self::Hovered => "Mouse hover or touch hover state",
            Self::Pressed => "Pressed or active state",
            Self::Focused => "Keyboard focus state",
            Self::Disabled => "Disabled, non-interactive state",
        }
    }

    /// Get the scale factor for this state
    #[must_use]
    pub const fn scale_factor(&self) -> f32 {
        match self {
            Self::Default => 1.0,
            Self::Hovered => super::constants::HOVER_SCALE,
            Self::Pressed => super::constants::PRESSED_SCALE,
            Self::Focused => 1.0,
            Self::Disabled => super::constants::DISABLED_SCALE,
        }
    }

    /// Check if this state requires visual feedback
    #[must_use]
    pub const fn requires_feedback(&self) -> bool {
        match self {
            Self::Default | Self::Focused => false,
            Self::Hovered | Self::Pressed | Self::Disabled => true,
        }
    }
}
