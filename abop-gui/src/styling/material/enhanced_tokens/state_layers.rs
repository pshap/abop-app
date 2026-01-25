//! State Layer Token Definitions
//!
//! This module contains state layer opacity tokens for different
//! theme variations following Material Design 3 specifications.

/// State layer tokens for opacity values
#[derive(Debug, Clone)]
pub struct StateLayerTokens {
    /// Hover state opacity
    pub hover_opacity: f32,
    /// Focus state opacity
    pub focus_opacity: f32,
    /// Pressed state opacity
    pub pressed_opacity: f32,
    /// Dragged state opacity
    pub dragged_opacity: f32,
    /// Disabled state opacity
    pub disabled_opacity: f32,
}

impl StateLayerTokens {
    /// Create state layer tokens for light theme
    pub fn light() -> Self {
        Self {
            hover_opacity: 0.08,
            focus_opacity: 0.12,
            pressed_opacity: 0.12,
            dragged_opacity: 0.16,
            disabled_opacity: 0.12,
        }
    }

    /// Create state layer tokens for dark theme
    pub fn dark() -> Self {
        Self {
            hover_opacity: 0.08,
            focus_opacity: 0.12,
            pressed_opacity: 0.12,
            dragged_opacity: 0.16,
            disabled_opacity: 0.12,
        }
    }
}
