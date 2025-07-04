//! Filled tonal button variant strategy implementation

use super::super::constants;
use super::super::strategy::ButtonVariantConfigBuilder;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct FilledTonalButtonStrategy;
    name = "FilledTonal";    config = |colors, _elevation, _tokens| {
        ButtonVariantConfigBuilder::new()
            .background(colors.secondary.container)
            .text_color(colors.secondary.on_container)
            .border(Color::TRANSPARENT, 0.0)
            .radius(constants::radius::MEDIUM) // Use Material Design medium radius constant
            .surface_interactions() // Enable Material Design surface interaction effects
            .build()
    }
}
