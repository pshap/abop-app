//! Checkbox style strategy implementation
//!
//! This module provides the checkbox styling strategy following Material Design 3
//! specifications for the unified strategy system.

use iced::{Background, Border, Color};
use crate::styling::material::tokens::core::MaterialTokens;
use crate::styling::color_utils::ColorUtils;

use super::{
    ComponentStyleStrategy, ComponentStyling,
    traits::ComponentState,
};

/// Material Design 3 checkbox style variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxStyleVariant {
    /// Standard checkbox
    Standard,
    /// Checkbox with error state
    Error,
}

/// Checkbox style strategy implementation
pub struct CheckboxStyleStrategy {
    #[allow(dead_code)] // Used for future variant-specific behavior
    variant: CheckboxStyleVariant,
    selected: bool,
    error: bool,
}

impl CheckboxStyleStrategy {
    /// Create a new standard checkbox strategy
    pub fn standard() -> Self {
        Self {
            variant: CheckboxStyleVariant::Standard,
            selected: false,
            error: false,
        }
    }

    /// Create a new error checkbox strategy
    pub fn error() -> Self {
        Self {
            variant: CheckboxStyleVariant::Error,
            selected: false,
            error: true,
        }
    }

    /// Set selection state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }    /// Calculate background color based on state
    fn background_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        if self.error {
            return if self.selected {
                colors.error.base
            } else {
                Color::TRANSPARENT
            };
        }

        if matches!(state, ComponentState::Disabled) {
            return if self.selected {
                ColorUtils::with_alpha(colors.on_surface, 0.12)
            } else {
                Color::TRANSPARENT
            };
        }

        if self.selected {
            colors.primary.base
        } else {
            Color::TRANSPARENT
        }
    }    /// Calculate border for checkbox
    fn border_style(&self, state: ComponentState, tokens: &MaterialTokens) -> Border {
        let colors = &tokens.colors;
          let color = if self.error {
            colors.error.base
        } else if matches!(state, ComponentState::Disabled) {
            ColorUtils::with_alpha(colors.on_surface, 0.38)
        } else if self.selected {
            colors.primary.base
        } else if matches!(state, ComponentState::Focused) {
            colors.primary.base
        } else {
            colors.on_surface_variant
        };

        Border {
            color,
            width: 2.0,
            radius: 2.0.into(),
        }
    }

    /// Calculate foreground (checkmark) color
    fn foreground_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        if self.error && self.selected {
            return colors.on_error();
        }

        if matches!(state, ComponentState::Disabled) {
            return ColorUtils::with_alpha(
                if self.selected {
                    colors.surface
                } else {
                    colors.on_surface
                },
                0.38,
            );
        }

        if self.selected {
            colors.on_primary()
        } else {
            Color::TRANSPARENT        }
    }
}

impl ComponentStyleStrategy for CheckboxStyleStrategy {fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        ComponentStyling {
            background: Background::Color(self.background_color(state, tokens)),
            text_color: tokens.colors.on_surface,
            border: self.border_style(state, tokens),
            shadow: None, // Checkboxes don't use elevation
            icon_color: Some(self.foreground_color(state, tokens)),
            opacity: if matches!(state, ComponentState::Disabled) { 0.38 } else { 1.0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::styling::material::tokens::core::MaterialTokens;

    #[test]
    fn test_checkbox_standard_styling() {
        let strategy = CheckboxStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        // Test default state
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        
        assert_eq!(styling.background, Background::Color(Color::TRANSPARENT));
        assert_eq!(styling.shadow, None);
    }

    #[test]
    fn test_checkbox_selected_styling() {
        let strategy = CheckboxStyleStrategy::standard().selected(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
          // Selected checkbox should have primary background
        if let Background::Color(bg_color) = styling.background {
            assert_eq!(bg_color, tokens.colors.primary.base);
        }
    }

    #[test]
    fn test_checkbox_error_styling() {
        let strategy = CheckboxStyleStrategy::error().selected(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
          // Error state should use error colors
        assert_eq!(styling.border.color, tokens.colors.error.base);
    }

    #[test]
    fn test_checkbox_disabled_styling() {
        let strategy = CheckboxStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Disabled, &tokens);
        
        // Disabled state should have reduced opacity
        assert_eq!(styling.opacity, 0.38);
    }    #[test]
    fn test_checkbox_interaction_states() {
        let strategy = CheckboxStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        // Test hover state
        let hovered_styling = strategy.get_styling(ComponentState::Hovered, &tokens);
        assert_eq!(hovered_styling.opacity, 1.0);
        
        // Test pressed state
        let pressed_styling = strategy.get_styling(ComponentState::Pressed, &tokens);
        assert_eq!(pressed_styling.opacity, 1.0);
    }
}
