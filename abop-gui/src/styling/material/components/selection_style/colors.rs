//! Color calculation system for selection components
//!
//! This module provides comprehensive color logic using Material Design 3 tokens
//! for consistent selection component styling.

use iced::{Background, Border, Color};

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::tokens::core::MaterialTokens;

use super::{
    constants,
    state::{SelectionSize, SelectionState, SelectionStyling, SelectionVariant},
};

/// Enhanced color calculation for selection components using Material Design 3 tokens
///
/// This structure provides comprehensive color logic while leveraging
/// the centralized `MaterialTokens` system for consistency and maintainability.
#[derive(Debug, Clone)]
pub struct SelectionColors {
    /// The material token system to use
    tokens: MaterialTokens,
    /// Current component variant
    variant: SelectionVariant,
    /// Component size
    size: SelectionSize,
    /// Whether the component is in an error state
    error_state: bool,
}

impl SelectionColors {
    /// Create new selection colors with the given token system
    #[must_use]
    pub fn new(tokens: MaterialTokens, variant: SelectionVariant) -> Self {
        Self {
            tokens,
            variant,
            size: SelectionSize::Medium,
            error_state: false,
        }
    }

    /// Create selection colors with borrowed tokens (optimized)
    #[must_use]
    pub fn with_tokens(tokens: &MaterialTokens, variant: SelectionVariant) -> Self {
        Self {
            tokens: tokens.clone(), // Only clone when necessary
            variant,
            size: SelectionSize::Medium,
            error_state: false,
        }
    }

    /// Set the size for this selection component
    #[must_use]
    pub fn with_size(mut self, size: SelectionSize) -> Self {
        self.size = size;
        self
    }

    /// Set the error state for this selection component
    #[must_use]
    pub fn with_error(mut self, error_state: bool) -> Self {
        self.error_state = error_state;
        self
    }

    /// Apply error state color if applicable
    #[must_use]
    fn apply_error_state(&self, state: SelectionState) -> Option<Color> {
        if !self.error_state {
            return None;
        }

        let colors = &self.tokens.colors;
        Some(if state.is_selected() {
            colors.error.base
        } else {
            colors.error.base // Error state for unselected (border)
        })
    }

    /// Apply disabled state color if applicable
    #[must_use]
    fn apply_disabled_state(&self, state: SelectionState, for_selected: bool) -> Option<Color> {
        if !state.is_disabled() {
            return None;
        }

        let colors = &self.tokens.colors;
        Some(if for_selected && state.is_selected() {
            ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
        } else {
            Color::TRANSPARENT
        })
    }

    /// Calculate the primary component color (background, border, or fill)
    ///
    /// This method centralizes the color logic using Material Design 3 token system.
    #[must_use]
    pub fn primary_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        // Handle error state first
        if let Some(error_color) = self.apply_error_state(state) {
            return if state.is_selected() {
                error_color
            } else {
                Color::TRANSPARENT
            };
        }

        // Handle disabled state
        if let Some(disabled_color) = self.apply_disabled_state(state, true) {
            return disabled_color;
        }

        // Default base color for selected state
        let base_color = if state.is_selected() {
            colors.primary.base
        } else {
            Color::TRANSPARENT
        };

        // Apply interaction state effects
        if state.is_pressed() {
            if state.is_selected() {
                match self.variant {
                    SelectionVariant::Chip => ColorUtils::darken(
                        colors.secondary.container,
                        constants::color::CHIP_PRESSED_DARKEN,
                    ),
                    _ => colors.secondary.container,
                }
            } else {
                Color::TRANSPARENT
            }
        } else if state.is_hovered() {
            if state.is_selected() {
                match self.variant {
                    SelectionVariant::Chip => colors.secondary.container,
                    _ => colors.secondary.container,
                }
            } else {
                Color::TRANSPARENT
            }
        } else if state.is_focused() {
            if state.is_selected() {
                colors.secondary.container
            } else {
                Color::TRANSPARENT
            }
        } else {
            base_color
        }
    }

    /// Calculate the border color for the selection component
    #[must_use]
    pub fn border_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        // Handle error state first (for unselected components)
        if let Some(error_color) = self.apply_error_state(state) {
            return if !state.is_selected() {
                error_color
            } else {
                colors.primary.base
            };
        }

        // Handle disabled state
        if let Some(disabled_color) = self.apply_disabled_state(state, true) {
            return disabled_color;
        }

        // Handle focused state
        if state.is_focused() {
            if state.is_selected() {
                return colors.on_secondary_container;
            }
            return colors.primary.base;
        }

        // Default state
        if state.is_selected() {
            return colors.primary.base;
        }

        // Unselected state
        colors.on_surface_variant
    }

    /// Calculate the foreground color (text, icon, or dot)
    #[must_use]
    pub fn foreground_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;

        match (state, self.error_state, self.variant) {
            // Error state takes highest priority
            (state, true, SelectionVariant::Checkbox) if state.is_selected() => colors.on_error,
            (state, true, SelectionVariant::Radio) if state.is_selected() => colors.error.base,
            (state, true, SelectionVariant::Chip) if state.is_selected() => colors.on_error,
            (state, true, SelectionVariant::Switch) if state.is_selected() => colors.on_error,

            // Handle disabled state
            (state, _, _) if state.is_disabled() => {
                if state.is_selected() {
                    ColorUtils::with_alpha(colors.on_primary, constants::opacity::DISABLED)
                } else {
                    ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)
                }
            }

            // Handle selected state
            (state, _, SelectionVariant::Checkbox) if state.is_selected() => colors.on_primary,
            (state, _, SelectionVariant::Radio) if state.is_selected() => colors.primary.base,
            (state, _, SelectionVariant::Chip) if state.is_selected() => colors.on_primary,
            (state, _, SelectionVariant::Switch) if state.is_selected() => colors.on_primary,

            // Default state
            _ => colors.on_surface_variant,
        }
    }

    /// Calculate the text color for component labels
    #[must_use]
    pub fn text_color(&self, state: SelectionState) -> Color {
        let colors = &self.tokens.colors;
        if state.is_disabled() {
            return ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED);
        }
        colors.on_surface
    }

    /// Calculate state layer color for interactions
    #[must_use]
    pub fn state_layer_color(&self, state: SelectionState) -> Option<Color> {
        use constants::opacity::{FOCUS, HOVER, PRESSED};

        if state.is_disabled() {
            return None;
        }

        let colors = &self.tokens.colors;

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

    /// Get border configuration for the component
    #[must_use]
    pub fn border(&self, state: SelectionState) -> Border {
        Border {
            color: self.border_color(state),
            width: self.size.border_width(),
            radius: self.variant.default_border_radius().into(),
        }
    }

    /// Create complete styling for the given state
    #[must_use]
    pub fn create_styling(&self, state: SelectionState) -> SelectionStyling {
        SelectionStyling {
            background: Background::Color(self.primary_color(state)),
            text_color: self.text_color(state),
            border: self.border(state),
            shadow: None, // Selection components typically don't use shadows
            foreground_color: self.foreground_color(state),
            state_layer: self.state_layer_color(state),
        }
    }
}
