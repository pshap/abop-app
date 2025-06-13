//! Icon button variant strategy implementation

use super::super::strategy::ButtonVariantConfigBuilder;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct IconButtonStrategy;
    name = "Icon";
    
    config = |colors, _elevation| {
        ButtonVariantConfigBuilder::new()
            .background(Color::TRANSPARENT)
            .text_color(colors.on_surface)
            .border(Color::TRANSPARENT, 0.0)
            .radius(20.0) // Icon radius (typically larger for circular icons)
            .surface_interactions()
            .build()
    }
}
