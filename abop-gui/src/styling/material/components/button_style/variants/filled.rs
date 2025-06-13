//! Filled button variant strategy implementation

use super::super::strategy::ButtonVariantConfigBuilder;
use super::super::constants;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct FilledButtonStrategy;
    name = "Filled";    config = |colors, _elevation| {
        ButtonVariantConfigBuilder::new()
            .background(colors.primary.base)
            .text_color(colors.primary.on_base)
            .border(Color::TRANSPARENT, 0.0)
            .radius(constants::radius::MEDIUM) // Use Material Design medium radius constant
            .build()
    }
}
