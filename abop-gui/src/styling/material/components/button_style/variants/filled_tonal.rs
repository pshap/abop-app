//! Filled tonal button variant strategy implementation

use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling, ButtonVariantConfig, ButtonStateHandler};
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::Color;

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
        let config = ButtonVariantConfig {
            base_background: colors.secondary.container,
            text_color: colors.secondary.on_container,
            icon_color: colors.secondary.on_container,
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
        "FilledTonal"
    }
}
