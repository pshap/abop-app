//! Button styling strategies
//!
//! This module implements the strategy pattern for Material Design 3 button variants,
//! providing consistent styling while supporting all theme modes and component states.

use super::{
    styling::ComponentStyling,
    traits::{ComponentState, ComponentStyleStrategy},
};
use crate::styling::material::MaterialTokens;
use iced::{Background, Border, Color};

/// Button styling variant strategies
///
/// Each variant implements a different Material Design 3 button style
/// with appropriate colors, elevation, and visual treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonStyleVariant {
    /// Filled button with solid background (highest emphasis)
    Filled,
    /// Filled tonal button with container background (high emphasis)
    FilledTonal,
    /// Outlined button with border and transparent background (medium emphasis)
    Outlined,
    /// Text button with no background or border (low emphasis)
    Text,
    /// Elevated button with shadow and background (highest emphasis)
    Elevated,
}

impl ButtonStyleVariant {
    /// Get the strategy implementation for this variant
    #[must_use]
    pub fn get_strategy(self) -> Box<dyn ComponentStyleStrategy> {
        match self {
            Self::Filled => Box::new(FilledButtonStrategy),
            Self::FilledTonal => Box::new(FilledTonalButtonStrategy),
            Self::Outlined => Box::new(OutlinedButtonStrategy),
            Self::Text => Box::new(TextButtonStrategy),
            Self::Elevated => Box::new(ElevatedButtonStrategy),
        }
    }
}

/// Button-specific styling result
///
/// This extends ComponentStyling with button-specific properties.
#[derive(Debug, Clone)]
pub struct ButtonStyling {
    /// Base component styling
    pub base: ComponentStyling,
    // Additional button-specific properties can be added here
}

impl ButtonStyling {
    /// Create from base component styling
    #[must_use]
    pub fn from_base(base: ComponentStyling) -> Self {
        Self { base }
    }
}

impl std::ops::Deref for ButtonStyling {
    type Target = ComponentStyling;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// =============================================================================
// Strategy Implementations
// =============================================================================

/// Strategy for filled buttons (solid background, highest emphasis)
struct FilledButtonStrategy;

impl ComponentStyleStrategy for FilledButtonStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        let colors = &tokens.colors;
        let states = &tokens.states;
        let (background_color, text_color) = match state {
            ComponentState::Default => (colors.primary.base, colors.on_primary()),
            ComponentState::Hovered => {
                let overlay = apply_state_overlay(
                    colors.primary.base,
                    colors.on_primary(),
                    states.opacity.hover,
                );
                (overlay, colors.on_primary())
            }
            ComponentState::Pressed => {
                let overlay = apply_state_overlay(
                    colors.primary.base,
                    colors.on_primary(),
                    states.opacity.pressed,
                );
                (overlay, colors.on_primary())
            }
            ComponentState::Focused => (colors.primary.base, colors.on_primary()),
            ComponentState::Disabled => (colors.on_surface, colors.surface),
            ComponentState::Loading => (colors.primary.container, colors.on_primary_container()),
        };

        ComponentStyling {
            background: Background::Color(background_color),
            text_color,
            icon_color: Some(text_color), // Icons use same color as text for filled buttons
            border: Border::default(),
            shadow: if matches!(state, ComponentState::Default | ComponentState::Focused) {
                Some(create_elevation_shadow(1))
            } else {
                None
            },
            opacity: if matches!(state, ComponentState::Disabled) {
                0.38
            } else {
                1.0
            },
        }
    }
}

/// Strategy for filled tonal buttons (container background, high emphasis)
struct FilledTonalButtonStrategy;

impl ComponentStyleStrategy for FilledTonalButtonStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        let colors = &tokens.colors;
        let states = &tokens.states;
        let (background_color, text_color) = match state {
            ComponentState::Default => {
                (colors.secondary.container, colors.on_secondary_container())
            }
            ComponentState::Hovered => {
                let overlay = apply_state_overlay(
                    colors.secondary.container,
                    colors.on_secondary_container(),
                    states.opacity.hover,
                );
                (overlay, colors.on_secondary_container())
            }
            ComponentState::Pressed => {
                let overlay = apply_state_overlay(
                    colors.secondary.container,
                    colors.on_secondary_container(),
                    states.opacity.pressed,
                );
                (overlay, colors.on_secondary_container())
            }
            ComponentState::Focused => {
                (colors.secondary.container, colors.on_secondary_container())
            }
            ComponentState::Disabled => (colors.on_surface, colors.surface),
            ComponentState::Loading => (colors.secondary.base, colors.secondary.on_base),
        };

        ComponentStyling {
            background: Background::Color(background_color),
            text_color,
            icon_color: Some(text_color),
            border: Border::default(),
            shadow: None, // Filled tonal buttons don't have elevation
            opacity: if matches!(state, ComponentState::Disabled) {
                0.38
            } else {
                1.0
            },
        }
    }
}

/// Strategy for outlined buttons (border with transparent background, medium emphasis)
struct OutlinedButtonStrategy;

impl ComponentStyleStrategy for OutlinedButtonStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        let colors = &tokens.colors;
        let states = &tokens.states;

        let (background_color, text_color, border_color) = match state {
            ComponentState::Default => (Color::TRANSPARENT, colors.primary.base, colors.outline),
            ComponentState::Hovered => {
                let overlay = apply_state_overlay(
                    Color::TRANSPARENT,
                    colors.primary.base,
                    states.opacity.hover,
                );
                (overlay, colors.primary.base, colors.outline)
            }
            ComponentState::Pressed => {
                let overlay = apply_state_overlay(
                    Color::TRANSPARENT,
                    colors.primary.base,
                    states.opacity.pressed,
                );
                (overlay, colors.primary.base, colors.outline)
            }
            ComponentState::Focused => {
                (Color::TRANSPARENT, colors.primary.base, colors.primary.base)
            }
            ComponentState::Disabled => (Color::TRANSPARENT, colors.on_surface, colors.outline),
            ComponentState::Loading => (
                Color::TRANSPARENT,
                colors.on_surface_variant,
                colors.outline_variant,
            ),
        };

        ComponentStyling {
            background: Background::Color(background_color),
            text_color,
            icon_color: Some(text_color),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: tokens.shapes.corner_medium.radius,
            },
            shadow: None,
            opacity: if matches!(state, ComponentState::Disabled) {
                0.38
            } else {
                1.0
            },
        }
    }
}

/// Strategy for text buttons (no background or border, low emphasis)
struct TextButtonStrategy;

impl ComponentStyleStrategy for TextButtonStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        let colors = &tokens.colors;
        let states = &tokens.states;

        let (background_color, text_color) = match state {
            ComponentState::Default => (Color::TRANSPARENT, colors.primary.base),
            ComponentState::Hovered => {
                let overlay = apply_state_overlay(
                    Color::TRANSPARENT,
                    colors.primary.base,
                    states.opacity.hover,
                );
                (overlay, colors.primary.base)
            }
            ComponentState::Pressed => {
                let overlay = apply_state_overlay(
                    Color::TRANSPARENT,
                    colors.primary.base,
                    states.opacity.pressed,
                );
                (overlay, colors.primary.base)
            }
            ComponentState::Focused => (Color::TRANSPARENT, colors.primary.base),
            ComponentState::Disabled => (Color::TRANSPARENT, colors.on_surface),
            ComponentState::Loading => (Color::TRANSPARENT, colors.on_surface_variant),
        };

        ComponentStyling {
            background: Background::Color(background_color),
            text_color,
            icon_color: Some(text_color),
            border: Border::default(),
            shadow: None,
            opacity: if matches!(state, ComponentState::Disabled) {
                0.38
            } else {
                1.0
            },
        }
    }
}

/// Strategy for elevated buttons (background with shadow, highest emphasis)
struct ElevatedButtonStrategy;

impl ComponentStyleStrategy for ElevatedButtonStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        let colors = &tokens.colors;
        let states = &tokens.states;

        let (background_color, text_color, elevation_level) = match state {
            ComponentState::Default => (colors.surface_container_low, colors.primary.base, 1),
            ComponentState::Hovered => {
                let overlay = apply_state_overlay(
                    colors.surface_container_low,
                    colors.primary.base,
                    states.opacity.hover,
                );
                (overlay, colors.primary.base, 2)
            }
            ComponentState::Pressed => {
                let overlay = apply_state_overlay(
                    colors.surface_container_low,
                    colors.primary.base,
                    states.opacity.pressed,
                );
                (overlay, colors.primary.base, 1)
            }
            ComponentState::Focused => (colors.surface_container_low, colors.primary.base, 1),
            ComponentState::Disabled => (colors.on_surface, colors.surface, 0),
            ComponentState::Loading => (colors.surface_container, colors.on_surface_variant, 0),
        };

        ComponentStyling {
            background: Background::Color(background_color),
            text_color,
            icon_color: Some(text_color),
            border: Border::default(),
            shadow: if elevation_level > 0 {
                Some(create_elevation_shadow(elevation_level))
            } else {
                None
            },
            opacity: if matches!(state, ComponentState::Disabled) {
                0.38
            } else {
                1.0
            },
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Apply a state overlay to a background color
///
/// This implements Material Design's state layer system for interactive feedback.
fn apply_state_overlay(background: Color, overlay_color: Color, opacity: f32) -> Color {
    if opacity == 0.0 {
        return background;
    }

    // Blend the overlay color onto the background
    let alpha = opacity;
    Color::new(
        background.r * (1.0 - alpha) + overlay_color.r * alpha,
        background.g * (1.0 - alpha) + overlay_color.g * alpha,
        background.b * (1.0 - alpha) + overlay_color.b * alpha,
        background.a,
    )
}

/// Create an elevation shadow for buttons
fn create_elevation_shadow(level: u8) -> iced::Shadow {
    let (offset_y, blur, color) = match level {
        1 => (1.0, 3.0, Color::new(0.0, 0.0, 0.0, 0.12)),
        2 => (2.0, 6.0, Color::new(0.0, 0.0, 0.0, 0.16)),
        3 => (4.0, 8.0, Color::new(0.0, 0.0, 0.0, 0.19)),
        _ => (1.0, 3.0, Color::new(0.0, 0.0, 0.0, 0.12)),
    };

    iced::Shadow {
        color,
        offset: iced::Vector::new(0.0, offset_y),
        blur_radius: blur,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::MaterialTokens;

    #[test]
    fn test_filled_button_strategy() {
        let tokens = MaterialTokens::light();
        let strategy = FilledButtonStrategy;

        let styling = strategy.get_styling(ComponentState::Default, &tokens);

        // Filled buttons should have primary background
        if let Background::Color(bg) = styling.background {
            assert_eq!(bg, tokens.colors.primary.base);
        } else {
            panic!("Expected solid color background");
        }

        assert_eq!(styling.text_color, tokens.colors.on_primary());
        assert!(styling.shadow.is_some());
    }

    #[test]
    fn test_outlined_button_strategy() {
        let tokens = MaterialTokens::light();
        let strategy = OutlinedButtonStrategy;

        let styling = strategy.get_styling(ComponentState::Default, &tokens);

        // Outlined buttons should have transparent background
        if let Background::Color(bg) = styling.background {
            assert_eq!(bg, Color::TRANSPARENT);
        } else {
            panic!("Expected transparent background");
        }

        assert_eq!(styling.text_color, tokens.colors.primary.base);
        assert_eq!(styling.border.width, 1.0);
        assert_eq!(styling.border.color, tokens.colors.outline);
    }

    #[test]
    fn test_disabled_state_opacity() {
        let tokens = MaterialTokens::light();
        let strategy = FilledButtonStrategy;

        let styling = strategy.get_styling(ComponentState::Disabled, &tokens);

        assert_eq!(styling.opacity, 0.38);
    }
}
