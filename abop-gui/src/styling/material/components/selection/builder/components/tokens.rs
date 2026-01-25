//! Static Material Design tokens for selection components
//!
//! This module provides static instances of MaterialTokens configured with
//! light and dark themes, used as fallbacks when no specific tokens
//! are provided for component styling.

use crate::styling::material::{MaterialColors, tokens::core::MaterialTokens};

/// Default light theme Material Design tokens for selection components.
///
/// This provides a static instance of MaterialTokens configured with light theme colors,
/// used as a fallback when no specific tokens are provided for component styling.
pub static LIGHT_TOKENS: std::sync::LazyLock<MaterialTokens> = std::sync::LazyLock::new(|| {
    MaterialTokens::default().with_colors(MaterialColors::light_default())
});

/// Default dark theme Material Design tokens for selection components.
///
/// This provides a static instance of MaterialTokens configured with dark theme colors,
/// used as a fallback when no specific tokens are provided for component styling.
pub static DARK_TOKENS: std::sync::LazyLock<MaterialTokens> = std::sync::LazyLock::new(|| {
    MaterialTokens::default().with_colors(MaterialColors::dark_default())
});
