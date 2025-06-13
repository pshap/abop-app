//! FAB (Floating Action Button) variant strategy implementation

use super::super::strategy::{ButtonState, ButtonStyleStrategy, ButtonStyling, ButtonVariantConfig, ButtonStateHandler};
use crate::styling::material::{MaterialColors, MaterialElevation, MaterialShapes, MaterialTokens};
use crate::styling::color_utils::ColorUtils;
use iced::Color;

/// Strategy for FAB variant (primary screen actions with elevation)
pub struct FabButtonStrategy;

impl ButtonStyleStrategy for FabButtonStrategy {
    fn get_styling(
        &self,
        state: ButtonState,
        tokens: &MaterialTokens,
        colors: &MaterialColors,
        material_elevation: &MaterialElevation,
        _shapes: &MaterialShapes,
    ) -> ButtonStyling {
        let base_background = colors.primary.container;
        let icon_color = colors.primary.on_container;
        
        // FAB uses special hover/press behavior with color blending
        let hover_bg = ColorUtils::blend_colors(
            base_background,
            icon_color,
            tokens.states.opacity.hover,
        );
        let pressed_bg = ColorUtils::blend_colors(
            base_background,
            icon_color,
            tokens.states.opacity.pressed,
        );
        
        let config = ButtonVariantConfig {
            base_background,
            text_color: icon_color,
            icon_color,
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
            border_radius: 28.0, // FAB radius (large, circular)
            shadow: Some(material_elevation.level3.shadow),
            uses_surface_on_interaction: false,
            custom_hover_background: Some(hover_bg),
            custom_pressed_background: Some(pressed_bg),
        };

        let mut styling = ButtonStateHandler::apply_state_styling(state, &config, tokens, colors);
        
        // Override shadows and special cases for FAB
        match state {
            ButtonState::Hovered => styling.shadow = Some(material_elevation.level4.shadow),
            ButtonState::Pressed => styling.shadow = Some(material_elevation.level3.shadow),
            ButtonState::Disabled => {
                styling.background = iced::Background::Color(ColorUtils::with_alpha(colors.on_surface, 0.12));
                styling.shadow = None;
            },
            ButtonState::Focused => {
                styling.background = iced::Background::Color(ColorUtils::blend_colors(
                    base_background,
                    icon_color,
                    tokens.states.opacity.focus,
                ));
                styling.shadow = Some(material_elevation.level3.shadow);
            },
            _ => {} // Keep default
        }

        styling
    }

    fn variant_name(&self) -> &'static str {
        "FAB"
    }

    fn supports_elevation(&self) -> bool {
        true
    }

    fn base_elevation(&self) -> f32 {
        3.0
    }
}
