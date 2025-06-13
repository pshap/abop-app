//! Outlined button variant strategy implementation

use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling, ButtonVariantConfig, ButtonStateHandler};
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::Color;

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
        let config = ButtonVariantConfig {
            base_background: Color::TRANSPARENT,
            text_color: colors.on_surface,
            icon_color: colors.on_surface,
            border_color: colors.outline,
            border_width: 1.0,
            border_radius: 12.0, // Medium radius
            shadow: None,
            uses_surface_on_interaction: true, // Uses surface colors on hover/press
            custom_hover_background: None,
            custom_pressed_background: None,
        };

        ButtonStateHandler::apply_state_styling(state, &config, tokens, colors)
    }

    fn variant_name(&self) -> &'static str {
        "Outlined"
    }

    fn has_border(&self) -> bool {
        true
    }
}
