//! FAB (Floating Action Button) variant strategy implementation

use super::super::constants::{elevation, radius};
use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling};
use super::create_button_border;
use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Color};

/// Strategy for FAB variant (primary screen actions with elevation)
pub struct FabButtonStrategy;

impl ButtonStyleStrategy for FabButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        material_elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        let base_background = colors.primary.container;
        let icon_color = colors.primary.on_container;

        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(base_background),
                text_color: icon_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::FAB),
                shadow: Some(material_elevation.level3.shadow),
                icon_color: Some(icon_color),
            },
            ButtonState::Hovered => ButtonStyling {
                background: Background::Color(ColorUtils::blend_colors(
                    base_background,
                    icon_color,
                    tokens.states.opacity.hover,
                )),
                text_color: icon_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::FAB),
                shadow: Some(material_elevation.level4.shadow),
                icon_color: Some(icon_color),
            },
            ButtonState::Pressed => ButtonStyling {
                background: Background::Color(ColorUtils::blend_colors(
                    base_background,
                    icon_color,
                    tokens.states.opacity.pressed,
                )),
                text_color: icon_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::FAB),
                shadow: Some(material_elevation.level3.shadow),
                icon_color: Some(icon_color),
            },
            ButtonState::Disabled => ButtonStyling {
                background: Background::Color(ColorUtils::with_alpha(colors.on_surface, 0.12)),
                text_color: ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                ),
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::FAB),
                shadow: None,
                icon_color: Some(ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                )),
            },
            ButtonState::Focused => ButtonStyling {
                background: Background::Color(ColorUtils::blend_colors(
                    base_background,
                    icon_color,
                    tokens.states.opacity.focus,
                )),
                text_color: icon_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::FAB),
                shadow: Some(material_elevation.level3.shadow),
                icon_color: Some(icon_color),
            },
        }
    }

    fn variant_name(&self) -> &'static str {
        "FAB"
    }

    fn supports_elevation(&self) -> bool {
        true
    }

    fn base_elevation(&self) -> f32 {
        elevation::LEVEL_3
    }
}
