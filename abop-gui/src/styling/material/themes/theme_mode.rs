//! Theme mode definitions for Material Design
//!
//! This module defines the different theme modes supported by the Material Design
//! token system, including automatic system preference detection.

/// Theme mode enumeration for Material Design tokens
///
/// This enum defines the different ways themes can be configured,
/// from simple light/dark modes to automatic system detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    /// Light theme mode
    ///
    /// Uses light color schemes with dark text on light backgrounds.
    /// Suitable for well-lit environments and accessibility needs.
    Light,

    /// Dark theme mode
    ///
    /// Uses dark color schemes with light text on dark backgrounds.
    /// Reduces eye strain in low-light environments and saves battery on OLED displays.
    Dark,

    /// Automatic theme mode
    ///
    /// Automatically switches between light and dark themes based on
    /// system preferences and time of day (when available).
    Auto,

    /// Custom theme mode
    ///
    /// Allows for completely custom theme definitions that don't follow
    /// standard light/dark patterns.
    Custom,
}

impl Default for ThemeMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl ThemeMode {
    /// Check if the theme mode is dark
    ///
    /// Returns true for Dark mode, false for Light mode.
    /// Auto and Custom modes require additional context to determine.
    #[must_use]
    pub const fn is_dark(&self) -> Option<bool> {
        match self {
            Self::Light => Some(false),
            Self::Dark => Some(true),
            Self::Auto | Self::Custom => None,
        }
    }

    /// Check if the theme mode is light
    ///
    /// Returns true for Light mode, false for Dark mode.
    /// Auto and Custom modes require additional context to determine.
    #[must_use]
    pub const fn is_light(&self) -> Option<bool> {
        match self {
            Self::Light => Some(true),
            Self::Dark => Some(false),
            Self::Auto | Self::Custom => None,
        }
    }

    /// Check if the theme mode requires system detection
    ///
    /// Returns true for Auto mode, which needs to detect system preferences.
    #[must_use]
    pub const fn requires_system_detection(&self) -> bool {
        matches!(self, Self::Auto)
    }

    /// Check if the theme mode allows customization
    ///
    /// Returns true for Custom mode, which supports arbitrary theme definitions.
    #[must_use]
    pub const fn allows_customization(&self) -> bool {
        matches!(self, Self::Custom)
    }
}

impl std::fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "Light"),
            Self::Dark => write!(f, "Dark"),
            Self::Auto => write!(f, "Auto"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}
