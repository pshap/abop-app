//! Strategy pattern implementation for selection component styling
//!
//! This module provides the strategy implementations for different selection
//! component variants (Checkbox, Radio, Switch, Chip) following Material Design 3.

use crate::styling::material::tokens::core::MaterialTokens;

use super::{
    SelectionColors, SelectionSize, SelectionState, SelectionStyleError, SelectionStyling,
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
        let colors = SelectionColors::with_tokens(tokens, self.variant())
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

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
}

/// Radio button strategy implementation  
pub struct RadioStrategy;

impl SelectionStyleStrategy for RadioStrategy {
    fn variant(&self) -> SelectionVariant {
        SelectionVariant::Radio
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
