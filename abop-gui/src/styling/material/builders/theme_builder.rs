//! Theme builder pattern for Material Design
//!
//! This module provides a builder pattern for creating and managing themes
//! with advanced configuration options. This is the foundation for Phase 3.

use crate::styling::material::{
    builders::tokens_builder::MaterialTokensBuilder,
    tokens::core::MaterialTokens,
};
use crate::theme::ThemeMode;

/// Builder for creating customized Material Design themes
///
/// This builder provides a high-level API for theme creation and management.
/// It will be expanded in Phase 3 to support advanced theme features.
#[derive(Debug, Default)]
pub struct ThemeBuilder {
    /// Theme mode configuration
    mode: Option<ThemeMode>,
    /// Whether to respect system preferences
    respect_system: bool,
    /// Whether to support dynamic theming
    dynamic: bool,
}

impl ThemeBuilder {
    /// Create a new `ThemeBuilder`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the theme mode
    #[must_use]
    pub const fn with_mode(mut self, mode: ThemeMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Enable system preference respect
    #[must_use]
    pub const fn respect_system_preferences(mut self) -> Self {
        self.respect_system = true;
        self
    }

    /// Enable dynamic theming
    #[must_use]
    pub const fn with_dynamic_theming(mut self) -> Self {
        self.dynamic = true;
        self
    }

    /// Build a `MaterialTokensBuilder` with the configured theme options
    #[must_use]
    pub fn build_tokens_builder(self) -> MaterialTokensBuilder {
        let mut builder = MaterialTokensBuilder::new();

        if let Some(mode) = self.mode {
            builder = builder.with_theme_mode(mode);
        }

        // Phase 3 will implement full theme logic
        builder
    }

    /// Build `MaterialTokens` directly with the configured theme options
    #[must_use]
    pub fn build(self) -> MaterialTokens {
        self.build_tokens_builder().build()
    }
}
