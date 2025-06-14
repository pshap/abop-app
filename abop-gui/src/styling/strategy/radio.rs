//! Radio button styling strategies for Material Design 3

use iced::{Background, Border, Color};
use super::{ComponentStyleStrategy, ComponentState, ComponentStyling};
use crate::styling::material::MaterialTokens;
use crate::styling::ColorUtils;

/// Style variant for radio buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioStyleVariant {
    /// Standard Material Design radio button
    Standard,
    /// Error state radio button
    Error,
}

/// Radio button style strategy implementation
/// 
/// Manages styling for Material Design 3 radio buttons with proper state handling.
/// Uses the variant enum to control behavior, eliminating redundant state tracking.
pub struct RadioStyleStrategy {
    variant: RadioStyleVariant,
    selected: bool,
}

impl RadioStyleStrategy {
    /// Create a new standard radio button strategy
    pub fn standard() -> Self {
        Self {
            variant: RadioStyleVariant::Standard,
            selected: false,
        }
    }

    /// Create a new error radio button strategy
    pub fn error() -> Self {
        Self {
            variant: RadioStyleVariant::Error,
            selected: false,
        }
    }

    /// Set selection state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }    /// Check if this radio button is in error state
    /// 
    /// Uses the variant enum to determine error state, eliminating redundant tracking
    fn is_error(&self) -> bool {
        matches!(self.variant, RadioStyleVariant::Error)
    }

    /// Calculate background color based on state
    /// 
    /// Radio button background follows Material Design 3 specifications:
    /// - Error state: Uses error color when selected, transparent when unselected
    /// - Disabled state: Low opacity surface color when selected, transparent when unselected
    /// - Selected state: Primary color for the filled circle
    /// - Unselected state: Transparent background (only border visible)
    fn background_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
          // Error state takes precedence over all other states
        if self.is_error() {
            return if self.selected {
                colors.error.base // Filled with error color when selected
            } else {
                Color::TRANSPARENT // Transparent background, error border only
            };
        }

        // Disabled state has reduced opacity
        if matches!(state, ComponentState::Disabled) {
            return if self.selected {
                ColorUtils::with_alpha(colors.on_surface, 0.12) // Low opacity when disabled+selected
            } else {
                Color::TRANSPARENT // No background when disabled+unselected
            };
        }

        // Normal states: filled when selected, transparent when unselected
        if self.selected {
            colors.primary.base // Primary color for selected radio dot
        } else {
            Color::TRANSPARENT // Only border visible when unselected
        }
    }

    /// Calculate border for radio button
    fn border_style(&self, state: ComponentState, tokens: &MaterialTokens) -> Border {
        let colors = &tokens.colors;
          let color = if self.is_error() {
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
            radius: 10.0.into(), // Radio buttons are circular
        }
    }

    /// Calculate foreground (dot) color
    fn foreground_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
          if self.is_error() && self.selected {
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
            Color::TRANSPARENT
        }
    }    /// Get state layer opacity for interaction states
    /// 
    /// Returns opacity values following Material Design 3 state layer specifications:
    /// - Pressed: 0.12 (12%) - High feedback for direct interaction
    /// - Hovered: 0.08 (8%) - Subtle hover indication
    /// - Focused: 0.10 (10%) - Clear focus visibility for accessibility
    /// - Loading: 0.08 (8%) - Similar to hover for loading states
    /// - Disabled/Default: 0.0 (0%) - No state layer overlay
    /// 
    /// Note: This will be used in future implementation of interactive state overlays
    /// to provide visual feedback during user interactions with radio buttons.
    fn state_layer_opacity(&self, state: ComponentState) -> f32 {
        match state {
            ComponentState::Pressed => 0.12,  // MD3: Strong press feedback
            ComponentState::Hovered => 0.08,  // MD3: Subtle hover indication
            ComponentState::Focused => 0.10,  // MD3: Accessibility-focused visibility
            ComponentState::Loading => 0.08,  // MD3: Loading state indication
            ComponentState::Disabled => 0.0,  // MD3: No overlay for disabled
            ComponentState::Default => 0.0,   // MD3: No overlay in default state
        }
    }
}

impl ComponentStyleStrategy for RadioStyleStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        // Calculate colors once to avoid redundant calculations
        let foreground = self.foreground_color(state, tokens);
        
        ComponentStyling {
            background: Background::Color(self.background_color(state, tokens)),
            border: self.border_style(state, tokens),
            text_color: foreground,
            icon_color: Some(foreground),
            shadow: None, // Radio buttons typically don't have shadows
            opacity: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_standard_styling() {
        let strategy = RadioStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(Color::TRANSPARENT));
        assert_eq!(styling.border.color, tokens.colors.on_surface_variant);
    }

    #[test]
    fn test_radio_selected_styling() {
        let strategy = RadioStyleStrategy::standard().selected(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        assert_eq!(styling.background, Background::Color(tokens.colors.primary.base));
        assert_eq!(styling.border.color, tokens.colors.primary.base);
        assert_eq!(styling.text_color, tokens.colors.on_primary());
    }

    #[test]
    fn test_radio_error_styling() {
        let strategy = RadioStyleStrategy::error().selected(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Default, &tokens);
        // Error state should use error colors
        assert_eq!(styling.border.color, tokens.colors.error.base);
    }

    #[test]
    fn test_radio_disabled_styling() {
        let strategy = RadioStyleStrategy::standard().selected(true);
        let tokens = MaterialTokens::default();
        
        let styling = strategy.get_styling(ComponentState::Disabled, &tokens);
        // Disabled state should have reduced opacity
        assert_eq!(styling.background, Background::Color(ColorUtils::with_alpha(tokens.colors.on_surface, 0.12)));
    }

    #[test]
    fn test_radio_interaction_states() {
        let strategy = RadioStyleStrategy::standard();
        let tokens = MaterialTokens::default();
        
        // Test hover state
        let hover_styling = strategy.get_styling(ComponentState::Hovered, &tokens);
        assert_eq!(hover_styling.border.color, tokens.colors.on_surface_variant);
        
        // Test focus state
        let focus_styling = strategy.get_styling(ComponentState::Focused, &tokens);
        assert_eq!(focus_styling.border.color, tokens.colors.primary.base);
    }
}
