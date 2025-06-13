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
        let icon_color = colors.primary.on_container;        // FAB uses special hover/press behavior with color blending using Material Design tokens
        let hover_bg = ColorUtils::blend_colors(
            base_background,
            icon_color,
            constants::opacity::HOVER, // Material Design hover opacity constant
        );
        let pressed_bg = ColorUtils::blend_colors(
            base_background,
            icon_color,
            constants::opacity::PRESSED, // Material Design pressed opacity constant
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
      custom_styling = |button_state, variant_config, material_tokens, material_colors| {
        let mut styling = super::super::strategy::ButtonStateHandler::apply_state_styling(
            button_state, variant_config, material_tokens, material_colors
        );
        
        // Override shadows and special cases for FAB
        match button_state {
            ButtonState::Hovered => styling.shadow = Some(material_tokens.elevation.level4.shadow),
            ButtonState::Pressed => styling.shadow = Some(material_tokens.elevation.level3.shadow),
            ButtonState::Disabled => {
                styling.background = iced::Background::Color(ColorUtils::with_alpha(material_colors.on_surface, constants::opacity::FAB_DISABLED_SURFACE));
                styling.shadow = None;
            },
            ButtonState::Focused => {
                styling.background = iced::Background::Color(ColorUtils::blend_colors(
                    variant_config.base_background,
                    variant_config.icon_color,
                    material_tokens.states.opacity.focus,
                ));
                styling.shadow = Some(material_tokens.elevation.level3.shadow);
            },
            _ => {} // Keep default
        }

        styling
    }
}
