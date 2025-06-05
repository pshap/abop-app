//! Outlined button variant strategy implementation

use super::super::constants::{border, radius};
use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling};
use super::create_button_border;
use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Color};

/// Strategy for outlined button variant (medium emphasis with border)
pub struct OutlinedButtonStrategy;

impl ButtonStyleStrategy for OutlinedButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        _elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        // Use on_primary color for better contrast on dark themes
        let text_color = colors.primary.on_base;
        let border_color = colors.outline;

        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(Color::TRANSPARENT),
                text_color,
                border: create_button_border(border_color, border::STANDARD, radius::MEDIUM),
                shadow: None,
                icon_color: Some(text_color),
            },
            ButtonState::Hovered => ButtonStyling {
                background: Background::Color(colors.surface_variant),
                text_color: colors.on_surface_variant,
                border: create_button_border(border_color, border::STANDARD, radius::MEDIUM),
                shadow: None,
                icon_color: Some(colors.on_surface_variant),
            },
            ButtonState::Pressed => ButtonStyling {
                background: Background::Color(ColorUtils::darken(colors.surface_variant, 0.1)),
                text_color: colors.on_surface_variant,
                border: create_button_border(border_color, border::STANDARD, radius::MEDIUM),
                shadow: None,
                icon_color: Some(colors.on_surface_variant),
            },
            ButtonState::Disabled => ButtonStyling {
                background: Background::Color(Color::TRANSPARENT),
                text_color: ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                ),
                border: create_button_border(
                    ColorUtils::with_alpha(colors.on_surface, tokens.states.opacity.disabled),
                    border::STANDARD,
                    radius::MEDIUM,
                ),
                shadow: None,
                icon_color: Some(ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                )),
            },
            ButtonState::Focused => ButtonStyling {
                background: Background::Color(ColorUtils::darken(colors.surface_variant, 0.05)),
                text_color: colors.on_surface_variant,
                border: create_button_border(
                    colors.primary.base,
                    border::FOCUS_RING,
                    radius::MEDIUM,
                ),
                shadow: None,
                icon_color: Some(colors.on_surface_variant),
            },
        }
    }

    fn variant_name(&self) -> &'static str {
        "Outlined"
    }

    fn has_border(&self) -> bool {
        true
    }
}
