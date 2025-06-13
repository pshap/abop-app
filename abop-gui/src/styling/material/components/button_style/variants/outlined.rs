//! Outlined button variant strategy implementation

use super::super::strategy::ButtonVariantConfigBuilder;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct OutlinedButtonStrategy;
    name = "Outlined";
    
    config = |colors, _elevation| {
        ButtonVariantConfigBuilder::new()
            .background(Color::TRANSPARENT)
            .text_color(colors.on_surface)
            .border(colors.outline, 1.0)
            .surface_interactions()
            .build()
    }
    
    has_border = true;
}
