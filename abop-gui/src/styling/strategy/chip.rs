//! Chip styling strategies for Material Design 3

use iced::{Background, Border, Color};
use super::{ComponentStyleStrategy, ComponentState, ComponentStyling};
use crate::styling::material::MaterialTokens;
use crate::styling::ColorUtils;

/// Style variant for chips
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipStyleVariant {
    /// Assist chip - helps users with a task
    Assist,
    /// Filter chip - filters content
    Filter,
    /// Input chip - represents user input
    Input,
    /// Suggestion chip - suggests actions
    Suggestion,
}

/// Chip style strategy implementation
pub struct ChipStyleStrategy {
    variant: ChipStyleVariant,
    selected: bool,
    elevated: bool,
    error: bool,
}

impl ChipStyleStrategy {
    /// Create a new assist chip strategy
    pub fn assist() -> Self {
        Self {
            variant: ChipStyleVariant::Assist,
            selected: false,
            elevated: false,
            error: false,
        }
    }

    /// Create a new filter chip strategy
    pub fn filter() -> Self {
        Self {
            variant: ChipStyleVariant::Filter,
            selected: false,
            elevated: false,
            error: false,
        }
    }

    /// Create a new input chip strategy
    pub fn input() -> Self {
        Self {
            variant: ChipStyleVariant::Input,
            selected: false,
            elevated: false,
            error: false,
        }
    }

    /// Create a new suggestion chip strategy
    pub fn suggestion() -> Self {
        Self {
            variant: ChipStyleVariant::Suggestion,
            selected: false,
            elevated: false,
            error: false,
        }
    }

    /// Set selection state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set elevated state
    pub fn elevated(mut self, elevated: bool) -> Self {
        self.elevated = elevated;
        self
    }

    /// Set error state
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Calculate background color based on variant and state
    fn background_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        if self.error {            return if self.selected {
                colors.error_container()
            } else {
                ColorUtils::with_alpha(colors.error.base, 0.12)
            };
        }

        if matches!(state, ComponentState::Disabled) {
            return ColorUtils::with_alpha(colors.on_surface, 0.12);
        }

        match self.variant {
            ChipStyleVariant::Assist => {
                if self.elevated {
                    colors.surface_container_low
                } else {
                    Color::TRANSPARENT
                }
            }
            ChipStyleVariant::Filter => {                if self.selected {
                    colors.secondary_container()
                } else if self.elevated {
                    colors.surface_container_low
                } else {
                    Color::TRANSPARENT
                }
            }
            ChipStyleVariant::Input => {
                if self.elevated {
                    colors.surface_container_low
                } else {
                    Color::TRANSPARENT
                }
            }
            ChipStyleVariant::Suggestion => {
                if self.elevated {
                    colors.surface_container_low
                } else {
                    Color::TRANSPARENT
                }
            }
        }
    }

    /// Calculate border for chip
    fn border_style(&self, state: ComponentState, tokens: &MaterialTokens) -> Border {
        let colors = &tokens.colors;
        
        let color = if self.error {
            colors.error.base
        } else if matches!(state, ComponentState::Disabled) {
            ColorUtils::with_alpha(colors.on_surface, 0.12)
        } else if self.elevated || self.selected {
            Color::TRANSPARENT
        } else {
            colors.outline
        };

        let width = if self.elevated || self.selected || self.error { 0.0 } else { 1.0 };

        Border {
            color,
            width,
            radius: 8.0.into(), // Chips have rounded corners
        }
    }

    /// Calculate text/content color
    fn text_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        if self.error {
            return if self.selected {
                colors.on_error_container()
            } else {
                colors.error.base
            };
        }

        if matches!(state, ComponentState::Disabled) {
            return ColorUtils::with_alpha(colors.on_surface, 0.38);
        }

        match self.variant {
            ChipStyleVariant::Assist => colors.on_surface,
            ChipStyleVariant::Filter => {
                if self.selected {
                    colors.on_secondary_container()
                } else {
                    colors.on_surface_variant
                }
            }
            ChipStyleVariant::Input => colors.on_surface,
            ChipStyleVariant::Suggestion => colors.on_surface_variant,
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

impl ComponentStyleStrategy for ChipStyleStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        ComponentStyling {
            background: Background::Color(self.background_color(state, tokens)),
            border: self.border_style(state, tokens),
            text_color: self.text_color(state, tokens),
            icon_color: Some(self.text_color(state, tokens)),
            shadow: None, // Chips can have elevation but handled separately
            opacity: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assist_chip_styling() {
        let strategy = ChipStyleStrategy::assist();
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(Color::TRANSPARENT));
        assert_eq!(styling.border.color, tokens.colors.outline);
        assert_eq!(styling.text_color, tokens.colors.on_surface);
    }

    #[test]
    fn test_filter_chip_selected_styling() {
        let strategy = ChipStyleStrategy::filter().selected(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(tokens.colors.secondary_container()));
        assert_eq!(styling.border.width, 0.0); // No border when selected
        assert_eq!(styling.text_color, tokens.colors.on_secondary_container());
    }

    #[test]
    fn test_chip_elevated_styling() {
        let strategy = ChipStyleStrategy::assist().elevated(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(tokens.colors.surface_container_low));
        assert_eq!(styling.border.width, 0.0); // No border when elevated
    }

    #[test]
    fn test_chip_error_styling() {
        let strategy = ChipStyleStrategy::input().error(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.border.color, tokens.colors.error.base);
        assert_eq!(styling.text_color, tokens.colors.error.base);
    }

    #[test]
    fn test_chip_disabled_styling() {
        let strategy = ChipStyleStrategy::suggestion();
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Disabled, &tokens);
        // Disabled state should have reduced opacity
        assert_eq!(styling.background, Background::Color(ColorUtils::with_alpha(tokens.colors.on_surface, 0.12)));
        assert_eq!(styling.text_color, ColorUtils::with_alpha(tokens.colors.on_surface, 0.38));
    }

    #[test]
    fn test_all_chip_variants() {
        let tokens = MaterialTokens::default();
        
        // Test all variants can be created and styled
        let variants = vec![
            ChipStyleStrategy::assist(),
            ChipStyleStrategy::filter(),
            ChipStyleStrategy::input(),
            ChipStyleStrategy::suggestion(),
        ];
        
        for strategy in variants {
            let styling = strategy.get_styling(ComponentState::Default, &tokens);
            // All should have valid styling
            assert!(styling.border.radius.top_left >= 0.0);
        }
    }
}
