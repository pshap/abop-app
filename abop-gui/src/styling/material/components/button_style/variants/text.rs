//! Text button variant strategy implementation

use super::super::strategy::ButtonVariantConfigBuilder;
use super::super::constants;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct TextButtonStrategy;
    name = "Text";
    
    config = |colors, _elevation, _tokens| {
        ButtonVariantConfigBuilder::new()
            .background(Color::TRANSPARENT)
            .text_color(colors.on_surface)
            .border(Color::TRANSPARENT, 0.0)
            .radius(constants::radius::MEDIUM) // Use Material Design medium radius constant
            .surface_interactions()
            .build()
    }
}
