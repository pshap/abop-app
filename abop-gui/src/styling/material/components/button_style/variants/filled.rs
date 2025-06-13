//! Filled button variant strategy implementation

use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling, ButtonVariantConfig, ButtonStateHandler};
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::Color;

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
        let config = ButtonVariantConfig {
            base_background: colors.primary.base,
            text_color: colors.primary.on_base,
            icon_color: colors.primary.on_base,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            border_radius: 12.0, // Medium radius
            shadow: None,
            uses_surface_on_interaction: false,
            custom_hover_background: None,
            custom_pressed_background: None,
        };

        ButtonStateHandler::apply_state_styling(state, &config, tokens, colors)
    }

    fn variant_name(&self) -> &'static str {
        "Filled"
    }
}
