//! Radio button strategy implementation for Material Design 3
//!
//! This module provides the radio button-specific styling strategy following Material Design 3
//! specifications. Radio buttons are circular selection controls that show a center dot
//! when selected and maintain transparent backgrounds.

use crate::styling::color_utils::ColorUtils;
use crate::styling::material::tokens::core::MaterialTokens;
use crate::styling::material::components::selection_style::lib::constants;
use iced::{Border, Color};

use super::super::{
    SelectionSize, SelectionState, SelectionVariant, SelectionStyleStrategy,
};

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
    }    fn calculate_text_color(&self, state: SelectionState, tokens: &MaterialTokens, error_state: bool) -> Color {
        let colors = &tokens.colors;
        
        // Handle error state first
        if error_state {
            return colors.error.base;
        }
        
        if state.is_disabled() {
            return ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED);
        }
        colors.on_surface
    }

    fn calculate_border(&self, state: SelectionState, tokens: &MaterialTokens, size: SelectionSize, error_state: bool) -> Border {
        let colors = &tokens.colors;        let border_color = if error_state && !state.is_selected() {
            colors.error.base
        } else if state.is_disabled() {
            ColorUtils::with_alpha(colors.on_surface, constants::opacity::DISABLED)        } else if state.is_focused() {
            colors.primary.base
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
