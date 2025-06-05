//! Text button variant strategy implementation

use super::super::constants::radius;
use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling};
use super::create_button_border;
use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Color};

/// Strategy for text button variant (low emphasis, text-only)
pub struct TextButtonStrategy;

impl ButtonStyleStrategy for TextButtonStrategy {
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

        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(Color::TRANSPARENT),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(text_color),
            },
            ButtonState::Hovered => ButtonStyling {
                background: Background::Color(colors.surface_variant),
                text_color: colors.on_surface_variant,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(colors.on_surface_variant),
            },
            ButtonState::Pressed => ButtonStyling {
                background: Background::Color(ColorUtils::darken(colors.surface_variant, 0.1)),
                text_color: colors.on_surface_variant,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(colors.on_surface_variant),
            },
            ButtonState::Disabled => ButtonStyling {
                background: Background::Color(Color::TRANSPARENT),
                text_color: ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                ),
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                )),
            },
            ButtonState::Focused => ButtonStyling {
                background: Background::Color(ColorUtils::darken(colors.surface_variant, 0.05)),
                text_color: colors.on_surface_variant,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(colors.on_surface_variant),
            },
        }
    }

    fn variant_name(&self) -> &'static str {
        "Text"
    }
}
