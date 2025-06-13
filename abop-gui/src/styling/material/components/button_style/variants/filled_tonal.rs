//! Filled tonal button variant strategy implementation

use super::super::strategy::ButtonVariantConfigBuilder;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct FilledTonalButtonStrategy;
    name = "FilledTonal";
    
    config = |colors, _elevation| {
        ButtonVariantConfigBuilder::new()
            .background(colors.secondary.container)
            .text_color(colors.secondary.on_container)
            .border(Color::TRANSPARENT, 0.0)
            .build()
    }
}
