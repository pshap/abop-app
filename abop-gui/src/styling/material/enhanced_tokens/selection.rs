//! Selection Token Definitions
//!
//! This module contains selection component token definitions
//! following Material Design 3 specifications.

use super::*;

/// Color tokens for selection components
#[derive(Debug, Clone)]
pub struct SelectionColors {
    /// Selection colors
    pub colors: SelectionColorSet,
    /// State layer tokens
    pub state_layers: StateLayerTokens,
}

/// Color set for selection variants
#[derive(Debug, Clone)]
pub struct SelectionColorSet {
    /// Base color for selection elements
    pub base: iced::Color,
    /// Color for content on top of base color
    pub on_base: iced::Color,
}

impl SelectionTokens {
    /// Create new selection tokens
    pub fn new(colors: &MaterialColors, state_layers: &StateLayerTokens) -> Self {
        Self {
            colors: SelectionColorSet {
                base: colors.primary.base,
                on_base: colors.primary.on_base,
            },
            state_layers: state_layers.clone(),
        }
    }
}
