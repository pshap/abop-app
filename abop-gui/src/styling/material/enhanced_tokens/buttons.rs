//! Button Token Definitions
//!
//! This module contains button color and shape token definitions
//! following Material Design 3 specifications.

use super::*;

/// Color set for button variants
#[derive(Debug, Clone)]
pub struct ButtonColorSet {
    /// Container background color for the button
    pub container: iced::Color,
    /// Color for content on container background
    pub on_container: iced::Color,
    /// Hover overlay color
    pub hover_overlay: iced::Color,
    /// Pressed overlay color
    pub pressed_overlay: iced::Color,
}

/// Button shapes configuration
#[derive(Debug, Clone)]
pub struct ButtonShapes {
    /// Corner radius for buttons
    pub corner_radius: f32,
}

impl ButtonTokens {
    /// Create new button tokens
    pub fn new(
        colors: &MaterialColors,
        state_layers: &StateLayerTokens,
        shapes: &ShapeTokens,
    ) -> Self {
        Self {
            primary: ButtonColorSet {
                container: colors.primary.base,
                on_container: colors.primary.on_base,
                hover_overlay: iced::Color {
                    a: state_layers.hover_opacity,
                    ..colors.primary.on_base
                },
                pressed_overlay: iced::Color {
                    a: state_layers.pressed_opacity,
                    ..colors.primary.on_base
                },
            },
            secondary: ButtonColorSet {
                container: colors.secondary.base,
                on_container: colors.secondary.on_base,
                hover_overlay: iced::Color {
                    a: state_layers.hover_opacity,
                    ..colors.secondary.on_base
                },
                pressed_overlay: iced::Color {
                    a: state_layers.pressed_opacity,
                    ..colors.secondary.on_base
                },
            },
            tertiary: ButtonColorSet {
                container: colors.tertiary.base,
                on_container: colors.tertiary.on_base,
                hover_overlay: iced::Color {
                    a: state_layers.hover_opacity,
                    ..colors.tertiary.on_base
                },
                pressed_overlay: iced::Color {
                    a: state_layers.pressed_opacity,
                    ..colors.tertiary.on_base
                },
            },
            shapes: ButtonShapes {
                corner_radius: shapes.corner_lg,
            },
        }
    }
}
