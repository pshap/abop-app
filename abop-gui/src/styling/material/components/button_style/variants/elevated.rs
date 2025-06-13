//! Elevated button variant strategy implementation

use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling, ButtonVariantConfig, ButtonStateHandler};
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use iced::Color;

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
        let config = ButtonVariantConfig {
            base_background: colors.surface_container_low,
            text_color: colors.primary.on_base,
            icon_color: colors.primary.on_base,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            border_radius: 12.0, // Medium radius
            shadow: Some(material_elevation.level1.shadow),
            uses_surface_on_interaction: false,
            custom_hover_background: None,
            custom_pressed_background: None,
        };

        let mut styling = ButtonStateHandler::apply_state_styling(state, &config, tokens, colors);
        
        // Override shadow for different states
        match state {
            ButtonState::Hovered => styling.shadow = Some(material_elevation.level2.shadow),
            ButtonState::Pressed => styling.shadow = Some(material_elevation.level1.shadow),
            ButtonState::Disabled => styling.shadow = None,
            _ => {} // Keep default shadow
        }

        styling
    }

    fn variant_name(&self) -> &'static str {
        "Elevated"
    }

    fn supports_elevation(&self) -> bool {
        true
    }

    fn base_elevation(&self) -> f32 {
        1.0
    }
}
