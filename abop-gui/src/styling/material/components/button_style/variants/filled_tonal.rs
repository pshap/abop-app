//! Filled tonal button variant strategy implementation

use super::super::constants::radius;
use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling};
use super::create_button_border;
use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Color};

/// Strategy for filled tonal button variant (medium emphasis, secondary actions)
pub struct FilledTonalButtonStrategy;

impl ButtonStyleStrategy for FilledTonalButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        _elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        let base_background = colors.secondary_container;
        let text_color = colors.on_secondary_container;

        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(base_background),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(text_color),
            },
            ButtonState::Hovered => ButtonStyling {
                background: Background::Color(ColorUtils::darken(base_background, 0.05)),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(text_color),
            },
            ButtonState::Pressed => ButtonStyling {
                background: Background::Color(ColorUtils::darken(base_background, 0.1)),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(text_color),
            },
            ButtonState::Disabled => ButtonStyling {
                background: Background::Color(ColorUtils::with_alpha(base_background, 0.38)),
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
                background: Background::Color(ColorUtils::darken(base_background, 0.075)),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(text_color),
            },
        }
    }

    fn variant_name(&self) -> &'static str {
        "FilledTonal"
    }
}
