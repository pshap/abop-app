//! Text button variant strategy implementation

use super::super::strategy::ButtonVariantConfigBuilder;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct TextButtonStrategy;
    name = "Text";
    
    config = |colors, _elevation| {
        ButtonVariantConfigBuilder::new()
            .background(Color::TRANSPARENT)
            .text_color(colors.on_surface)
            .border(Color::TRANSPARENT, 0.0)
            .surface_interactions()
            .build()
    }
}
