//! Text Field Token Definitions
//!
//! This module contains text field color and shape token definitions
//! following Material Design 3 specifications.

use super::*;

/// Color set for text field variants
#[derive(Debug, Clone)]
pub struct TextFieldColors {
    /// Background color for the text field container
    pub container: iced::Color,
    /// Text color that appears within the container
    pub on_container: iced::Color,
    /// Default outline color for text field borders
    pub outline: iced::Color,
    /// Outline color when text field is focused
    pub outline_focused: iced::Color,
    /// Error state color for validation failures
    pub error: iced::Color,
    /// Overlay color applied on hover interactions
    pub hover_overlay: iced::Color,
}

/// Shape tokens for text fields
#[derive(Debug, Clone)]
pub struct TextFieldShapes {
    /// Corner radius for text field borders in logical pixels
    pub corner_radius: f32,
}

impl TextFieldTokens {
    /// Create new text field tokens
    pub fn new(
        colors: &MaterialColors,
        state_layers: &StateLayerTokens,
        shapes: &ShapeTokens,
    ) -> Self {
        Self {
            colors: TextFieldColors {
                container: colors.surface_variant,
                on_container: colors.on_surface_variant,
                outline: colors.outline,
                outline_focused: colors.primary.base,
                error: colors.error.base,
                hover_overlay: iced::Color {
                    a: state_layers.hover_opacity,
                    ..colors.on_surface
                },
            },
            shapes: TextFieldShapes {
                corner_radius: shapes.corner_sm,
            },
        }
    }
}
