//! Switch styling strategies for Material Design 3

use iced::{Background, Border, Color};
use super::{ComponentStyleStrategy, ComponentState, ComponentStyling};
use crate::styling::material::MaterialTokens;
use crate::styling::ColorUtils;

/// Style variant for switches
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchStyleVariant {
    /// Standard Material Design switch
    Standard,
    /// Error state switch
    Error,
}

/// Switch style strategy implementation
pub struct SwitchStyleStrategy {
    #[allow(dead_code)] // Used for future variant-specific behavior
    variant: SwitchStyleVariant,
    enabled: bool,
    error: bool,
}

impl SwitchStyleStrategy {
    /// Create a new standard switch strategy
    pub fn standard() -> Self {
        Self {
            variant: SwitchStyleVariant::Standard,
            enabled: false,
            error: false,
        }
    }

    /// Create a new error switch strategy
    pub fn error() -> Self {
        Self {
            variant: SwitchStyleVariant::Error,
            enabled: false,
            error: true,
        }
    }

    /// Set enabled state
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Calculate track (background) color based on state
    fn track_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        if self.error {
            return if self.enabled {
                colors.error.base
            } else {
                ColorUtils::with_alpha(colors.error.base, 0.12)
            };
        }

        if matches!(state, ComponentState::Disabled) {
            return ColorUtils::with_alpha(colors.on_surface, 0.12);
        }

        if self.enabled {
            colors.primary.base
        } else {
            colors.surface_variant
        }
    }

    /// Calculate thumb color based on state
    fn thumb_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        if self.error {
            return if self.enabled {
                colors.on_error()
            } else {
                colors.on_surface_variant
            };
        }

        if matches!(state, ComponentState::Disabled) {
            return ColorUtils::with_alpha(
                if self.enabled {
                    colors.surface
                } else {
                    colors.on_surface
                },
                0.38,
            );
        }

        if self.enabled {
            colors.on_primary()
        } else {
            colors.outline
        }
    }

    /// Calculate border for switch track
    fn border_style(&self, state: ComponentState, tokens: &MaterialTokens) -> Border {
        let colors = &tokens.colors;
        
        let color = if self.error {
            colors.error.base
        } else if matches!(state, ComponentState::Disabled) {
            ColorUtils::with_alpha(colors.on_surface, 0.12)
        } else if self.enabled {
            Color::TRANSPARENT // No border when enabled
        } else {
            colors.outline
        };

        Border {
            color,
            width: if self.enabled { 0.0 } else { 2.0 },
            radius: 16.0.into(), // Switches have rounded track
        }
    }    /// Get state layer opacity for interaction states
    #[allow(dead_code)] // Future enhancement
    fn state_layer_opacity(&self, state: ComponentState) -> f32 {
        match state {
            ComponentState::Pressed => 0.12,
            ComponentState::Hovered => 0.08,
            ComponentState::Focused => 0.10,
            ComponentState::Loading => 0.08,
            ComponentState::Disabled => 0.0,
            ComponentState::Default => 0.0,
        }
    }
}

impl ComponentStyleStrategy for SwitchStyleStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        ComponentStyling {
            background: Background::Color(self.track_color(state, tokens)),
            border: self.border_style(state, tokens),
            text_color: self.thumb_color(state, tokens), // Using text_color for thumb
            icon_color: Some(self.thumb_color(state, tokens)),
            shadow: None, // Switches typically don't have shadows on track
            opacity: self.state_layer_opacity(state), // Use calculated state layer opacity
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_standard_styling() {
        let strategy = SwitchStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(tokens.colors.surface_variant));
        assert_eq!(styling.border.color, tokens.colors.outline);
        assert_eq!(styling.text_color, tokens.colors.outline);
    }

    #[test]
    fn test_switch_enabled_styling() {
        let strategy = SwitchStyleStrategy::standard().enabled(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(tokens.colors.primary.base));
        assert_eq!(styling.border.width, 0.0); // No border when enabled
        assert_eq!(styling.text_color, tokens.colors.on_primary());
    }

    #[test]
    fn test_switch_error_styling() {
        let strategy = SwitchStyleStrategy::error().enabled(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        // Error state should use error colors
        assert_eq!(styling.border.color, tokens.colors.error.base);
        assert_eq!(styling.background, Background::Color(tokens.colors.error.base));
    }

    #[test]
    fn test_switch_disabled_styling() {
        let strategy = SwitchStyleStrategy::standard().enabled(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Disabled, &tokens);
        // Disabled state should have reduced opacity
        assert_eq!(styling.background, Background::Color(ColorUtils::with_alpha(tokens.colors.on_surface, 0.12)));
    }

    #[test]
    fn test_switch_interaction_states() {
        let strategy = SwitchStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        // Test hover state - should maintain same styling as default for track
        let hover_styling = strategy.get_styling(ComponentState::Hovered, &tokens);
        assert_eq!(hover_styling.background, Background::Color(tokens.colors.surface_variant));
        
        // Test focus state
        let focus_styling = strategy.get_styling(ComponentState::Focused, &tokens);
        assert_eq!(focus_styling.background, Background::Color(tokens.colors.surface_variant));
    }
}
