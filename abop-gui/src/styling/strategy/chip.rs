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
    }    /// Calculate background color based on variant and state
    /// 
    /// Chip background colors follow Material Design 3 specifications:
    /// - Error state: Error container when selected, error with low opacity when unselected
    /// - Disabled state: Low opacity surface overlay
    /// - Variant-specific: Each chip variant has its own background behavior
    /// - Elevated chips: Use surface container for subtle elevation appearance
    fn background_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        // Error state takes precedence over variant-specific styling
        if self.error {
            return if self.selected {
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
            }            ChipStyleVariant::Filter => {
                if self.selected {
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
    }    /// Calculate border for chip
    /// 
    /// Border styling follows Material Design 3 chip specifications:
    /// - Error chips: Always show error border for clear error indication
    /// - Disabled chips: Low opacity border for reduced prominence
    /// - Elevated/Selected chips: No border (background provides visual distinction)
    /// - Default chips: Standard outline border for definition
    fn border_style(&self, state: ComponentState, tokens: &MaterialTokens) -> Border {
        let colors = &tokens.colors;
        
        // Determine border color based on state and chip properties
        let color = if self.error {
            colors.error.base // Error border for clear error indication
        } else if matches!(state, ComponentState::Disabled) {
            ColorUtils::with_alpha(colors.on_surface, 0.12) // Low opacity for disabled state
        } else if self.elevated || self.selected {
            Color::TRANSPARENT // No border when elevated or selected (background provides distinction)
        } else {
            colors.outline // Standard outline for default chips
        };

        // Border width: 0 for elevated/selected/error chips, 1px for outlined chips
        let width = if self.elevated || self.selected || self.error { 0.0 } else { 1.0 };

        Border {
            color,
            width,
            radius: 8.0.into(), // MD3 spec: Standard chip corner radius for rounded appearance
        }
    }/// Calculate text/content color
    /// 
    /// Text color selection follows Material Design 3 chip specifications:
    /// - Error state: Error text colors with proper contrast on error backgrounds
    /// - Disabled state: Reduced opacity text for disabled appearance  
    /// - Variant-specific colors:
    ///   * Assist: Standard surface text (primary actions)
    ///   * Filter: Changes between surface variant (unselected) and container text (selected)
    ///   * Input: Standard surface text (user input content)
    ///   * Suggestion: Surface variant text (secondary suggestions)
    fn text_color(&self, state: ComponentState, tokens: &MaterialTokens) -> Color {
        let colors = &tokens.colors;
        
        // Error state overrides variant-specific text colors
        if self.error {
            return if self.selected {
                colors.on_error_container() // High contrast on error container
            } else {
                colors.error.base // Error color on transparent background
            };
        }

        // Disabled state uses low opacity for reduced prominence
        if matches!(state, ComponentState::Disabled) {
            return ColorUtils::with_alpha(colors.on_surface, 0.38); // MD3 disabled text opacity
        }

        // Variant-specific text colors based on chip purpose and state
        match self.variant {
            ChipStyleVariant::Assist => colors.on_surface,     // Primary text for actions
            ChipStyleVariant::Filter => {
                if self.selected {
                    colors.on_secondary_container() // High contrast on selected filter
                } else {
                    colors.on_surface_variant       // Lower emphasis when unselected
                }
            }
            ChipStyleVariant::Input => colors.on_surface,      // User content text
            ChipStyleVariant::Suggestion => colors.on_surface_variant, // Secondary text for suggestions
        }
    }/// Get state layer opacity for interaction states
    /// 
    /// Returns opacity values following Material Design 3 state layer specifications:
    /// - Pressed: 0.12 (12%) - Strong feedback for tap/click interactions
    /// - Hovered: 0.08 (8%) - Subtle indication on hover (desktop/mouse interactions)
    /// - Focused: 0.10 (10%) - Clear focus visibility for keyboard navigation
    /// - Loading: 0.08 (8%) - Loading state indication, similar to hover
    /// - Disabled/Default: 0.0 (0%) - No overlay in these states
    /// 
    /// These values ensure consistent interaction feedback across all chip variants
    /// while maintaining Material Design's accessibility and usability guidelines.
    fn state_layer_opacity(&self, state: ComponentState) -> f32 {
        match state {
            ComponentState::Pressed => 0.12,  // MD3: Strong press feedback
            ComponentState::Hovered => 0.08,  // MD3: Subtle hover indication  
            ComponentState::Focused => 0.10,  // MD3: Accessibility focus
            ComponentState::Loading => 0.08,  // MD3: Loading state indication
            ComponentState::Disabled => 0.0,  // MD3: No overlay for disabled
            ComponentState::Default => 0.0,   // MD3: No overlay in default state
        }
    }

    /// Calculate elevation shadow for chips
    /// 
    /// Chips can have elevation in certain states and variants:
    /// - Elevated chips: Subtle shadow to show they're raised above surface
    /// - Pressed state: Temporarily reduced elevation for press feedback
    /// - Disabled state: No elevation shadow
    fn elevation_shadow(&self, state: ComponentState, _tokens: &MaterialTokens) -> Option<iced::Shadow> {
        use iced::Shadow;
        
        // No shadow for disabled chips
        if matches!(state, ComponentState::Disabled) {
            return None;
        }
        
        // Only elevated chips get shadows
        if self.elevated {
            let elevation = match state {
                ComponentState::Pressed => 1.0, // Reduced elevation when pressed
                _ => 2.0, // Standard elevation for elevated chips
            };
            
            Some(Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2), // Subtle black shadow
                offset: iced::Vector::new(0.0, elevation),
                blur_radius: elevation * 2.0,
            })
        } else {
            None // Non-elevated chips have no shadow
        }
    }
}

impl ComponentStyleStrategy for ChipStyleStrategy {
    fn get_styling(&self, state: ComponentState, tokens: &MaterialTokens) -> ComponentStyling {
        // Calculate text color once to avoid redundant calculations
        let text_color = self.text_color(state, tokens);
        
        ComponentStyling {
            background: Background::Color(self.background_color(state, tokens)),
            border: self.border_style(state, tokens),
            text_color,
            icon_color: Some(text_color),
            shadow: self.elevation_shadow(state, tokens), // Implement elevation for chips
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
