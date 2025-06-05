//! Elevated button variant strategy implementation

use super::super::constants::{elevation, radius};
use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling};
use super::create_button_border;
use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Color};

/// Strategy for elevated button variant (high emphasis with shadow)
pub struct ElevatedButtonStrategy;

impl ButtonStyleStrategy for ElevatedButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        material_elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        let base_background = colors.surface_container_low;
        let text_color = colors.primary.on_base;

        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(base_background),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: Some(material_elevation.level1.shadow),
                icon_color: Some(text_color),
            },
            ButtonState::Hovered => ButtonStyling {
                background: Background::Color(ColorUtils::darken(base_background, 0.1)),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: Some(material_elevation.level2.shadow),
                icon_color: Some(text_color),
            },
            ButtonState::Pressed => ButtonStyling {
                background: Background::Color(ColorUtils::darken(base_background, 0.2)),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: Some(material_elevation.level1.shadow),
                icon_color: Some(text_color),
            },
            ButtonState::Disabled => ButtonStyling {
                background: Background::Color(ColorUtils::with_alpha(
                    base_background,
                    tokens.states.opacity.disabled,
                )),
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
                background: Background::Color(ColorUtils::darken(base_background, 0.05)),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: Some(material_elevation.level1.shadow),
                icon_color: Some(text_color),
            },
        }
    }

    fn variant_name(&self) -> &'static str {
        "Elevated"
    }

    fn supports_elevation(&self) -> bool {
        true
    }

    fn base_elevation(&self) -> f32 {
        elevation::LEVEL_1
    }
}
