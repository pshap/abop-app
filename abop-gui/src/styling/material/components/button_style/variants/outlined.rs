//! Outlined button variant strategy implementation

use super::super::constants;
use super::super::strategy::ButtonVariantConfigBuilder;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct OutlinedButtonStrategy;
    name = "Outlined";

    config = |colors, _elevation, _tokens| {
        ButtonVariantConfigBuilder::new()
            .background(Color::TRANSPARENT)
            .text_color(colors.on_surface)
            .border(colors.outline, constants::border::STANDARD)
            .radius(constants::radius::MEDIUM) // Use Material Design medium radius constant
            .surface_interactions()
            .build()
    }

    has_border = true;
}
