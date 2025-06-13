//! Dynamic theme management for Material Design
//!
//! This module provides infrastructure for dynamic theme switching and
//! runtime theme updates. This is the foundation for Phase 3 implementation.

use iced::Color;

use crate::styling::material::tokens::core::MaterialTokens;
use crate::theme::ThemeMode;

/// Dynamic theme management system
///
/// This struct provides functionality for managing theme changes at runtime,
/// including system preference detection and smooth theme transitions.
#[derive(Debug, Clone)]
pub struct DynamicTheme {
    /// Current theme mode
    current_mode: ThemeMode,
    /// Current material tokens
    current_tokens: MaterialTokens,
    /// Whether to respect system preferences
    respect_system: bool,
}

impl Default for DynamicTheme {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicTheme {
    /// Create a new dynamic theme with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_mode: ThemeMode::default(),
            current_tokens: MaterialTokens::default(),
            respect_system: true,
        }
    }

    /// Create a dynamic theme with a specific mode
    #[must_use]
    pub fn with_mode(mode: ThemeMode) -> Self {
        let tokens = match mode {
            ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
            ThemeMode::Dark | ThemeMode::MaterialDark => MaterialTokens::dark(),
            ThemeMode::System => {
                // System detection - default to dark for now
                MaterialTokens::dark()
            }
            ThemeMode::MaterialDynamic => MaterialTokens::default(),
        };

        Self {
            current_mode: mode,
            current_tokens: tokens,
            respect_system: true,
        }
    }

    /// Get the current theme mode
    #[must_use]
    pub const fn current_mode(&self) -> ThemeMode {
        self.current_mode
    }

    /// Get the current material tokens
    #[must_use]
    pub const fn current_tokens(&self) -> &MaterialTokens {
        &self.current_tokens
    }

    /// Switch to a new theme mode
    pub fn switch_to_mode(&mut self, mode: ThemeMode) {
        self.current_mode = mode;
        self.current_tokens = match mode {
            ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
            ThemeMode::Dark | ThemeMode::MaterialDark => MaterialTokens::dark(),
            ThemeMode::System => {
                // System detection - default to dark for now
                MaterialTokens::dark()
            }
            ThemeMode::MaterialDynamic => MaterialTokens::default(),
        };
    }

    /// Update tokens with a seed color
    pub fn update_with_seed_color(&mut self, seed: Color) {
        let is_dark = self.current_tokens.is_dark_theme();
        self.current_tokens = MaterialTokens::from_seed_color(seed, is_dark);
    }

    /// Enable or disable system preference respect
    pub const fn set_respect_system(&mut self, respect: bool) {
        self.respect_system = respect;
    }

    /// Check if system preferences are respected
    #[must_use]
    pub const fn respects_system(&self) -> bool {
        self.respect_system
    }

    /// Check if the current theme is dark
    #[must_use]
    pub fn is_dark_theme(&self) -> bool {
        self.current_tokens.is_dark_theme()
    }

    /// Get a preview of what tokens would look like with a different mode
    #[must_use]
    pub fn preview_mode(&self, mode: ThemeMode) -> MaterialTokens {
        match mode {
            ThemeMode::Light | ThemeMode::MaterialLight => MaterialTokens::light(),
            ThemeMode::Dark | ThemeMode::MaterialDark => MaterialTokens::dark(),
            ThemeMode::System => {
                // System detection - default to dark for now
                MaterialTokens::dark()
            }
            ThemeMode::MaterialDynamic => MaterialTokens::default(),
        }
    }
}
