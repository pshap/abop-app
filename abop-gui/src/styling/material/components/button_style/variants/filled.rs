//! Filled button variant strategy implementation

use super::super::constants::radius;
use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling};
use super::create_button_border;
use crate::styling::color_utils::ColorUtils;
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::{Background, Color};

/// Strategy for filled button variant (high emphasis, primary actions)
pub struct FilledButtonStrategy;

impl ButtonStyleStrategy for FilledButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        _elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        let base_background = colors.primary.base;
        let text_color = colors.on_primary;
        // Use proper Material Design 3 primary.on_base color for icons
        // This ensures WCAG AA compliance with the corrected dark theme colors
        let icon_color = colors.primary.on_base;

        match state {
            ButtonState::Default => ButtonStyling {
                background: Background::Color(base_background),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(icon_color),
            },
            ButtonState::Hovered => ButtonStyling {
                // For dark themes, we apply a darkening effect rather than lightening
                // to maintain proper contrast with light text
                background: Background::Color(
                    ColorUtils::darken(base_background, 0.05), // Darken the base color slightly for hover state
                ),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(icon_color),
            },
            ButtonState::Pressed => ButtonStyling {
                // For dark themes, we apply a darkening effect rather than lightening
                // to maintain proper contrast with light text
                background: Background::Color(
                    ColorUtils::darken(base_background, 0.1), // Darken the base color more for pressed state
                ),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(icon_color),
            },
            ButtonState::Disabled => ButtonStyling {
                // Use the same background as default but with reduced opacity
                background: Background::Color(ColorUtils::with_alpha(base_background, 0.38)),
                // Use centralized disabled color approach
                text_color: ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                ),
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                // Match icon color with text color for consistency
                icon_color: Some(ColorUtils::with_alpha(
                    colors.on_surface,
                    tokens.states.opacity.disabled,
                )),
            },
            ButtonState::Focused => ButtonStyling {
                // For dark themes, we apply a darkening effect rather than lightening
                // to maintain proper contrast with light text
                background: Background::Color(
                    ColorUtils::darken(base_background, 0.075), // Darken the base color for focus state
                ),
                text_color,
                border: create_button_border(Color::TRANSPARENT, 0.0, radius::MEDIUM),
                shadow: None,
                icon_color: Some(icon_color),
            },
        }
    }

    fn variant_name(&self) -> &'static str {
        "Filled"
    }
}
