//! Elevated button variant strategy implementation

use super::super::strategy::{ButtonState, ButtonVariantConfigBuilder};
use super::super::constants;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct ElevatedButtonStrategy;
    name = "Elevated";
    
    config = |colors, elevation| {
        ButtonVariantConfigBuilder::new()
            .background(colors.surface_container_low)
            .text_color(colors.primary.on_base)
            .border(Color::TRANSPARENT, 0.0)
            .radius(constants::radius::MEDIUM) // Use Material Design medium radius constant
            .shadow(elevation.level1.shadow)
            .build()
    }
    
    supports_elevation = true;
    base_elevation = 1.0;
    
    custom_styling = |state, config, tokens, colors| {
        let mut styling = super::super::strategy::ButtonStateHandler::apply_state_styling(
            state, config, tokens, colors
        );
        
        // Override shadow for different states
        match state {
            ButtonState::Hovered => styling.shadow = Some(tokens.elevation.level2.shadow),
            ButtonState::Pressed => styling.shadow = Some(tokens.elevation.level1.shadow),
            ButtonState::Disabled => styling.shadow = None,
            _ => {} // Keep default shadow
        }

        return styling;
    }
}
