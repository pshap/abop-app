//! Theme management for Material Design token system
//!
//! This module provides infrastructure for managing themes, including
//! theme modes, dynamic theming, and system integration.

pub mod dynamic;
pub mod theme_mode;

pub use dynamic::DynamicTheme;
pub use theme_mode::ThemeMode;
