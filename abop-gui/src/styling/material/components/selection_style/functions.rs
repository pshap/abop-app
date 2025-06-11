//! Convenience factory functions for selection component styling
//!
//! This module provides easy-to-use factory functions for creating
//! selection component styles following Material Design 3.

use crate::styling::material::tokens::core::MaterialTokens;

use super::{builder::SelectionStyleBuilder, state::SelectionVariant};

/// Create a selection style builder for checkbox components
#[must_use]
pub fn checkbox_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::with_tokens(tokens, SelectionVariant::Checkbox)
}

/// Create a selection style builder for radio button components
#[must_use]
pub fn radio_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::with_tokens(tokens, SelectionVariant::Radio)
}

/// Create a selection style builder for chip components
#[must_use]
pub fn chip_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::with_tokens(tokens, SelectionVariant::Chip)
}

/// Create a selection style builder for switch components
#[must_use]
pub fn switch_style(tokens: &MaterialTokens) -> SelectionStyleBuilder {
    SelectionStyleBuilder::with_tokens(tokens, SelectionVariant::Switch)
}
