//! Icon button variant strategy implementation

use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling, ButtonVariantConfig, ButtonStateHandler};
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::Color;

/// Strategy for icon button variant (compact, icon-only)
pub struct IconButtonStrategy;

impl ButtonStyleStrategy for IconButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        _elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        let config = ButtonVariantConfig {
            base_background: Color::TRANSPARENT,
            text_color: colors.on_surface,
            icon_color: colors.on_surface,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            border_radius: 20.0, // Icon radius (typically larger for circular icons)
            shadow: None,
            uses_surface_on_interaction: true, // Uses surface colors on hover/press
            custom_hover_background: None,
            custom_pressed_background: None,
        };

        ButtonStateHandler::apply_state_styling(state, &config, tokens, colors)
    }

    fn variant_name(&self) -> &'static str {
        "Icon"
    }
}
