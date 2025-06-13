//! Elevated button variant strategy implementation

use super::super::constants;
use super::super::strategy::{ButtonState, ButtonVariantConfigBuilder};
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct ElevatedButtonStrategy;
    name = "Elevated";

    config = |colors, elevation, _tokens| {
        ButtonVariantConfigBuilder::new()
            .background(colors.surface_container_low)
            .text_color(colors.primary.on_base)
            .border(Color::TRANSPARENT, 0.0)
            .radius(constants::radius::MEDIUM) // Use Material Design medium radius constant
            .shadow(elevation.level1.shadow)
            .surface_interactions() // Enable Material Design surface interaction effects
            .build()
    }

    supports_elevation = true;
    base_elevation = 1.0;

    custom_styling = |button_state, variant_config, material_tokens, material_colors| {
        let mut styling = super::super::strategy::ButtonStateHandler::apply_state_styling(
            button_state, variant_config, material_tokens, material_colors
        );

        // Override shadow for different states
        match button_state {
            ButtonState::Hovered => styling.shadow = Some(material_tokens.elevation.level2.shadow),
            ButtonState::Pressed => styling.shadow = Some(material_tokens.elevation.level1.shadow),
            ButtonState::Disabled => styling.shadow = None,
            _ => {} // Keep default shadow
        }

        styling
    }
}
