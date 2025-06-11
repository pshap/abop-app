//! Strategy pattern implementation for selection component styling
//!
//! This module provides the strategy trait and factory for different selection
//! component variants (Checkbox, Radio, Switch, Chip) following Material Design 3.

use crate::styling::material::tokens::core::MaterialTokens;
use iced::{Background, Border, Color};

use super::{
    SelectionSize, SelectionState, SelectionStyleError, SelectionStyling, SelectionVariant,
};

// Import strategy implementations from variants module
use super::variants::{CheckboxStrategy, ChipStrategy, RadioStrategy, SwitchStrategy};

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
    fn calculate_background_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        error_state: bool,
    ) -> Color;

    /// Calculate the text color for this variant and state
    fn calculate_text_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        error_state: bool,
    ) -> Color;

    /// Calculate the border for this variant and state
    fn calculate_border(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Border;

    /// Calculate the foreground color (icon/dot) for this variant and state
    fn calculate_foreground_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        error_state: bool,
    ) -> Color;

    /// Calculate the state layer color for interactions
    fn calculate_state_layer_color(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
    ) -> Option<Color>;

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

/// Factory function to create strategy instances
pub fn create_strategy(
    variant: SelectionVariant,
) -> Result<Box<dyn SelectionStyleStrategy>, SelectionStyleError> {
    match variant {
        SelectionVariant::Checkbox => Ok(Box::new(CheckboxStrategy)),
        SelectionVariant::Radio => Ok(Box::new(RadioStrategy)),
        SelectionVariant::Chip => Ok(Box::new(ChipStrategy)),
        SelectionVariant::Switch => Ok(Box::new(SwitchStrategy)),
    }
}

/// Factory function for backward compatibility - panics on error
pub fn create_strategy_unchecked(variant: SelectionVariant) -> Box<dyn SelectionStyleStrategy> {
    create_strategy(variant).expect("Failed to create strategy")
}
