//! Chip strategy implementation for Material Design 3
//!
//! This module provides the chip-specific styling strategy following Material Design 3
//! specifications. Chips are compact elements that support custom icons and have
//! unique pressed state darkening behavior for enhanced visual feedback.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::components::selection_style::lib::constants;
use crate::styling::material::tokens::core::MaterialTokens;
use iced::{Border, Color};

use super::super::{SelectionSize, SelectionState, SelectionStyleStrategy, SelectionVariant};

/// Chip strategy implementation
pub struct ChipStrategy;

impl SelectionStyleStrategy for ChipStrategy {
    fn variant(&self) -> SelectionVariant {
        SelectionVariant::Chip
    }

    fn supports_icons(&self) -> bool {
        true
    }

    fn calculate_background_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        error_state: bool,
    ) -> Color {
        let colors = &tokens.colors;

        // Handle error state first
        if error_state {
            return if state.is_selected() {
                colors.error.base
            } else {
                Color::TRANSPARENT
            };
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
        }; // Apply interaction state effects specific to chips
        if state.is_pressed() && state.is_selected() {
            ColorUtils::darken(
                colors.secondary.container,
                constants::color::CHIP_PRESSED_DARKEN,
            )
        } else if state.is_selected() && (state.is_hovered() || state.is_focused()) {
            colors.secondary.container
        } else {
            base_color
        }
    }

    fn calculate_text_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        _error_state: bool,
    ) -> Color {
        let colors = &tokens.colors;
        if state.is_disabled() {
            return ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED);
        }
        colors.on_surface
    }

    fn calculate_border(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Border {
        let colors = &tokens.colors;
        let border_color = if error_state && !state.is_selected() {
            colors.error.base
        } else if state.is_disabled() {
            // Both selected and unselected disabled states use the same color
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

    fn calculate_foreground_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        error_state: bool,
    ) -> Color {
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

        // Handle selected state - chip shows icon/checkmark
        if state.is_selected() {
            colors.on_primary
        } else {
            colors.on_surface_variant
        }
    }

    fn calculate_state_layer_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
    ) -> Option<Color> {
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
