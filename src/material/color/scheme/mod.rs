//! Color schemes for Material Design 3
//!
//! This module provides color scheme definitions for Material Design 3,
//! including light and dark themes, as well as dynamic theming support.

mod light;
mod dark;
pub mod dynamic;

use crate::material::color::{Srgb, ColorRoles};
use self::dynamic::DynamicTheme;

/// Theme variant (light or dark)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Light theme variant
    Light,
    /// Dark theme variant
    Dark,
}

/// Theme data structure for Material Design 3
///
/// The `Theme` struct represents a complete Material Design 3 theme, including
/// color roles, variant (light/dark), and other theme properties. It provides
/// methods for creating, customizing, and managing themes.
///
/// # Examples
///
/// ```rust
/// use abop_iced::material::color::{Theme, ThemeVariant, Srgb};
///
/// // Create a light theme
/// let mut theme = Theme::light();
///
/// // Create a dark theme
/// let dark_theme = Theme::dark();
///
/// // Create a theme from a seed color
/// let custom_theme = Theme::from_seed(Srgb::new(0.5, 0.2, 0.8), ThemeVariant::Light);
///
/// // Toggle between light and dark
/// theme.toggle();
/// ```
#[derive(Debug, Clone)]
pub struct Theme {
    /// Color scheme variant (light/dark)
    ///
    /// Determines whether the theme uses light or dark color values.
    /// This affects all color roles and their relationships.
    pub variant: ThemeVariant,
    /// Core color roles
    ///
    /// Contains all the semantic color roles used throughout the application,
    /// including primary, secondary, tertiary, error, and surface colors.
    pub colors: ColorRoles,
    // ... other theme properties will be added in subsequent phases
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Theme {
    /// Create a new light theme
    pub fn light() -> Self {
        light::light_theme()
    }
    
    /// Create a new dark theme
    pub fn dark() -> Self {
        dark::dark_theme()
    }
    
    /// Create a new theme from a seed color
    ///
    /// # Arguments
    /// * `seed` - The seed color in sRGB color space
    /// * `variant` - The theme variant (light or dark)
    ///
    /// # Example
    /// ```rust
    /// use abop_iced::material::color::{Theme, ThemeVariant, Srgb};
    /// 
    /// let theme = Theme::from_seed(Srgb::new(0.5, 0.2, 0.8), ThemeVariant::Light);
    /// ```
    pub fn from_seed(seed: Srgb, variant: ThemeVariant) -> Self {
        DynamicTheme::new()
            .with_seed_color(seed)
            .with_variant(variant)
            .generate_theme()
    }
    
    /// Toggle between light and dark theme
    pub fn toggle(&mut self) {
        self.variant = match self.variant {
            ThemeVariant::Light => ThemeVariant::Dark,
            ThemeVariant::Dark => ThemeVariant::Light,
        };
        
        // Update colors based on the new variant
        let new_theme = match self.variant {
            ThemeVariant::Light => Self::light(),
            ThemeVariant::Dark => Self::dark(),
        };
        
        self.colors = new_theme.colors;
    }
    
    /// Check if the current theme is dark
    pub fn is_dark(&self) -> bool {
        matches!(self.variant, ThemeVariant::Dark)
    }
    
    /// Check if the current theme is light
    pub fn is_light(&self) -> bool {
        matches!(self.variant, ThemeVariant::Light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_toggle() {
        let mut theme = Theme::light();
        assert!(theme.is_light());
        
        theme.toggle();
        assert!(theme.is_dark());
        
        theme.toggle();
        assert!(theme.is_light());
    }
    
    #[test]
    fn test_default_theme() {
        let theme = Theme::default();
        assert!(theme.is_light());
    }
}
