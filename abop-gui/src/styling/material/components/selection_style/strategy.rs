//! Strategy pattern implementation for selection component styling
//!
//! This module provides the strategy implementations for different selection
//! component variants (Checkbox, Radio, Switch, Chip) following Material Design 3.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::tokens::core::MaterialTokens;
use iced::{Background, Border, Color};

use super::{
    constants, SelectionColors, SelectionSize, SelectionState, SelectionStyleError, SelectionStyling,
    SelectionVariant,
};

/// Strategy trait for selection component styling following Material Design 3
pub trait SelectionStyleStrategy {
    /// Get the selection variant for this strategy
    fn variant(&self) -> SelectionVariant;

    /// Get styling for a specific selection state
    ///
    /// # Arguments
    /// * `state` - The current selection state
    /// * `tokens` - Material Design tokens for consistent styling
    /// * `size` - Component size variant
    /// * `error_state` - Whether the component is in error state
    ///
    /// # Returns
    /// Complete styling configuration for the given state
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let background = self.calculate_background_color(state, tokens, error_state);
        let text_color = self.calculate_text_color(state, tokens, error_state);
        let border = self.calculate_border(state, tokens, size, error_state);
        let foreground_color = self.calculate_foreground_color(state, tokens, error_state);
        let state_layer = self.calculate_state_layer_color(state, tokens);

        Ok(SelectionStyling {
            background: Background::Color(background),
            text_color,
            border,
            shadow: None, // Selection components typically don't use shadows
            foreground_color,
            state_layer,
        })
    }

    /// Calculate the background color for this variant and state
    fn calculate_background_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color;
    
    /// Calculate the text color for this variant and state
    fn calculate_text_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color;
    
    /// Calculate the border for this variant and state
    fn calculate_border(&self, state: SelectionState, tokens: &MaterialTokens, size: SelectionSize, error_state: bool) -> Border;
    
    /// Calculate the foreground color (icon/dot) for this variant and state
    fn calculate_foreground_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color;
    
    /// Calculate the state layer color for interactions
    fn calculate_state_layer_color(&self, state: SelectionState, tokens: &MaterialTokens) -> Option<Color>;

    /// Get the variant name for debugging and logging
    fn variant_name(&self) -> &'static str {
        self.variant().name()
    }

    /// Whether this variant supports error states
    fn supports_error_state(&self) -> bool {
        true
    }

    /// Whether this variant supports custom icons
    fn supports_icons(&self) -> bool {
        false
    }

    /// Whether this variant supports indeterminate state
    fn supports_indeterminate(&self) -> bool {
        false
    }

    /// Get the default size for this variant
    fn default_size(&self) -> SelectionSize {
        SelectionSize::Medium
    }
}

/// Context information for selection styling operations
#[derive(Debug, Clone, Default)]
pub struct SelectionStyleContext {
    /// Whether this component represents a primary selection
    pub is_primary: bool,
    /// Whether the component is in an error state
    pub error_state: bool,
    /// Whether the component has custom content (icons, etc.)
    pub has_custom_content: bool,
    /// Whether the component is part of a group
    pub is_part_of_group: bool,
}

/// Checkbox strategy implementation
pub struct CheckboxStrategy;

impl SelectionStyleStrategy for CheckboxStrategy {
    fn variant(&self) -> SelectionVariant {
        SelectionVariant::Checkbox
    }

    fn supports_indeterminate(&self) -> bool {
        true
    }

    fn calculate_background_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color {
        let colors = &tokens.colors;

        // Handle error state first
        if error_state {
            return if state.is_selected() { colors.error.base } else { Color::TRANSPARENT };
        }

        // Handle disabled state
        if state.is_disabled() {
            return if state.is_selected() {
                ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
            } else {
                Color::TRANSPARENT
            };
        }

        // Default base color for selected state
        let base_color = if state.is_selected() {
            colors.primary.base
        } else {
            Color::TRANSPARENT
        };

        // Apply interaction state effects for checkbox
        if state.is_pressed() && state.is_selected() {
            colors.secondary.container
        } else if state.is_hovered() && state.is_selected() {
            colors.secondary.container
        } else if state.is_focused() && state.is_selected() {
            colors.secondary.container
        } else {
            base_color
        }
    }

    fn calculate_text_color(&self, state: SelectionState, tokens: &MaterialTokens, _error_state: bool) -> Color {
        let colors = &tokens.colors;
        if state.is_disabled() {
            return ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED);
        }
        colors.on_surface
    }

    fn calculate_border(&self, state: SelectionState, tokens: &MaterialTokens, size: SelectionSize, error_state: bool) -> Border {
        let colors = &tokens.colors;

        let border_color = if error_state && !state.is_selected() {
            colors.error.base
        } else if state.is_disabled() {
            if state.is_selected() {
                ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
            } else {
                ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
            }
        } else if state.is_focused() {
            if state.is_selected() {
                colors.on_secondary_container
            } else {
                colors.primary.base
            }
        } else if state.is_selected() {
            colors.primary.base
        } else {
            colors.on_surface_variant
        };

        Border {
            color: border_color,
            width: size.border_width(),
            radius: self.variant().default_border_radius().into(),
        }
    }

    fn calculate_foreground_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color {
        let colors = &tokens.colors;

        // Error state takes highest priority
        if error_state && state.is_selected() {
            return colors.on_error;
        }

        // Handle disabled state
        if state.is_disabled() {
            return if state.is_selected() {
                ColorUtils::with_alpha(colors.on_primary, constants::opacity::DISABLED)
            } else {
                ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
            };
        }

        // Handle selected state - checkbox shows checkmark
        if state.is_selected() {
            colors.on_primary
        } else {
            colors.on_surface_variant
        }
    }

    fn calculate_state_layer_color(&self, state: SelectionState, tokens: &MaterialTokens) -> Option<Color> {
        use constants::opacity::{FOCUS, HOVER, PRESSED};

        if state.is_disabled() {
            return None;
        }

        let colors = &tokens.colors;

        if state.is_pressed() {
            return Some(ColorUtils::with_alpha(colors.on_surface, PRESSED));
        }

        if state.is_hovered() {
            return Some(ColorUtils::with_alpha(colors.on_surface, HOVER));
        }

        if state.is_focused() {
            return Some(ColorUtils::with_alpha(colors.primary.base, FOCUS));
        }

        None
    }
}

/// Radio button strategy implementation  
pub struct RadioStrategy;

impl SelectionStyleStrategy for RadioStrategy {
    fn variant(&self) -> SelectionVariant {
        SelectionVariant::Radio
    }

    fn calculate_background_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color {
        let colors = &tokens.colors;

        // Handle error state first
        if error_state {
            return if state.is_selected() { colors.error.base } else { Color::TRANSPARENT };
        }

        // Handle disabled state
        if state.is_disabled() {
            return if state.is_selected() {
                ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
            } else {
                Color::TRANSPARENT
            };
        }

        // Radio buttons have transparent background, only the dot is colored
        Color::TRANSPARENT
    }

    fn calculate_text_color(&self, state: SelectionState, tokens: &MaterialTokens, _error_state: bool) -> Color {
        let colors = &tokens.colors;
        if state.is_disabled() {
            return ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED);
        }
        colors.on_surface
    }

    fn calculate_border(&self, state: SelectionState, tokens: &MaterialTokens, size: SelectionSize, error_state: bool) -> Border {
        let colors = &tokens.colors;

        let border_color = if error_state && !state.is_selected() {
            colors.error.base
        } else if state.is_disabled() {
            ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
        } else if state.is_focused() {
            if state.is_selected() {
                colors.on_secondary_container
            } else {
                colors.primary.base
            }
        } else if state.is_selected() {
            colors.primary.base
        } else {
            colors.on_surface_variant
        };

        Border {
            color: border_color,
            width: size.border_width(),
            radius: self.variant().default_border_radius().into(),
        }
    }

    fn calculate_foreground_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color {
        let colors = &tokens.colors;

        // Error state takes highest priority
        if error_state && state.is_selected() {
            return colors.error.base;
        }

        // Handle disabled state
        if state.is_disabled() {
            return if state.is_selected() {
                ColorUtils::with_alpha(colors.on_primary, constants::opacity::DISABLED)
            } else {
                ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
            };
        }

        // Handle selected state - radio shows center dot
        if state.is_selected() {
            colors.primary.base
        } else {
            colors.on_surface_variant
        }
    }

    fn calculate_state_layer_color(&self, state: SelectionState, tokens: &MaterialTokens) -> Option<Color> {
        use constants::opacity::{FOCUS, HOVER, PRESSED};

        if state.is_disabled() {
            return None;
        }

        let colors = &tokens.colors;

        if state.is_pressed() {
            return Some(ColorUtils::with_alpha(colors.on_surface, PRESSED));
        }

        if state.is_hovered() {
            return Some(ColorUtils::with_alpha(colors.on_surface, HOVER));
        }

        if state.is_focused() {
            return Some(ColorUtils::with_alpha(colors.primary.base, FOCUS));
        }

        None
    }
}

/// Chip strategy implementation
pub struct ChipStrategy;

impl SelectionStyleStrategy for ChipStrategy {
    fn variant(&self) -> SelectionVariant {
        SelectionVariant::Chip
    }

    fn supports_icons(&self) -> bool {
        true
    }
}

/// Switch strategy implementation
pub struct SwitchStrategy;

impl SelectionStyleStrategy for SwitchStrategy {
    fn variant(&self) -> SelectionVariant {
        SelectionVariant::Switch
    }
}

/// Factory function to create strategy instances
pub fn create_strategy(variant: SelectionVariant) -> Box<dyn SelectionStyleStrategy> {
    match variant {
        SelectionVariant::Checkbox => Box::new(CheckboxStrategy),
        SelectionVariant::Radio => Box::new(RadioStrategy),
        SelectionVariant::Chip => Box::new(ChipStrategy),
        SelectionVariant::Switch => Box::new(SwitchStrategy),
    }
}
