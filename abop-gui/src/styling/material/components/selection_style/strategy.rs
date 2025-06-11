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
    ) -> Result<SelectionStyling, SelectionStyleError>;

    /// Get the variant name for debugging and logging
    fn variant_name(&self) -> &'static str;

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
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::with_tokens(tokens, SelectionVariant::Checkbox)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Checkbox"
    }

    fn supports_indeterminate(&self) -> bool {
        true
    }
}

/// Radio button strategy implementation  
pub struct RadioStrategy;

impl SelectionStyleStrategy for RadioStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::with_tokens(tokens, SelectionVariant::Radio)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Radio"
    }
}

/// Chip strategy implementation
pub struct ChipStrategy;

impl SelectionStyleStrategy for ChipStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::with_tokens(tokens, SelectionVariant::Chip)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Chip"
    }

    fn supports_icons(&self) -> bool {
        true
    }
}

/// Switch strategy implementation
pub struct SwitchStrategy;

impl SelectionStyleStrategy for SwitchStrategy {
    fn get_styling(
        &self,
        state: SelectionState,
        tokens: &MaterialTokens,
        size: SelectionSize,
        error_state: bool,
    ) -> Result<SelectionStyling, SelectionStyleError> {
        let colors = SelectionColors::with_tokens(tokens, SelectionVariant::Switch)
            .with_size(size)
            .with_error(error_state);
        Ok(colors.create_styling(state))
    }

    fn variant_name(&self) -> &'static str {
        "Switch"
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
