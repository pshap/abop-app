//! FAB (Floating Action Button) variant strategy implementation

use super::super::strategy::{ButtonState, ButtonVariantConfigBuilder};
use super::super::constants;
use crate::styling::color_utils::ColorUtils;
use crate::button_strategy;
use iced::Color;

button_strategy! {
    struct FabButtonStrategy;
    name = "FAB";
    
    config = |colors, elevation| {
        let base_background = colors.primary.container;
        let icon_color = colors.primary.on_container;
        
        // FAB uses special hover/press behavior with color blending
        let hover_bg = ColorUtils::blend_colors(
            base_background,
            icon_color,
            0.08, // Default hover opacity from Material tokens
        );
        let pressed_bg = ColorUtils::blend_colors(
            base_background,
            icon_color,
            0.12, // Default pressed opacity from Material tokens
        );
        
        ButtonVariantConfigBuilder::new()
            .background(base_background)
            .text_color(icon_color)
            .border(Color::TRANSPARENT, 0.0)
            .radius(constants::radius::FAB) // Use Material Design FAB radius constant
            .shadow(elevation.level3.shadow)
            .hover_background(hover_bg)
            .pressed_background(pressed_bg)
            .build()
    }
    
    supports_elevation = true;
    base_elevation = 3.0;
    
    custom_styling = |state, config, tokens, colors| {
        let mut styling = super::super::strategy::ButtonStateHandler::apply_state_styling(
            state, config, tokens, colors
        );
        
        // Override shadows and special cases for FAB
        match state {
            ButtonState::Hovered => styling.shadow = Some(tokens.elevation.level4.shadow),
            ButtonState::Pressed => styling.shadow = Some(tokens.elevation.level3.shadow),
            ButtonState::Disabled => {
                styling.background = iced::Background::Color(ColorUtils::with_alpha(colors.on_surface, 0.12));
                styling.shadow = None;
            },
            ButtonState::Focused => {
                styling.background = iced::Background::Color(ColorUtils::blend_colors(
                    config.base_background,
                    config.icon_color,
                    tokens.states.opacity.focus,
                ));
                styling.shadow = Some(tokens.elevation.level3.shadow);
            },
            _ => {} // Keep default
        }

        return styling;
    }
}
